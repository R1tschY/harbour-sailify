use std::path::PathBuf;

#[must_use]
pub fn config_home() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .unwrap_or_else(|| {
            let mut path = std::env::var_os("HOME").expect("no home dir");
            path.push("/.config");
            path
        })
        .into()
}

#[must_use]
pub fn cache_home() -> PathBuf {
    std::env::var_os("XDG_CACHE_HOME")
        .unwrap_or_else(|| {
            let mut path = std::env::var_os("HOME").expect("no home dir");
            path.push("/.cache");
            path
        })
        .into()
}
