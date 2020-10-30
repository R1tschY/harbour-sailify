use std::env;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{Async, Future, Poll, Stream};
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::session::Session;
use librespot::core::version;
use librespot::playback::audio_backend::{self, Sink};
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::playback::mixer::{self, Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};
use log::{error, info, warn};
use qt5qml::core::{QByteArray, QString};
use qt5qml::QBox;
use sha1::{Digest, Sha1};
use tokio_core::reactor::{Core, Handle};
use url::Url;

use crate::player::qtgateway::{
    serialize_event, LibrespotEvent, LibrespotGateway, LibrespotGatewayPrivate,
};
use crate::player::Options;

#[derive(Clone, Debug)]
pub enum ControlMessage {
    Shutdown,
}

#[derive(Clone)]
pub struct LibrespotConfig {
    pub backend: fn(Option<String>) -> Box<dyn Sink>,
    pub device: Option<String>,

    pub mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,

    pub cache: Option<Cache>,
    pub player_config: PlayerConfig,
    pub session_config: SessionConfig,
    pub connect_config: ConnectConfig,
    pub mixer_config: MixerConfig,
    pub credentials: Option<Credentials>,
}

pub struct Controller {
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    backend: fn(Option<String>) -> Box<dyn Sink>,
    device: Option<String>,
    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,
    mixer_config: MixerConfig,
    handle: Handle,

    control_rx: UnboundedReceiver<ControlMessage>,

    spirc: Option<Spirc>,
    spirc_task: Option<SpircTask>,
    connect: Box<dyn Future<Item = Session, Error = io::Error>>,

    shutdown: bool,
    last_credentials: Option<Credentials>,
    auto_connect_times: Vec<Instant>,

    player_event_channel: Option<UnboundedReceiver<PlayerEvent>>,
    gateway: QBox<LibrespotGateway>,
}

impl Controller {
    pub fn new(
        handle: Handle,
        control_rx: UnboundedReceiver<ControlMessage>,
        gateway: QBox<LibrespotGateway>,
        setup: LibrespotConfig,
    ) -> Controller {
        let mut task = Controller {
            handle: handle.clone(),
            cache: setup.cache,
            session_config: setup.session_config,
            player_config: setup.player_config,
            connect_config: setup.connect_config,
            backend: setup.backend,
            device: setup.device,
            mixer: setup.mixer,
            mixer_config: setup.mixer_config,

            connect: Box::new(futures::future::empty()),
            spirc: None,
            spirc_task: None,
            shutdown: false,
            last_credentials: None,
            auto_connect_times: Vec::new(),
            control_rx,

            player_event_channel: None,
            gateway,
        };

        if let Some(credentials) = setup.credentials {
            task.credentials(credentials);
        }

        task
    }

    fn credentials(&mut self, credentials: Credentials) {
        self.last_credentials = Some(credentials.clone());
        let config = self.session_config.clone();
        let handle = self.handle.clone();

        let connection = Session::connect(config, credentials, self.cache.clone(), handle);

        self.connect = connection;
        self.spirc = None;
        let task = mem::replace(&mut self.spirc_task, None);
        if let Some(task) = task {
            self.handle.spawn(task);
        }
        self.send_event(LibrespotEvent::Connecting);
    }

    fn send_event(&mut self, evt: LibrespotEvent) {
        Self::send_gateway_event(&mut self.gateway, evt)
    }

    fn send_gateway_event(gateway: &mut LibrespotGateway, evt: LibrespotEvent) {
        unsafe {
            gateway.playerEvent(&QByteArray::from_bytes(&serialize_event(evt)));
        }
    }
}

impl Future for Controller {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

            match self.connect.poll() {
                Ok(Async::NotReady) => (),
                Ok(Async::Ready(session)) => {
                    self.connect = Box::new(futures::future::empty());
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

                    let (spirc, spirc_task) = Spirc::new(connect_config, session, player, mixer);
                    self.spirc = Some(spirc);
                    self.spirc_task = Some(spirc_task);
                    self.player_event_channel = Some(event_channel);
                    self.send_event(LibrespotEvent::Connected {});

                    progress = true;
                }
                Err(error) => {
                    error!("Could not connect to server: {}", error);
                    self.connect = Box::new(futures::future::empty());
                    self.send_event(LibrespotEvent::ConnectionError {
                        message: format!("{}", error),
                    });
                }
            }

            if let Async::Ready(Some(msg)) = self.control_rx.poll().unwrap() {
                match msg {
                    ControlMessage::Shutdown => {
                        if !self.shutdown {
                            if let Some(ref spirc) = self.spirc {
                                Self::send_gateway_event(
                                    &mut self.gateway,
                                    LibrespotEvent::Shutdown,
                                );
                                spirc.shutdown();
                            } else {
                                return Ok(Async::Ready(()));
                            }
                            self.shutdown = true;
                        } else {
                            return Ok(Async::Ready(()));
                        }
                    }
                };

                progress = true;
            }

            let mut drop_spirc_and_try_to_reconnect = false;
            if let Some(ref mut spirc_task) = self.spirc_task {
                if let Async::Ready(()) = spirc_task.poll().unwrap() {
                    if self.shutdown {
                        return Ok(Async::Ready(()));
                    } else {
                        warn!("Spirc shut down unexpectedly");
                        drop_spirc_and_try_to_reconnect = true;
                        self.send_event(LibrespotEvent::StartReconnect);
                    }
                    progress = true;
                }
            }
            if drop_spirc_and_try_to_reconnect {
                self.spirc_task = None;
                let now = Instant::now();
                while (!self.auto_connect_times.is_empty())
                    && ((now - self.auto_connect_times[0]).as_secs() > 600)
                {
                    let _ = self.auto_connect_times.remove(0);
                }

                if let Some(credentials) = self.last_credentials.clone() {
                    if self.auto_connect_times.len() >= 5 {
                        warn!("Spirc shut down too often. Not reconnecting automatically.");
                        self.send_event(LibrespotEvent::ConnectionError {
                            message: "Spirc shut down too often. Not reconnecting automatically."
                                .to_string(),
                        });
                    } else {
                        self.auto_connect_times.push(now);
                        self.credentials(credentials);
                    }
                }
            }

            if let Some(ref mut player_event_channel) = self.player_event_channel {
                if let Async::Ready(Some(event)) = player_event_channel.poll().unwrap() {
                    if let Some(evt) = LibrespotEvent::from_event(event) {
                        self.send_event(evt);
                    }
                    progress = true;
                }
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}