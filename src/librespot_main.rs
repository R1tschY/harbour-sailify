use futures::sync::mpsc::UnboundedReceiver;
use futures::{Async, Future, Poll, Stream};
use log::{error, info, trace, warn};
use sha1::{Digest, Sha1};
use std::env;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::time::Instant;
use tokio_core::reactor::{Core, Handle};
use tokio_io::IoStream;
use url::Url;

use librespot::core::authentication::{get_credentials, Credentials};
use librespot::core::cache::Cache;
use librespot::core::config::{ConnectConfig, DeviceType, SessionConfig, VolumeCtrl};
use librespot::core::session::Session;
use librespot::core::version;

use librespot::connect::discovery::{discovery, DiscoveryStream};
use librespot::connect::spirc::{Spirc, SpircTask};
use librespot::playback::audio_backend::{self, Sink};
use librespot::playback::config::{Bitrate, PlayerConfig};
use librespot::playback::mixer::{self, Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEvent};

fn device_id(name: &str) -> String {
    hex::encode(Sha1::digest(name.as_bytes()))
}

fn setup_logging(verbose: bool) {
    let mut builder = env_logger::Builder::new();
    match env::var("RUST_LOG") {
        Ok(config) => {
            builder.parse_filters(&config);
            builder.init();

            if verbose {
                warn!("`--verbose` flag overidden by `RUST_LOG` environment variable");
            }
        }
        Err(_) => {
            if verbose {
                builder.parse_filters("libmdns=info,librespot=trace");
            } else {
                builder.parse_filters("libmdns=info,librespot=info");
            }
            builder.init();
        }
    }
}

#[derive(Clone)]
struct Setup {
    backend: fn(Option<String>) -> Box<dyn Sink>,
    device: Option<String>,

    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,

    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    mixer_config: MixerConfig,
    credentials: Option<Credentials>,
    enable_discovery: bool,
    zeroconf_port: u16,
}

struct Options {
    cache: Option<PathBuf>,
    audio_cache: bool,
    device_name: String,
    device_type: DeviceType,
    bitrate: Bitrate,
    username: String,
    password: String,
    proxy: Option<String>,
    ap_port: Option<u16>,
    discovery: bool,
    backend: Option<String>,
    device: Option<String>,
    mixer: Option<String>,
    mixer_name: String,
    mixer_card: String,
    mixer_index: u32,
    mixer_linear_volume: bool,
    initial_volume: Option<u16>,
    zeroconf_port: u16,
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
            device_name: "".to_string(),
            device_type: DeviceType::default(),
            bitrate: Bitrate::default(),
            username: "".to_string(),
            password: "".to_string(),
            proxy: None,
            ap_port: None,
            discovery: true,
            backend: None,
            device: None,
            mixer: None,
            mixer_name: "PCM".to_string(),
            mixer_card: "default".to_string(),
            mixer_index: 0,
            mixer_linear_volume: false,
            initial_volume: None,
            zeroconf_port: 0,
            volume_normalisation: false,
            normalisation_pregain: None,
            volume_ctrl: VolumeCtrl::default(),
            autoplay: false,
            gapless: true
        }
    }
}


fn setup() -> Setup {
    setup_logging(true);

    info!(
        "sailify/{} librespot/{}",
        env!("CARGO_PKG_VERSION"),
        version::semver(),
    );

    let mut opts = Options::new();
    opts.username = env::var("SPOTIFY_USERNAME").expect("SPOTIFY_USERNAME env var missing");
    opts.password = env::var("SPOTIFY_PASSWORD").expect("SPOTIFY_PASSWORD env var missing");

    let backend = audio_backend::find(opts.backend).expect("Invalid backend");

    let mixer = mixer::find(opts.mixer.as_ref()).expect("Invalid mixer");

    let mixer_config = MixerConfig {
        card: opts.mixer_card,
        mixer: opts.mixer_name,
        index: opts.mixer_index,
        mapped_volume: !opts.mixer_linear_volume,
    };

    let audio_cache: bool = opts.audio_cache;
    let cache = opts.cache
        .map(|cache_location| Cache::new(cache_location, audio_cache));

    let initial_volume = opts.initial_volume
        .map(|volume| {
            if volume > 100 {
                panic!("Initial volume must be in the range 0-100");
            }
            (volume as i32 * 0xFFFF / 100) as u16
        })
        .or_else(|| cache.as_ref().and_then(Cache::volume))
        .unwrap_or(0x8000);

    let credentials = {
        let cached_credentials = cache.as_ref().and_then(Cache::credentials);

        let password = |_: &String| -> String {
            panic!();
        };

        get_credentials(
            Some(opts.username),
            Some(opts.password),
            cached_credentials,
            password,
        )
    };

    let session_config = {
        let device_id = device_id(&opts.device_name);

        SessionConfig {
            user_agent: version::version_string(),
            device_id: device_id,
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
        }
    };

    let player_config = {
        PlayerConfig {
            bitrate: opts.bitrate,
            gapless: opts.gapless,
            normalisation: opts.volume_normalisation,
            normalisation_pregain: opts.normalisation_pregain
                .unwrap_or(PlayerConfig::default().normalisation_pregain),
        }
    };

    let connect_config = ConnectConfig {
        name: opts.device_name,
        device_type: opts.device_type,
        volume: initial_volume,
        volume_ctrl: opts.volume_ctrl,
        autoplay: opts.autoplay,
    };

    Setup {
        backend: backend,
        cache: cache,
        session_config: session_config,
        player_config: player_config,
        connect_config: connect_config,
        credentials: credentials,
        device: opts.device,
        enable_discovery: opts.discovery,
        zeroconf_port: opts.zeroconf_port,
        mixer: mixer,
        mixer_config: mixer_config,
    }
}

