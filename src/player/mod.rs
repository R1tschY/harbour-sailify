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

use crate::player::controller::{ControlMessage, LibrespotConfig, LibrespotController};
use crate::player::error::{LibrespotError, LibrespotResult};
use crate::player::qtgateway::{LibrespotEvent, LibrespotEventListener};
use crate::utils::xdg;

mod bindings;
pub mod controller;
pub mod error;
pub mod qobject;
pub mod qtgateway;

pub const CLIENT_ID: &str = env!("SAILIFY_CLIENT_ID");

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

#[derive(Clone)]
pub struct Options {
    pub system_cache: Option<PathBuf>,
    pub audio_cache: Option<PathBuf>,
    pub device_name: String,
    pub device_id: String,
    pub bitrate: Bitrate,
    pub username: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub ap_port: Option<u16>,
    pub format: AudioFormat,
    pub backend: Option<String>,
    pub backend_device: Option<String>,
    pub mixer: Option<String>,
    pub mixer_name: String,
    pub mixer_card: String,
    pub mixer_index: u32,
    pub initial_volume: Option<u16>,
    pub volume_normalisation: bool,
    pub normalisation_pregain: Option<f64>,
    pub volume_ctrl: VolumeCtrl,
    pub autoplay: bool,
    pub gapless: bool,
    pub cache_size_limit: Option<u64>,
}

impl Default for Options {
    fn default() -> Self {
        let hw_name = OsRelease::new_from("/etc/hw-release")
            .ok()
            .map(|hw| hw.name)
            .unwrap_or_else(|| "Sailfish OS".to_string());
        let cache_dir = xdg::config_home().join("harbour-sailify").join("librespot");

        let device_id_path = cache_dir.join("device_id");
        let device_id = if let Ok(device_id) = fs::read_to_string(&device_id_path) {
            device_id
        } else {
            let mut buffer = Uuid::encode_buffer();
            let device_id = Uuid::new_v4().to_simple().encode_lower(&mut buffer);

            fs::create_dir_all(&cache_dir).unwrap();
            fs::write(&device_id_path, &device_id).unwrap();
            device_id.to_string()
        };

        Self {
            audio_cache: Some(cache_dir.join("files")),
            system_cache: Some(cache_dir),
            device_name: hw_name,
            device_id,
            bitrate: Bitrate::default(),
            username: None,
            password: None,
            proxy: None,
            ap_port: None,
            format: Default::default(),
            backend: None,
            backend_device: None,
            mixer: None,
            mixer_name: "PCM".to_string(),
            mixer_card: "default".to_string(),
            mixer_index: 0,
            initial_volume: None,
            volume_normalisation: false,
            normalisation_pregain: None,
            volume_ctrl: VolumeCtrl::default(),
            autoplay: false,
            gapless: true,
            cache_size_limit: Some(2 * 1024 * 1024 * 1024),
        }
    }
}

fn setup(opts: Options) -> LibrespotResult<LibrespotConfig> {
    info!(
        "sailify/{} librespot/{}",
        env!("CARGO_PKG_VERSION"),
        version::SEMVER,
    );

    let backend = audio_backend::find(opts.backend.clone()).ok_or_else(|| {
        LibrespotError::IllegalConfig(format!("Invalid backend {:?}", &opts.backend))
    })?;

    let mixer = mixer::find(opts.mixer.as_ref().map(|s| s as &str))
        .ok_or_else(|| LibrespotError::IllegalConfig(format!("Invalid mixer {:?}", &opts.mixer)))?;

    let mixer_config = MixerConfig {
        device: opts.mixer_card,
        index: opts.mixer_index,
        control: opts.mixer_name,
        volume_ctrl: Default::default(),
    };

    let cache = Cache::new(opts.system_cache, opts.audio_cache, opts.cache_size_limit)?;

    let initial_volume = opts
        .initial_volume
        .map(|volume| {
            if volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            (volume as i32 * 0xFFFF / 100) as u16
        })
        .or_else(|| cache.volume());

    let credentials = match (opts.username, opts.password) {
        (Some(username), Some(password)) => Credentials::with_password(username, password),
        _ => cache
            .credentials()
            .ok_or(LibrespotError::MissingCredentials)?,
    };

    let session_config = SessionConfig {
        user_agent: version::VERSION_STRING.to_string(),
        device_id: opts.device_id,
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
        ..Default::default()
    };

    let connect_config = ConnectConfig {
        name: opts.device_name,
        device_type: DeviceType::Smartphone,
        initial_volume,
        has_volume_ctrl: !matches!(mixer_config.volume_ctrl, VolumeCtrl::Fixed),
        autoplay: opts.autoplay,
    };

    Ok(LibrespotConfig {
        format: opts.format,
        backend,
        cache,
        session_config,
        player_config,
        connect_config,
        credentials,
        device: opts.backend_device,
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
        if let Some(system_cache) = &opts.system_cache {
            match fs::remove_file(system_cache.join("credentials.json")) {
                Ok(_) => (),
                Err(err) if err.kind() == io::ErrorKind::NotFound => (),
                // TODO: what should we do?
                Err(err) => error!("Failed to remove credentials: {:?}", err),
            };
        }
    }

    pub fn run(
        listener: Arc<dyn LibrespotEventListener>,
        options: Options,
    ) -> LibrespotResult<Self> {
        let setup = setup(options)?;

        let (control_tx, control_rx) = unbounded();
        let control_tx_ = control_tx.clone();

        let x = Mutex::new((control_tx, control_rx));
        let handle = thread::Builder::new()
            .name("librespot".to_string())
            .spawn(move || {
                let listener_clone = listener.clone();
                let result = panic::catch_unwind(move || {
                    info!("CORE START");
                    let core = Builder::new_current_thread()
                        .thread_name("librespot-runtime")
                        .build()
                        .unwrap();
                    let (control_tx, control_rx) = x.into_inner().unwrap();

                    let controller_future = LibrespotController::run(
                        core.handle().clone(),
                        control_tx,
                        control_rx,
                        listener_clone,
                        setup,
                    );
                    let _ = core.block_on(controller_future);
                    info!("CORE END");
                });
                if let Err(err) = result {
                    let message = if let Some(s) = err.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "Unknown internal error".to_string()
                    };

                    info!("CORE CRASH: {}", message);
                    listener.notify(LibrespotEvent::Panic { message });
                    panic::resume_unwind(err);
                }
            })
            .unwrap();

        Ok(LibrespotThread {
            handle,
            control: control_tx_,
        })
    }

    pub fn shutdown(self) {
        if self
            .control
            .unbounded_send(ControlMessage::Shutdown)
            .is_err()
        {
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

    pub fn refresh_token(&self) {
        let _ = self.control.unbounded_send(ControlMessage::RefreshToken);
    }
}
