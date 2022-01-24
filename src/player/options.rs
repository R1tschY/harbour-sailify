use std::fs;
use std::path::PathBuf;

use librespot_playback::config::{AudioFormat, Bitrate, VolumeCtrl};
use os_release::OsRelease;
use uuid::Uuid;

use crate::utils::xdg;

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
            .map_or_else(|| "Sailfish OS".to_string(), |hw| hw.name);
        let cache_dir = xdg::config_home().join("harbour-sailify").join("librespot");

        let device_id_path = cache_dir.join("device_id");
        let device_id = if let Ok(device_id) = fs::read_to_string(&device_id_path) {
            device_id
        } else {
            let mut buffer = Uuid::encode_buffer();
            let device_id = Uuid::new_v4().to_simple().encode_lower(&mut buffer);

            fs::create_dir_all(&cache_dir).unwrap();
            fs::write(&device_id_path, &device_id).unwrap();
            (*device_id).to_string()
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
            format: AudioFormat::default(),
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