struct Main {
    cache: Option<Cache>,
    player_config: PlayerConfig,
    session_config: SessionConfig,
    connect_config: ConnectConfig,
    backend: fn(Option<String>) -> Box<dyn Sink>,
    device: Option<String>,
    mixer: fn(Option<MixerConfig>) -> Box<dyn Mixer>,
    mixer_config: MixerConfig,
    handle: Handle,

    discovery: Option<DiscoveryStream>,
    signal: IoStream<()>,

    spirc: Option<Spirc>,
    spirc_task: Option<SpircTask>,
    connect: Box<dyn Future<Item = Session, Error = io::Error>>,

    shutdown: bool,
    last_credentials: Option<Credentials>,
    auto_connect_times: Vec<Instant>,

    player_event_channel: Option<UnboundedReceiver<PlayerEvent>>,
}

impl Main {
    fn new(handle: Handle, setup: Setup) -> Main {
        let mut task = Main {
            handle: handle.clone(),
            cache: setup.cache,
            session_config: setup.session_config,
            player_config: setup.player_config,
            connect_config: setup.connect_config,
            backend: setup.backend,
            device: setup.device,
            mixer: setup.mixer,
            mixer_config: setup.mixer_config,

            connect: Box::new(futures::future::empty()),
            discovery: None,
            spirc: None,
            spirc_task: None,
            shutdown: false,
            last_credentials: None,
            auto_connect_times: Vec::new(),
            signal: Box::new(tokio_signal::ctrl_c().flatten_stream()),

            player_event_channel: None,
        };

        if setup.enable_discovery {
            let config = task.connect_config.clone();
            let device_id = task.session_config.device_id.clone();

            task.discovery =
                Some(discovery(&handle, config, device_id, setup.zeroconf_port).unwrap());
        }

        if let Some(credentials) = setup.credentials {
            task.credentials(credentials);
        }

        task
    }

    fn credentials(&mut self, credentials: Credentials) {
        self.last_credentials = Some(credentials.clone());
        let config = self.session_config.clone();
        let handle = self.handle.clone();

        let connection = Session::connect(config, credentials, self.cache.clone(), handle);

        self.connect = connection;
        self.spirc = None;
        let task = mem::replace(&mut self.spirc_task, None);
        if let Some(task) = task {
            self.handle.spawn(task);
        }
    }
}

impl Future for Main {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        loop {
            let mut progress = false;

            if let Some(Async::Ready(Some(creds))) =
            self.discovery.as_mut().map(|d| d.poll().unwrap())
            {
                if let Some(ref spirc) = self.spirc {
                    spirc.shutdown();
                }
                self.auto_connect_times.clear();
                self.credentials(creds);

                progress = true;
            }

            match self.connect.poll() {
                Ok(Async::Ready(session)) => {
                    self.connect = Box::new(futures::future::empty());
                    let mixer_config = self.mixer_config.clone();
                    let mixer = (self.mixer)(Some(mixer_config));
                    let player_config = self.player_config.clone();
                    let connect_config = self.connect_config.clone();

                    let audio_filter = mixer.get_audio_filter();
                    let backend = self.backend;
                    let device = self.device.clone();
                    let (player, event_channel) =
                        Player::new(player_config, session.clone(), audio_filter, move || {
                            (backend)(device)
                        });


                    let (spirc, spirc_task) = Spirc::new(connect_config, session, player, mixer);
                    self.spirc = Some(spirc);
                    self.spirc_task = Some(spirc_task);
                    self.player_event_channel = Some(event_channel);

                    progress = true;
                }
                Ok(Async::NotReady) => (),
                Err(error) => {
                    error!("Could not connect to server: {}", error);
                    self.connect = Box::new(futures::future::empty());
                }
            }

            if let Async::Ready(Some(())) = self.signal.poll().unwrap() {
                trace!("Ctrl-C received");
                if !self.shutdown {
                    if let Some(ref spirc) = self.spirc {
                        spirc.shutdown();
                    } else {
                        return Ok(Async::Ready(()));
                    }
                    self.shutdown = true;
                } else {
                    return Ok(Async::Ready(()));
                }

                progress = true;
            }

            let mut drop_spirc_and_try_to_reconnect = false;
            if let Some(ref mut spirc_task) = self.spirc_task {
                if let Async::Ready(()) = spirc_task.poll().unwrap() {
                    if self.shutdown {
                        return Ok(Async::Ready(()));
                    } else {
                        warn!("Spirc shut down unexpectedly");
                        drop_spirc_and_try_to_reconnect = true;
                    }
                    progress = true;
                }
            }
            if drop_spirc_and_try_to_reconnect {
                self.spirc_task = None;
                let now = Instant::now();
                while (!self.auto_connect_times.is_empty())
                    && ((now - self.auto_connect_times[0]).as_secs() > 600)
                {
                    let _ = self.auto_connect_times.remove(0);
                }

                if let Some(credentials) = self.last_credentials.clone() {
                    if self.auto_connect_times.len() >= 5 {
                        warn!("Spirc shut down too often. Not reconnecting automatically.");
                    } else {
                        self.auto_connect_times.push(now);
                        self.credentials(credentials);
                    }
                }
            }

            if let Some(ref mut player_event_channel) = self.player_event_channel {
                if let Async::Ready(Some(event)) = player_event_channel.poll().unwrap() {
                    progress = true;
                    info!("Player event: {:?}", event)
                }
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}

fn main() {
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "full")
    }

    let mut core = Core::new().unwrap();
    core.run(Main::new(core.handle(), setup())).unwrap()
}
