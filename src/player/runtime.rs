use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs, io, panic, thread};

use futures::channel::mpsc::{unbounded, UnboundedSender};
use librespot_core::authentication::Credentials;
use librespot_core::cache::Cache;
use librespot_core::config::{ConnectConfig, DeviceType, SessionConfig};
use librespot_core::version;
use librespot_playback::config::{PlayerConfig, VolumeCtrl};
use librespot_playback::mixer::MixerConfig;
use librespot_playback::{audio_backend, mixer};
use log::{error, info, warn};
use tokio::runtime::Builder;

use crate::player::controller::{ControlMessage, LibrespotConfig, LibrespotController};
use crate::player::error::{LibrespotError, LibrespotResult};
use crate::player::events::{LibrespotEvent, LibrespotEventListener};
use crate::player::options::Options;

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
        volume_ctrl: VolumeCtrl::default(),
    };

    let cache = Cache::new(opts.system_cache, opts.audio_cache, opts.cache_size_limit)?;

    let initial_volume = opts
        .initial_volume
        .map(|volume| {
            assert!(volume <= 100, "Initial volume must be in the range 0-100");
            (i32::from(volume) * 0xFFFF / 100) as u16
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
        ..PlayerConfig::default()
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

pub struct PlayerRuntime {
    handle: JoinHandle<()>,
    control: UnboundedSender<ControlMessage>,
}

impl PlayerRuntime {
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

    pub fn start(
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
                        .enable_time()
                        .enable_io()
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
                    core.block_on(controller_future);
                    info!("CORE END");
                });
                if let Err(err) = result {
                    let message = if let Some(s) = err.downcast_ref::<&str>() {
                        (*s).to_string()
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

        Ok(PlayerRuntime {
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
            info!("join shutdown");
            self.handle.join().unwrap();
            info!("joined shutdown");
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
