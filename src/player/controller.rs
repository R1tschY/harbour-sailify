use std::time::Instant;

use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::compat::Future01CompatExt;
use futures::StreamExt;
use futures_01::future::Future as Future01;
use futures_01::stream::Stream as Stream01;
use librespot_connect::spirc::Spirc;
use librespot_core::authentication::Credentials;
use librespot_core::cache::Cache;
use librespot_core::config::{ConnectConfig, SessionConfig};
use librespot_core::keymaster::get_token;
use librespot_core::session::Session;
use librespot_playback::audio_backend::Sink;
use librespot_playback::config::PlayerConfig;
use librespot_playback::mixer::{Mixer, MixerConfig};
use librespot_playback::player::Player;
use log::{error, info, warn};
use tokio_core::reactor::Handle;

use crate::player::qtgateway::{LibrespotEvent, LibrespotGateway};
use crate::player::{CLIENT_ID, SCOPES};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlMessage {
    Shutdown,
    Play,
    Pause,
    Next,
    Previous,

    RefreshToken,

    // internal
    AutoReconnect,
}

#[derive(Clone)]
pub struct LibrespotConfig {
    pub backend: fn(Option<String>) -> Box<dyn Sink>,
    pub device: Option<String>,

    pub mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,

    pub cache: Cache,
    pub player_config: PlayerConfig,
    pub session_config: SessionConfig,
    pub connect_config: ConnectConfig,
    pub mixer_config: MixerConfig,
    pub credentials: Credentials,
}

pub struct LibrespotController {
    cache: Cache,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    backend: fn(Option<String>) -> Box<dyn Sink>,
    device: Option<String>,
    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,
    mixer_config: MixerConfig,
    handle: Handle,

    control_rx: UnboundedReceiver<ControlMessage>,
    control_tx: UnboundedSender<ControlMessage>,

    spirc: Option<Spirc>,
    session: Option<Session>,

    credentials: Credentials,
    auto_connect_times: Vec<Instant>,

    gateway: LibrespotGateway,
}

impl LibrespotController {
    pub async fn run(
        handle: Handle,
        control_tx: UnboundedSender<ControlMessage>,
        control_rx: UnboundedReceiver<ControlMessage>,
        gateway: LibrespotGateway,
        setup: LibrespotConfig,
    ) {
        let self_ = LibrespotController {
            handle: handle.clone(),
            cache: setup.cache,
            session_config: setup.session_config,
            player_config: setup.player_config,
            connect_config: setup.connect_config,
            backend: setup.backend,
            device: setup.device,
            mixer: setup.mixer,
            mixer_config: setup.mixer_config,

            spirc: None,
            session: None,

            credentials: setup.credentials,
            auto_connect_times: Vec::new(),
            control_rx,
            control_tx,

            gateway,
        };
        self_.run_internal().await;
    }

    pub async fn run_internal(mut self) {
        if !self.login().await {
            return;
        }

        while let Some(msg) = self.control_rx.next().await {
            if let Some(ref spirc) = self.spirc {
                match msg {
                    ControlMessage::Play => spirc.play(),
                    ControlMessage::Next => spirc.next(),
                    ControlMessage::Pause => spirc.pause(),
                    ControlMessage::Previous => spirc.prev(),
                    ControlMessage::Shutdown => {
                        self.shutdown();
                        return;
                    }
                    ControlMessage::AutoReconnect => {
                        if !self.autoreconnect().await {
                            return;
                        }
                    }
                    ControlMessage::RefreshToken => {
                        if let Some(session) = &self.session {
                            let gateway = self.gateway.clone();
                            self.handle
                                .spawn(get_token(&session, CLIENT_ID, SCOPES).then(
                                    move |result| {
                                        let evt_data = result.map_err(|err| format!("{:?}", err));
                                        gateway
                                            .send(LibrespotEvent::TokenChanged { token: evt_data });
                                        futures_01::future::ok::<(), ()>(())
                                    },
                                ));
                        }
                    }
                };
            }
        }
    }

    async fn login(&mut self) -> bool {
        info!("Logging in ...");
        self.spirc = None;
        self.gateway.send(LibrespotEvent::Connecting);

        // connect with credentials
        let session_future = Session::connect(
            self.session_config.clone(),
            self.credentials.clone(),
            Some(self.cache.clone()),
            self.handle.clone(),
        );
        let session = match session_future.compat().await {
            Ok(session) => session,
            Err(error) => {
                error!("Could not connect to server: {}", error);
                self.gateway.send(LibrespotEvent::ConnectionError {
                    message: format!("{}", error),
                });
                return false;
            }
        };
        self.session = Some(session.clone());
        info!("Connected");

        // setup
        let mixer_config = self.mixer_config.clone();
        let mixer = (self.mixer)(Some(mixer_config));
        let player_config = self.player_config.clone();
        let connect_config = self.connect_config.clone();

        let audio_filter = mixer.get_audio_filter();
        let backend = self.backend;
        let device = self.device.clone();
        let (player, event_channel) =
            Player::new(player_config, session.clone(), audio_filter, move || {
                (backend)(device)
            });

        let (spirc, spirc_task) = Spirc::new(connect_config, session.clone(), player, mixer);
        self.spirc = Some(spirc);

        let control_tx = self.control_tx.clone();
        self.handle.spawn(spirc_task.map(move |()| {
            let _ = control_tx.unbounded_send(ControlMessage::AutoReconnect);
        }));

        let gateway = self.gateway.clone();
        self.handle.spawn(
            event_channel
                .for_each(move |event| {
                    if let Some(evt) = LibrespotEvent::from_event(event) {
                        gateway.send(evt);
                    }
                    futures_01::future::ok(())
                })
                .map_err(|_| ()),
        );

        // get token
        let token = get_token(&session, CLIENT_ID, SCOPES)
            .compat()
            .await
            .map_err(|err| format!("{:?}", err));
        self.gateway.send(LibrespotEvent::TokenChanged { token });
        self.gateway.send(LibrespotEvent::Connected);

        true
    }

    fn shutdown(&mut self) {
        self.gateway.send(LibrespotEvent::Shutdown);
        if let Some(ref spirc) = self.spirc {
            spirc.shutdown();
        }
    }

    async fn autoreconnect(&mut self) -> bool {
        warn!("Spirc shut down unexpectedly");
        self.gateway.send(LibrespotEvent::StartReconnect);

        let now = Instant::now();
        while (!self.auto_connect_times.is_empty())
            && ((now - self.auto_connect_times[0]).as_secs() > 600)
        {
            let _ = self.auto_connect_times.remove(0);
        }

        if self.auto_connect_times.len() >= 5 {
            warn!("Spirc shut down too often. Not reconnecting automatically.");
            self.gateway.send(LibrespotEvent::ConnectionError {
                message: "Spirc shut down too often. Not reconnecting automatically.".to_string(),
            });
            false
        } else {
            self.auto_connect_times.push(now);
            self.login().await
        }
    }
}
