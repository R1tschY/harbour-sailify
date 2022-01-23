use std::sync::Arc;
use std::time::Instant;

use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::StreamExt;
use librespot_connect::spirc::Spirc;
use librespot_core::authentication::Credentials;
use librespot_core::cache::Cache;
use librespot_core::config::{ConnectConfig, SessionConfig};
use librespot_core::keymaster::get_token;
use librespot_core::session::Session;
use librespot_playback::audio_backend::SinkBuilder;
use librespot_playback::config::{AudioFormat, PlayerConfig};
use librespot_playback::mixer::{MixerConfig, MixerFn};
use librespot_playback::player::{Player, PlayerEventChannel};
use log::{error, info, warn};
use tokio::runtime::Handle;

use crate::player::qtgateway::{LibrespotEvent, LibrespotEventListener, LibrespotEventListenerRef};
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
    pub format: AudioFormat,
    pub backend: SinkBuilder,
    pub device: Option<String>,

    pub mixer: MixerFn,

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
    format: AudioFormat,
    backend: SinkBuilder,
    device: Option<String>,
    mixer: MixerFn,
    mixer_config: MixerConfig,
    handle: Handle,

    control_rx: UnboundedReceiver<ControlMessage>,
    control_tx: UnboundedSender<ControlMessage>,

    spirc: Option<Spirc>,
    session: Option<Session>,

    credentials: Credentials,
    auto_connect_times: Vec<Instant>,

    listener: Arc<dyn LibrespotEventListener>,
}

impl LibrespotController {
    pub async fn run(
        handle: Handle,
        control_tx: UnboundedSender<ControlMessage>,
        control_rx: UnboundedReceiver<ControlMessage>,
        listener: Arc<dyn LibrespotEventListener>,
        setup: LibrespotConfig,
    ) {
        let self_ = LibrespotController {
            handle: handle.clone(),
            cache: setup.cache,
            session_config: setup.session_config,
            player_config: setup.player_config,
            connect_config: setup.connect_config,
            format: setup.format,
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

            listener,
        };
        self_.run_internal().await;
    }

    async fn get_token(session: Session, listener: LibrespotEventListenerRef) {
        let token_result = get_token(&session, CLIENT_ID, SCOPES).await;
        listener.notify(LibrespotEvent::TokenChanged {
            token: token_result.map_err(|err| format!("{:?}", err)),
        });
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
                            self.handle
                                .spawn(Self::get_token(session.clone(), self.listener.clone()));
                        }
                    }
                };
            }
        }
    }

    async fn login(&mut self) -> bool {
        info!("Logging in ...");
        self.spirc = None;
        self.listener.notify(LibrespotEvent::Connecting);

        // connect with credentials
        let session_future = Session::connect(
            self.session_config.clone(),
            self.credentials.clone(),
            Some(self.cache.clone()),
        );
        let session = match session_future.await {
            Ok(session) => session,
            Err(error) => {
                error!("Could not connect to server: {:?}", error);
                self.listener.notify(LibrespotEvent::ConnectionError {
                    message: format!("{:?}", error),
                });
                return false;
            }
        };
        self.session = Some(session.clone());
        info!("Connected");

        // setup
        let mixer_config = self.mixer_config.clone();
        let mixer = (self.mixer)(mixer_config);
        let player_config = self.player_config.clone();
        let connect_config = self.connect_config.clone();

        let audio_filter = mixer.get_audio_filter();
        let format = self.format;
        let backend = self.backend;
        let device = self.device.clone();
        let (player, event_channel) =
            Player::new(player_config, session.clone(), audio_filter, move || {
                (backend)(device, format)
            });

        let (spirc, spirc_task) = Spirc::new(connect_config, session.clone(), player, mixer);
        self.spirc = Some(spirc);

        let control_tx = self.control_tx.clone();
        self.handle.spawn(async move {
            spirc_task.await;
            let _ = control_tx.unbounded_send(ControlMessage::AutoReconnect);
        });

        self.handle.spawn(Self::run_event_channel(
            event_channel,
            self.listener.clone(),
        ));

        // get token
        let token = get_token(&session, CLIENT_ID, SCOPES)
            .await
            .map_err(|err| format!("{:?}", err));
        self.listener.notify(LibrespotEvent::TokenChanged { token });
        self.listener.notify(LibrespotEvent::Connected);

        true
    }

    fn shutdown(&mut self) {
        self.listener.notify(LibrespotEvent::Shutdown);
        if let Some(ref spirc) = self.spirc {
            spirc.shutdown();
        }
    }

    async fn run_event_channel(
        mut event_channel: PlayerEventChannel,
        listener: LibrespotEventListenerRef,
    ) {
        while let Some(event) = event_channel.recv().await {
            if let Some(evt) = LibrespotEvent::from_event(event) {
                listener.notify(evt);
            }
        }
    }

    async fn autoreconnect(&mut self) -> bool {
        warn!("Spirc shut down unexpectedly");
        self.listener.notify(LibrespotEvent::StartReconnect);

        let now = Instant::now();
        while (!self.auto_connect_times.is_empty())
            && ((now - self.auto_connect_times[0]).as_secs() > 600)
        {
            let _ = self.auto_connect_times.remove(0);
        }

        if self.auto_connect_times.len() >= 5 {
            warn!("Spirc shut down too often. Not reconnecting automatically.");
            self.listener.notify(LibrespotEvent::ConnectionError {
                message: "Spirc shut down too often. Not reconnecting automatically.".to_string(),
            });
            false
        } else {
            self.auto_connect_times.push(now);
            self.login().await
        }
    }
}
