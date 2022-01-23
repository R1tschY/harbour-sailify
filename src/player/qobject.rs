use std::env;
use std::sync::Arc;

use log::{error, info, warn};

use crate::player::error::LibrespotError;
use crate::player::qtgateway::{LibrespotEvent, LibrespotEventListener};
use crate::player::{LibrespotThread, Options};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlayerState {
    Stopped = 0,
    Playing = 1,
    Paused = 2,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MediaStatus {
    NoMedia = 0,
    Loading = 1,
    Loaded = 2,
    Buffering = 3,
    Stalled = 4,
    Buffered = 5,
    EndOfMedia = 6,
    InvalidMedia = 7,
    UnknownStatus = 8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectionStatus {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,

    Crashed = 100,
}

pub struct SailifyPlayer {
    thread: Option<LibrespotThread>,
    options: Options,
    listener: Arc<dyn LibrespotEventListener>,
}

fn setup_logging() {
    let rust_log = env::var("RUST_LOG")
        .unwrap_or_else(|_| "libmdns=info,librespot=info,sailify=debug".to_string());

    env_logger::Builder::new().parse_filters(&rust_log).init();
}

impl SailifyPlayer {
    #[must_use]
    pub fn new(listener: Arc<dyn LibrespotEventListener>) -> Self {
        setup_logging();
        Self {
            thread: None,
            options: Options::default(),
            listener,
        }
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        self.thread.is_some()
    }

    pub fn start(&mut self) -> bool {
        if self.is_running() {
            warn!("Already started player");
            return true;
        }

        info!("Starting player ...");

        match LibrespotThread::run(self.listener.clone(), self.options.clone()) {
            Ok(thread) => {
                self.thread = Some(thread);
                true
            }
            Err(err) => {
                self.set_error(err);
                false
            }
        }
    }

    fn set_error(&mut self, err: LibrespotError) {
        error!("Librespot error: {}", err);
        self.listener.notify(LibrespotEvent::Error { err });
    }

    pub fn stop(&mut self) {
        if !self.is_running() {
            return;
        }

        info!("Shutting down ...");

        self.shutdown_thread();
    }

    pub fn logout(&mut self) {
        info!("Logging out ...");

        self.stop();
        LibrespotThread::remove_credentials(&self.options);
    }

    pub fn play(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.play();
        }
    }

    pub fn pause(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.pause();
        }
    }

    pub fn next(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.next();
        }
    }

    pub fn previous(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.previous();
        }
    }

    fn shutdown_thread(&mut self) {
        if let Some(thread) = std::mem::replace(&mut self.thread, None) {
            thread.shutdown();
        }
    }

    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.options.username.as_ref().map(|s| s as &str)
    }

    pub fn set_username(&mut self, value: Option<&str>) {
        self.options.username = value.map(ToString::to_string);
    }

    #[must_use]
    pub fn password(&self) -> Option<&str> {
        self.options.password.as_ref().map(|s| s as &str)
    }

    pub fn set_password(&mut self, value: Option<&str>) {
        self.options.password = value.map(ToString::to_string);
    }

    #[must_use]
    pub fn is_active(&self) -> bool {
        self.thread.is_some()
    }

    pub fn refresh_access_token(&self) {
        if let Some(ref thread) = &self.thread {
            thread.refresh_token();
        }
    }

    #[must_use]
    pub fn device_id(&self) -> &str {
        &self.options.device_id
    }

    #[must_use]
    pub fn device_name(&self) -> &str {
        &self.options.device_name
    }
}

impl Drop for SailifyPlayer {
    fn drop(&mut self) {
        self.shutdown_thread();
    }
}

