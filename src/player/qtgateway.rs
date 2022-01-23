use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use crate::player::error::LibrespotError;
use librespot_core::keymaster::Token;
use librespot_playback::player::PlayerEvent;

#[derive(Debug)]
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
        token: Result<Token, String>,
    },
    Error {
        err: LibrespotError,
    },
    Panic {
        message: String,
    },
}

impl LibrespotEvent {
    #[must_use]
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

pub trait LibrespotEventListener: RefUnwindSafe + UnwindSafe + Sync + Send {
    fn notify(&self, evt: LibrespotEvent);
}

pub type LibrespotEventListenerRef = Arc<dyn LibrespotEventListener>;
