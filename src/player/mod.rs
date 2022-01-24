use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::{env, panic};

use futures::channel::mpsc::{unbounded, UnboundedSender};
use librespot_core::authentication::Credentials;
use librespot_core::cache::Cache;
use librespot_core::config::{ConnectConfig, DeviceType, SessionConfig};
use librespot_core::version;
use librespot_playback::audio_backend;
use librespot_playback::config::{AudioFormat, Bitrate, PlayerConfig, VolumeCtrl};
use librespot_playback::mixer::{self, MixerConfig};
use log::{error, info, warn};
use os_release::OsRelease;
use tokio::runtime::Builder;
use uuid::Uuid;

use options::Options;

use crate::player::controller::{ControlMessage, LibrespotConfig, LibrespotController};
use crate::player::error::{LibrespotError, LibrespotResult};
use crate::player::events::{LibrespotEvent, LibrespotEventListener, LibrespotEventListenerRef};
use crate::player::runtime::PlayerRuntime;
use crate::utils::xdg;

mod bindings;
pub mod controller;
pub mod error;
pub mod events;
pub mod options;
pub mod runtime;

/// cbindgen:ignore
pub const CLIENT_ID: &str = env!("SAILIFY_CLIENT_ID");

/// cbindgen:ignore
pub const SCOPES: &str = "user-read-private,\
playlist-read-private,\
playlist-read-collaborative,\
user-library-read,\
user-library-modify,\
user-top-read,\
user-follow-read,\
user-follow-modify,\
user-read-recently-played,\
user-read-private,\
user-read-playback-state,\
user-read-currently-playing,\
user-modify-playback-state,\
streaming";

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
    thread: Option<PlayerRuntime>,
    options: Options,
    listener: LibrespotEventListenerRef,
}

fn setup_logging() {
    let rust_log = env::var("RUST_LOG")
        .unwrap_or_else(|_| "libmdns=info,librespot=info,sailify=debug".to_string());

    let _ = env_logger::Builder::new()
        .parse_filters(&rust_log)
        .try_init();
}

impl SailifyPlayer {
    #[must_use]
    pub fn new(listener: LibrespotEventListenerRef) -> Self {
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

        match PlayerRuntime::start(self.listener.clone(), self.options.clone()) {
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
        PlayerRuntime::remove_credentials(&self.options);
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
