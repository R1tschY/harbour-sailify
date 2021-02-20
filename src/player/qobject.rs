use std::sync::mpsc;
use std::sync::mpsc::{channel, TryRecvError};

use librespot_core::keymaster::Token;
use log::{error, info, warn};
use qt5qml::core::ToQString;
use qt5qml::core::{QObjectRef, QString};
use qt5qml::cstr;

use crate::player::error::LibrespotError;
use crate::player::qtgateway::{LibrespotEvent, LibrespotGateway};
use crate::player::{LibrespotThread, Options};
use crate::utils::from_qstring;

include!(concat!(env!("OUT_DIR"), "/qffi_Librespot.rs"));

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlayerStatus {
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
}

pub struct LibrespotPrivate {
    qobject: *mut Librespot,
    qt_tx: mpsc::Sender<LibrespotEvent>,
    qt_rx: mpsc::Receiver<LibrespotEvent>,
    thread: Option<LibrespotThread>,
    options: Options,

    token: Option<String>,

    error_kind: Option<String>,
    error_string: Option<String>,

    status: PlayerStatus,
    connection: ConnectionStatus,
    track: Option<String>,
    paused: bool,
    position_ms: u32,
    duration_ms: u32,
}

pub fn register_librespot() {
    Librespot::register_type(cstr!("Sailify"), 0, 1, cstr!("Librespot"));
}

impl LibrespotPrivate {
    pub fn new(qobject: *mut Librespot) -> Self {
        let (qt_tx, qt_rx) = channel();
        Self {
            qobject,
            qt_tx,
            qt_rx,
            thread: None,
            options: Options::new(),

            token: None,

            error_kind: None,
            error_string: None,
            status: PlayerStatus::NoMedia,
            connection: ConnectionStatus::Disconnected,
            track: None,
            paused: false,
            position_ms: 0,
            duration_ms: 0,
        }
    }

    pub fn username(&self) -> QString {
        self.options.username.to_qstring()
    }

    pub fn set_username(&mut self, value: &QString) {
        self.options.username = from_qstring(value);
    }

    // #[property(write = set_password, notify = password_changed)]
    pub fn password(&self) -> QString {
        self.options.password.to_qstring()
    }

    pub fn set_password(&mut self, value: &QString) {
        self.options.password = from_qstring(value);
    }

    // #[slot]
    pub fn on_player_event(&mut self) {
        let evt = match self.qt_rx.try_recv() {
            Ok(evt) => evt,
            Err(TryRecvError::Empty) => {
                return warn!("Empty queue but expected event");
            }
            Err(TryRecvError::Disconnected) => {
                return warn!("Queue disconnected");
            }
        };

        info!("GOT event: {:?}", evt);

        match evt {
            LibrespotEvent::Stopped { .. } => {
                self.set_status(PlayerStatus::NoMedia);
                self.set_track_uri(None);
            }
            LibrespotEvent::Changed { new_track_id } => {
                self.set_track_uri(Some(new_track_id));
            }
            LibrespotEvent::Loading {
                track_id,
                position_ms,
                ..
            } => {
                self.set_track_uri(Some(track_id));
                self.set_status(PlayerStatus::Loading);
                self.set_position(position_ms);
            }
            LibrespotEvent::Playing {
                track_id,
                position_ms,
                duration_ms,
                ..
            } => {
                self.set_track_uri(Some(track_id));
                self.set_status(PlayerStatus::Loaded);
                self.set_position(position_ms);
                self.set_duration(duration_ms);
                self.set_paused(false);
            }
            LibrespotEvent::Paused {
                track_id,
                position_ms,
                duration_ms,
                ..
            } => {
                self.set_track_uri(Some(track_id));
                self.set_position(position_ms);
                self.set_duration(duration_ms);
                self.set_paused(true);
            }
            LibrespotEvent::Unavailable { track_id, .. } => {
                self.set_track_uri(Some(track_id));
                self.set_status(PlayerStatus::InvalidMedia);
                self.set_position(0);
                self.set_duration(0);
            }
            LibrespotEvent::VolumeSet { .. } => {}
            LibrespotEvent::Connecting => self.set_connection_status(ConnectionStatus::Connecting),
            LibrespotEvent::Connected => {
                self.set_connection_status(ConnectionStatus::Connected);
            }
            LibrespotEvent::ConnectionError { message } => {
                self.set_error(LibrespotError::Connection(message));
                self.shutdown();
            }
            LibrespotEvent::Shutdown => {
                self.set_connection_status(ConnectionStatus::Disconnected);
                self.set_status(PlayerStatus::NoMedia);
                self.thread = None;
            }
            LibrespotEvent::StartReconnect => {
                self.set_connection_status(ConnectionStatus::Connecting);
            }
            LibrespotEvent::TokenChanged { token } => self.set_token(token),
        }
    }