// #[repr(transparent)]
// pub struct SailifyPlayer {
//     thread: Option<LibrespotThread>,
//     options: Options,
//
//     access_token: Option<(String, Instant)>,
//
//     error: Option<LibrespotError>,
//
//     media_status: MediaStatus,
//     connection: ConnectionStatus,
//     track: Option<String>,
//     state: PlayerState,
//     position_ms: u32,
//     position_instant: Instant,
//     duration_ms: u32,
//
//     listener: Box<dyn LibrespotEventListener>,
// }
//
// impl SailifyPlayer {
//     pub fn new(listener: Box<dyn LibrespotEventListener>) -> Self {
//         Self {
//             thread: None,
//             options: Options::new(),
//
//             access_token: None,
//
//             error: None,
//             media_status: MediaStatus::NoMedia,
//             connection: ConnectionStatus::Disconnected,
//             track: None,
//             state: PlayerState::Stopped,
//             position_ms: 0,
//             duration_ms: 0,
//             position_instant: Instant::now(),
//
//             listener,
//         }
//     }
//
//     // #[slot]
//     pub fn _on_player_event(&mut self) {
//  /*       let evt = match self.qt_rx.try_recv() {
//             Ok(evt) => evt,
//             Err(TryRecvError::Empty) => {
//                 return warn!("Empty queue but expected event");
//             }
//             Err(TryRecvError::Disconnected) => {
//                 return warn!("Queue disconnected");
//             }
//         };
//
//         info!("GOT event: {:?}", evt);
//         match evt {
//             LibrespotEvent::Stopped { .. } => {
//                 self.set_media_status(MediaStatus::NoMedia);
//                 self.set_track_uri(None);
//                 self.set_position(0, PlayerState::Stopped);
//             }
//             LibrespotEvent::Changed { new_track_id } => {
//                 self.set_track_uri(Some(new_track_id));
//             }
//             LibrespotEvent::Loading {
//                 track_id,
//                 position_ms,
//                 ..
//             } => {
//                 self.set_track_uri(Some(track_id));
//                 self.set_media_status(MediaStatus::Loading);
//                 self.set_position(position_ms, PlayerState::Stopped);
//             }
//             LibrespotEvent::Playing {
//                 track_id,
//                 position_ms,
//                 duration_ms,
//                 ..
//             } => {
//                 self.set_track_uri(Some(track_id));
//                 self.set_media_status(MediaStatus::Loaded);
//                 self.set_position(position_ms, PlayerState::Playing);
//                 self.set_duration(duration_ms);
//                 self.timer.start();
//             }
//             LibrespotEvent::Paused {
//                 track_id,
//                 position_ms,
//                 duration_ms,
//                 ..
//             } => {
//                 self.set_track_uri(Some(track_id));
//                 self.set_position(position_ms, PlayerState::Paused);
//                 self.set_duration(duration_ms);
//                 self.timer.stop();
//             }
//             LibrespotEvent::Unavailable { track_id, .. } => {
//                 self.set_track_uri(Some(track_id));
//                 self.set_media_status(MediaStatus::InvalidMedia);
//                 self.set_position(0, PlayerState::Stopped);
//                 self.set_duration(0);
//                 self.timer.stop();
//             }
//             LibrespotEvent::VolumeSet { .. } => {}
//             LibrespotEvent::Connecting => self.set_connection_status(ConnectionStatus::Connecting),
//             LibrespotEvent::Connected => {
//                 self.set_connection_status(ConnectionStatus::Connected);
//             }
//             LibrespotEvent::ConnectionError { message } => {
//                 self.set_error(LibrespotError::Connection(message));
//                 self.shutdown();
//                 self.set_position(0, PlayerState::Stopped);
//             }
//             LibrespotEvent::Shutdown => {
//                 self.set_connection_status(ConnectionStatus::Disconnected);
//                 self.set_media_status(MediaStatus::NoMedia);
//                 self.set_position(0, PlayerState::Stopped);
//                 self.thread = None;
//             }
//             LibrespotEvent::StartReconnect => {
//                 self.set_connection_status(ConnectionStatus::Connecting);
//                 self.set_position(0, PlayerState::Stopped);
//             }
//             LibrespotEvent::TokenChanged { token } => self.set_access_token(token),
//             LibrespotEvent::Panic { message } => {
//                 self.set_error(LibrespotError::Panic(message));
//                 self.set_connection_status(ConnectionStatus::Crashed);
//                 self.set_media_status(MediaStatus::NoMedia);
//                 self.set_position(0, PlayerState::Stopped);
//                 self.thread = None;
//             }
//         }*/
//     }
//
//     // status
//
//     pub fn media_status(&self) -> i32 {
//         self.media_status as i32
//     }
//
//     pub fn set_media_status(&mut self, status: MediaStatus) {
//         if self.media_status != status {
//             self.media_status = status;
//
//             unsafe { &mut *self.qobject }.media_status_changed(self.media_status as i32);
//         }
//     }
//
//     // connection status
//
//     pub fn connection_status(&self) -> i32 {
//         self.connection as i32
//     }
//
//     pub fn set_connection_status(&mut self, status: ConnectionStatus) {
//         if self.connection != status {
//             self.connection = status;
//
//             unsafe { &mut *self.qobject }.connection_status_changed(self.connection as i32);
//         }
//     }
//
//     // track uri
//
//     pub fn track_uri(&self) -> QString {
//         self.track.to_qstring()
//     }
//
//     pub fn set_track_uri(&mut self, uri: Option<String>) {
//         if self.track != uri {
//             self.track = uri;
//
//             unsafe { &mut *self.qobject }.track_uri_changed(&self.track.to_qstring());
//         }
//     }
//
//     // paused
//
//     pub fn playback_status(&self) -> QString {
//         match self.state {
//             PlayerState::Stopped => "stopped",
//             PlayerState::Playing => "playing",
//             PlayerState::Paused => "paused",
//         }
//         .to_qstring()
//     }
//
//     // position
//
//     pub fn position(&self) -> u32 {
//         if self.state == PlayerState::Playing {
//             let update = Instant::now()
//                 .duration_since(self.position_instant)
//                 .as_millis() as u32;
//             self.position_ms + update
//         } else {
//             self.position_ms
//         }
//     }
//
//     pub fn set_position(&mut self, value: u32, state: PlayerState) {
//         self.position_instant = Instant::now();
//
//         if self.state != state {
//             self.state = state;
//             unsafe { &mut *self.qobject }.playback_status_changed();
//         }
//
//         if self.position_ms != value {
//             self.position_ms = value;
//             unsafe { &mut *self.qobject }.position_changed(value);
//         }
//
//         if self.state == PlayerState::Playing {
//             self.timer.start();
//         } else {
//             self.timer.stop();
//         }
//     }
//
//     // duration
//
//     pub fn duration(&self) -> u32 {
//         self.duration_ms
//     }
//
//     pub fn set_duration(&mut self, value: u32) {
//         if self.duration_ms != value {
//             self.duration_ms = value;
//
//             unsafe { &mut *self.qobject }.duration_changed(value);
//         }
//     }
//
//
//     // private
//
//     pub fn update_position(&self) {
//         if self.state == PlayerState::Playing {
//             unsafe { &mut *self.qobject }.position_changed(self.position());
//         }
//     }
// }
//
// impl Drop for SailifyPlayer {
//     fn drop(&mut self) {
//         self.shutdown_thread()
//     }
// }
