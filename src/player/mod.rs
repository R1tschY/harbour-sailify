use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;

use futures::channel::mpsc::{unbounded, UnboundedSender};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::version;
use librespot::playback::audio_backend;
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::playback::mixer::{self, MixerConfig};
use log::{error, info, warn};
use sha1::{Digest, Sha1};
use tokio_core::reactor::Core;

use crate::player::controller::{ControlMessage, LibrespotConfig, LibrespotController};
use crate::player::error::{LibrespotError, LibrespotResult};
use crate::player::qtgateway::LibrespotGateway;
use crate::utils::{xdg, UnsafeSend};

pub mod controller;
pub mod error;
pub mod qobject;
pub mod qtgateway;

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

pub const CLIENT_ID: &str = env!("SAILIFY_CLIENT_ID");

pub const SCOPES: &str = "user-read-private,\
playlist-read-private,\
playlist-read-collaborative,\
user-library-read,\
user-library-modify,\
user-top-read,\
user-read-recently-played";

#[derive(Clone)]
pub struct Options {
    pub cache: PathBuf,
    pub audio_cache: bool,
    pub device_name: String,
    pub bitrate: Bitrate,
    pub username: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub ap_port: Option<u16>,
    pub backend: Option<String>,
    pub device: Option<String>,
    pub mixer: Option<String>,
    pub mixer_name: String,
    pub mixer_card: String,
    pub mixer_index: u32,
    pub mixer_linear_volume: bool,
    pub initial_volume: Option<u16>,
    pub volume_normalisation: bool,
    pub normalisation_pregain: Option<f32>,
    pub volume_ctrl: VolumeCtrl,
    pub autoplay: bool,
    pub gapless: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            cache: xdg::config_home().join("harbour-sailify").join("librespot"),
            audio_cache: true,
            device_name: "Sailify".to_string(),
            bitrate: Bitrate::default(),
            username: None,
            password: None,
            proxy: None,
            ap_port: None,
            backend: None,
            device: None,
            mixer: None,
            mixer_name: "PCM".to_string(),
            mixer_card: "default".to_string(),
            mixer_index: 0,
            mixer_linear_volume: false,
            initial_volume: None,
            volume_normalisation: false,
            normalisation_pregain: None,
            volume_ctrl: VolumeCtrl::default(),
            autoplay: false,
            gapless: true,
        }
    }
}

fn setup(opts: Options) -> LibrespotResult<LibrespotConfig> {
    info!(
        "sailify/{} librespot/{}",
        env!("CARGO_PKG_VERSION"),
        version::semver(),
    );

    let backend = audio_backend::find(opts.backend.clone()).ok_or_else(|| {
        LibrespotError::IllegalConfig(format!("Invalid backend {:?}", &opts.backend))
    })?;

    let mixer = mixer::find(opts.mixer.as_ref())
        .ok_or_else(|| LibrespotError::IllegalConfig(format!("Invalid mixer {:?}", &opts.mixer)))?;

    let mixer_config = MixerConfig {
        card: opts.mixer_card,
        mixer: opts.mixer_name,
        index: opts.mixer_index,
        mapped_volume: !opts.mixer_linear_volume,
    };

    let audio_cache: bool = opts.audio_cache;
    let cache = Cache::new(opts.cache, audio_cache);

    let initial_volume = opts
        .initial_volume
        .map(|volume| {
            if volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            (volume as i32 * 0xFFFF / 100) as u16
        })
        .or_else(|| Cache::volume(&cache))
        .unwrap_or(0x8000);

    let credentials = match (opts.username, opts.password) {
        (Some(username), Some(password)) => Credentials::with_password(username, password),
        _ => cache
            .credentials()
            .ok_or(LibrespotError::MissingCredentials)?,
    };

    let session_config = SessionConfig {
        user_agent: version::version_string(),
        device_id: device_id(&opts.device_name),
        proxy: None,
        ap_port: opts.ap_port,
    };

    let player_config = PlayerConfig {
        bitrate: opts.bitrate,
        gapless: opts.gapless,
        normalisation: opts.volume_normalisation,
        normalisation_pregain: opts
            .normalisation_pregain
            .unwrap_or(PlayerConfig::default().normalisation_pregain),
    };

    let connect_config = ConnectConfig {
        name: opts.device_name,
        device_type: DeviceType::Smartphone,
        volume: initial_volume,
        volume_ctrl: opts.volume_ctrl,
        autoplay: opts.autoplay,
    };

    Ok(LibrespotConfig {
        backend,
        cache,
        session_config,
        player_config,
        connect_config,
        credentials,
        device: opts.device,
        mixer,
        mixer_config,
    })
}

pub struct LibrespotThread {
    handle: JoinHandle<()>,
    control: UnboundedSender<ControlMessage>,
}

impl LibrespotThread {
    pub fn remove_credentials(opts: &Options) {
        match fs::remove_file(opts.cache.join("credentials.json")) {
            Ok(_) => (),
            Err(err) if err.kind() == io::ErrorKind::NotFound => (),
            // TODO: what should we do?
            Err(err) => error!("Failed to remove credentials: {:?}", err),
        };
    }

    pub fn run(gateway: LibrespotGateway, options: Options) -> LibrespotResult<Self> {
        let sendable_gateway = UnsafeSend::new(gateway);
        let setup = setup(options)?;

        let (control_tx, control_rx) = unbounded();
        let control_tx_ = control_tx.clone();
        let handle = thread::Builder::new()
            .name("librespot".to_string())
            .spawn(move || {
                let core = Core::new().unwrap();
                LibrespotController::new(
                    core.handle(),
                    control_tx,
                    control_rx,
                    unsafe { sendable_gateway.unwrap() },
                    setup,
                );
            })
            .unwrap();

        Ok(LibrespotThread {
            handle,
            control: control_tx_,
        })
    }

    pub fn shutdown(self) {
        if let Err(_) = self.control.unbounded_send(ControlMessage::Shutdown) {
            warn!("Shutdown could not send because thread is already dead");
        } else {
            self.handle.join().unwrap();
        }
    }

    pub fn play(&self) {
        let _ = self.control.unbounded_send(ControlMessage::Play);
    }

    pub fn pause(&self) {
        let _ = self.control.unbounded_send(ControlMessage::Pause);
    }

    pub fn next(&self) {
        let _ = self.control.unbounded_send(ControlMessage::Next);
    }

    pub fn previous(&self) {
        let _ = self.control.unbounded_send(ControlMessage::Previous);
    }
}
