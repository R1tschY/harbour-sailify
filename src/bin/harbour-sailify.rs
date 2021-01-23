use std::{env, fs, io};

use qt5qml::core::QApplicationFactory;
use sailfishapp::SailfishApp;

use sailify::player::qobject::register_librespot;
use sailify::utils::xdg;

fn setup_logging() {
    let rust_log =
        env::var("RUST_LOG").unwrap_or("libmdns=info,librespot=info,sailify=debug".to_string());

    env_logger::Builder::new().parse_filters(&rust_log).init();
}

fn create_config_dir() {
    if let Err(err) = fs::create_dir(xdg::config_home().join("harbour-sailify")) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            panic!("Failed to create config dir: {:?}", err);
        }
    }
}

fn main() {
    setup_logging();

    let app = SailfishApp::new_from_env_args();

    create_config_dir();

    register_librespot();

    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}