    // #[slot]
    pub fn login(&mut self) {
        if self.is_active() {
            warn!("Already logged in");
            return;
        }

        info!("Logging in ...");

        let gateway: LibrespotGateway = LibrespotGateway::new(
            unsafe { &mut *self.qobject }.as_qobject(),
            self.qt_tx.clone(),
        );

        match LibrespotThread::run(gateway, self.options.clone()) {
            Ok(thread) => {
                self.thread = Some(thread);
                unsafe { &mut *self.qobject }.activeChanged(true);
            }
            Err(err) => self.set_error(err),
        }
    }

    // #[slot]
    pub fn logout(&mut self) {
        if !self.is_active() {
            warn!("Logout not possible: librespot is shutdown");
            return;
        }

        info!("Logging out ...");

        self.shutdown();
        LibrespotThread::remove_credentials(&self.options);
    }

    // #[slot]
    pub fn shutdown(&mut self) {
        if !self.is_active() {
            return;
        }

        info!("Shutting down ...");

        self.shutdown_thread();
        self.set_connection_status(ConnectionStatus::Disconnected);
        self.set_status(PlayerStatus::NoMedia);

        unsafe { &mut *self.qobject }.activeChanged(false);
    }

    // #[slot]
    pub fn play(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.play()
        }
    }

    // #[slot]
    pub fn pause(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.pause()
        }
    }

    // #[slot]
    pub fn next(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.next()
        }
    }

    // #[slot]
    pub fn previous(&mut self) {
        if let Some(ref thread) = &self.thread {
            thread.previous()
        }
    }

    fn shutdown_thread(&mut self) {
        if let Some(thread) = std::mem::replace(&mut self.thread, None) {
            thread.shutdown()
        }
    }

    // active

    pub fn is_active(&self) -> bool {
        self.thread.is_some()
    }

    // error

    pub fn error_kind(&self) -> QString {
        self.error_kind.to_qstring()
    }

    pub fn error_string(&self) -> QString {
        self.error_string.to_qstring()
    }

    fn set_error(&mut self, err: LibrespotError) {
        let message = format!("{}", &err);
        let kind = format!("{:?}", &err);

        error!("Librespot error: {}", &message);
        self.error_kind = Some(kind);
        self.error_string = Some(message);

        unsafe { &mut *self.qobject }.errorOccurred();
    }

    // status

    pub fn status(&self) -> i32 {
        self.status as i32
    }

    pub fn set_status(&mut self, status: PlayerStatus) {
        if self.status != status {
            self.status = status;

            unsafe { &mut *self.qobject }.statusChanged(self.status as i32);
        }
    }

    // connection status

    pub fn connection_status(&self) -> i32 {
        self.connection as i32
    }

    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        if self.connection != status {
            self.connection = status;

            unsafe { &mut *self.qobject }.connectionStatusChanged(self.connection as i32);
        }
    }

    // track uri

    pub fn track_uri(&self) -> QString {
        self.track.to_qstring()
    }

    pub fn set_track_uri(&mut self, uri: Option<String>) {
        if self.track != uri {
            self.track = uri;

            unsafe { &mut *self.qobject }.trackUriChanged(&self.track.to_qstring());
        }
    }

    // paused

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, value: bool) {
        if self.paused != value {
            self.paused = value;

            unsafe { &mut *self.qobject }.pausedChanged(value);
        }
    }

    // position

    pub fn position(&self) -> u32 {
        self.position_ms
    }

    pub fn set_position(&mut self, value: u32) {
        if self.position_ms != value {
            self.position_ms = value;

            unsafe { &mut *self.qobject }.positionChanged(value);
        }
    }

    // duration

    pub fn duration(&self) -> u32 {
        self.duration_ms
    }

    pub fn set_duration(&mut self, value: u32) {
        if self.duration_ms != value {
            self.duration_ms = value;

            unsafe { &mut *self.qobject }.durationChanged(value);
        }
    }

    // token

    pub fn token(&self) -> QString {
        self.token.to_qstring()
    }

    fn set_token(&mut self, value: Option<Token>) {
        self.token = value.map(|t| t.access_token);
        unsafe { &mut *self.qobject }.tokenChanged(&self.token.to_qstring());
    }
}

impl Drop for LibrespotPrivate {
    fn drop(&mut self) {
        self.shutdown_thread()
    }
}
