use std::io;
use std::mem;
use std::time::Instant;

use futures::sync::mpsc::UnboundedReceiver;
use futures::{Async, Future, Poll, Stream};
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, SessionConfig};
use librespot::core::session::Session;
use librespot::playback::audio_backend::Sink;
use librespot::playback::config::PlayerConfig;
use librespot::playback::mixer::{Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};
use log::{error, warn};
use qt5qml::core::QByteArray;
use qt5qml::QBox;
use tokio_core::reactor::Handle;

use crate::player::qtgateway::{LibrespotEvent, LibrespotGateway};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControlMessage {
    Shutdown,
    Play,
    Pause,
    Next,
    Previous,
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
    pub credentials: Option<Credentials>,
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

    spirc: Option<Spirc>,
    spirc_task: Option<SpircTask>,
    connect: Box<dyn Future<Item = Session, Error = io::Error>>,

    shutdown: bool,
    last_credentials: Option<Credentials>,
    auto_connect_times: Vec<Instant>,

    player_event_channel: Option<UnboundedReceiver<PlayerEvent>>,
    gateway: LibrespotGateway,
}

impl LibrespotController {
    pub fn new(
        handle: Handle,
        control_rx: UnboundedReceiver<ControlMessage>,
        gateway: LibrespotGateway,
        setup: LibrespotConfig,
    ) -> LibrespotController {
        let mut task = LibrespotController {
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

        let connection = Session::connect(config, credentials, Some(self.cache.clone()), handle);

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
        gateway.send(evt);
    }
}

impl Future for LibrespotController {
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
                if msg == ControlMessage::Shutdown {
                    if !self.shutdown {
                        if let Some(ref spirc) = self.spirc {
                            Self::send_gateway_event(&mut self.gateway, LibrespotEvent::Shutdown);
                            spirc.shutdown();
                        } else {
                            return Ok(Async::Ready(()));
                        }
                        self.shutdown = true;
                    } else {
                        return Ok(Async::Ready(()));
                    }
                } else if !self.shutdown {
                    if let Some(ref spirc) = self.spirc {
                        match msg {
                            ControlMessage::Play => spirc.play(),
                            ControlMessage::Next => spirc.next(),
                            ControlMessage::Pause => spirc.pause(),
                            ControlMessage::Previous => spirc.prev(),
                            ControlMessage::Shutdown => unreachable!(),
                        };
                    }
                }

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
