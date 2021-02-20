use std::ptr;
use std::sync::mpsc;

use librespot::core::keymaster::Token;
use librespot::playback::player::PlayerEvent;
use log::warn;
use qt5qml::core::{ConnectionTypeKind, QObject, QObjectRef};
use qt5qml::{signal, slot, QBox};

#[derive(Debug, Clone)]
pub enum LibrespotEvent {
    Stopped {
        play_request_id: u64,
        track_id: String,
    },
    Changed {
        new_track_id: String,
    },
    Loading {
        play_request_id: u64,
        track_id: String,
        position_ms: u32,
    },
    Playing {
        play_request_id: u64,
        track_id: String,
        position_ms: u32,
        duration_ms: u32,
    },
    Paused {
        play_request_id: u64,
        track_id: String,
        position_ms: u32,
        duration_ms: u32,
    },
    Unavailable {
        play_request_id: u64,
        track_id: String,
    },
    VolumeSet {
        volume: u16,
    },
    Connecting,
    Connected,
    ConnectionError {
        message: String,
    },
    Shutdown,
    StartReconnect,
    TokenChanged {
        token: Option<Token>,
    },
}

impl LibrespotEvent {
    pub fn from_event(evt: PlayerEvent) -> Option<Self> {
        Some(match evt {
            PlayerEvent::Playing {
                play_request_id,
                track_id,
                position_ms,
                duration_ms,
            } => LibrespotEvent::Playing {
                play_request_id,
                track_id: track_id.to_uri(),
                position_ms,
                duration_ms,
            },
            PlayerEvent::Changed { new_track_id, .. } => LibrespotEvent::Changed {
                new_track_id: new_track_id.to_uri(),
            },
            PlayerEvent::Loading {
                track_id,
                play_request_id,
                position_ms,
            } => LibrespotEvent::Loading {
                track_id: track_id.to_uri(),
                play_request_id,
                position_ms,
            },
            PlayerEvent::Paused {
                track_id,
                position_ms,
                play_request_id,
                duration_ms,
            } => LibrespotEvent::Paused {
                track_id: track_id.to_uri(),
                position_ms,
                play_request_id,
                duration_ms,
            },
            PlayerEvent::Stopped {
                play_request_id,
                track_id,
            } => LibrespotEvent::Stopped {
                play_request_id,
                track_id: track_id.to_uri(),
            },
            PlayerEvent::Unavailable {
                track_id,
                play_request_id,
            } => LibrespotEvent::Unavailable {
                play_request_id,
                track_id: track_id.to_uri(),
            },
            PlayerEvent::VolumeSet { volume } => LibrespotEvent::VolumeSet { volume },
            _ => return None,
        })
    }
}

mod details {
    include!(concat!(env!("OUT_DIR"), "/qffi_LibrespotGateway.rs"));

    pub struct LibrespotGatewayPrivate;

    impl LibrespotGatewayPrivate {
        pub fn new(_: *mut LibrespotGateway) -> Self {
            Self
        }
    }
}

pub struct LibrespotGateway {
    qobject: QBox<details::LibrespotGateway>,
    tx: mpsc::Sender<LibrespotEvent>,
}

impl LibrespotGateway {
    pub fn new(parent: &QObject, tx: mpsc::Sender<LibrespotEvent>) -> Self {
        let mut qobject = details::LibrespotGateway::new(ptr::null_mut());

        QObject::connect(
            qobject.as_qobject(),
            signal!("playerEvent()"),
            parent,
            slot!("_onPlayerEvent()"),
            ConnectionTypeKind::Queued,
        );
        qobject.move_to_thread(None);

        Self { qobject, tx }
    }

    pub fn send(&mut self, evt: LibrespotEvent) {
        if let Err(_) = self.tx.send(evt) {
            warn!("Failed to send librespot event to qt thread");
        } else {
            self.qobject.playerEvent();
        }
    }
}
