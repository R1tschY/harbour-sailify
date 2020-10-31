use librespot::playback::player::PlayerEvent;
use serde::{Deserialize, Serialize};

include!(concat!(env!("OUT_DIR"), "/qffi_LibrespotGateway.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn serialize_event(evt: LibrespotEvent) -> Vec<u8> {
    bincode::serialize(&evt).unwrap()
}

pub fn deserialize_event(evt: &[u8]) -> LibrespotEvent {
    bincode::deserialize(&evt).unwrap()
}

pub struct LibrespotGatewayPrivate {
    _qobject: *mut LibrespotGateway,
}

impl LibrespotGatewayPrivate {
    pub fn new(qobject: *mut LibrespotGateway) -> Self {
        Self { _qobject: qobject }
    }
}
