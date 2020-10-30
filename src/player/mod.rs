use std::env;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{Async, Future, Poll, Stream};
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::session::Session;
use librespot::core::version;
use librespot::playback::audio_backend::{self, Sink};
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::playback::mixer::{self, Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};
use log::{error, info, warn};
use qt5qml::core::{QByteArray, QString};
use qt5qml::QBox;
use sha1::{Digest, Sha1};
use tokio_core::reactor::{Core, Handle};
use url::Url;

use crate::player::controller::{ControlMessage, Controller, LibrespotConfig};
use crate::player::qtgateway::{serialize_event, LibrespotGateway};
use crate::utils::UnsafeSend;

pub mod controller;
pub mod qobject;
pub mod qtgateway;

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

pub struct Options {
    cache: Option<PathBuf>,
    audio_cache: bool,
    device_name: String,
    bitrate: Bitrate,
    username: String,
    password: String,
    proxy: Option<String>,
    ap_port: Option<u16>,
    backend: Option<String>,
    device: Option<String>,
    mixer: Option<String>,
    mixer_name: String,
    mixer_card: String,
    mixer_index: u32,
    mixer_linear_volume: bool,
    initial_volume: Option<u16>,
    volume_normalisation: bool,
    normalisation_pregain: Option<f32>,
    volume_ctrl: VolumeCtrl,
    autoplay: bool,
    gapless: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            cache: None,
            audio_cache: true,
            device_name: "Sailify".to_string(),
            bitrate: Bitrate::default(),
            username: "".to_string(),
            password: "".to_string(),
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

fn setup(opts: Options) -> LibrespotConfig {
    info!(
        "sailify/{} librespot/{}",
        env!("CARGO_PKG_VERSION"),
        version::semver(),
    );

    let backend = audio_backend::find(opts.backend).expect("Invalid backend");

    let mixer = mixer::find(opts.mixer.as_ref()).expect("Invalid mixer");

    let mixer_config = MixerConfig {
        card: opts.mixer_card,
        mixer: opts.mixer_name,
        index: opts.mixer_index,
        mapped_volume: !opts.mixer_linear_volume,
    };

    let audio_cache: bool = opts.audio_cache;
    let cache = opts
        .cache
        .map(|cache_location| Cache::new(cache_location, audio_cache));

    let initial_volume = opts
        .initial_volume
        .map(|volume| {
            if volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            (volume as i32 * 0xFFFF / 100) as u16
        })
        .or_else(|| cache.as_ref().and_then(Cache::volume))
        .unwrap_or(0x8000);

    //let cached_credentials = cache.as_ref().and_then(Cache::credentials);
    let credentials = Credentials::with_password(opts.username, opts.password);

    let session_config = SessionConfig {
        user_agent: version::version_string(),
        device_id: device_id(&opts.device_name),
        proxy: opts.proxy.or(std::env::var("http_proxy").ok()).map(
            |s| {
                match Url::parse(&s) {
                    Ok(url) => {
                        if url.host().is_none() || url.port_or_known_default().is_none() {
                            panic!("Invalid proxy url, only urls on the format \"http://host:port\" are allowed");
                        }

                        if url.scheme() != "http" {
                            panic!("Only unsecure http:// proxies are supported");
                        }
                        url
                    },
                    Err(err) => panic!("Invalid proxy url: {}, only urls on the format \"http://host:port\" are allowed", err)
                }
            },
        ),
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

    LibrespotConfig {
        backend,
        cache,
        session_config,
        player_config,
        connect_config,
        credentials: Some(credentials),
        device: opts.device,
        mixer,
        mixer_config,
    }
}

pub struct LibrespotThread {
    handle: JoinHandle<()>,
    control: UnboundedSender<ControlMessage>,
}

impl LibrespotThread {
    pub fn run(gateway: QBox<LibrespotGateway>, options: Options) -> Self {
        let sendable_gateway = UnsafeSend::new(gateway);
        let (control_tx, control_rx) = futures::sync::mpsc::unbounded();
        let handle = thread::Builder::new()
            .name("librespot".to_string())
            .spawn(move || {
                let mut core = Core::new().unwrap();
                core.run(Controller::new(
                    core.handle(),
                    control_rx,
                    unsafe { sendable_gateway.unwrap() },
                    setup(options),
                ))
                .unwrap();
            })
            .unwrap();

        LibrespotThread {
            handle,
            control: control_tx,
        }
    }

    pub fn shutdown(self) {
        if let Err(_) = self.control.unbounded_send(ControlMessage::Shutdown) {
            warn!("Shutdown could not send because thread is already dead");
        } else {
            self.handle.join().unwrap();
        }
    }
}
