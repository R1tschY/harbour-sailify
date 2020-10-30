use qt5qml::core::QApplicationFactory;
use sailfishapp::SailfishApp;
use sailify::player::qobject::register_librespot;
use std::env;

fn setup_logging() {
    let mut builder = env_logger::Builder::new();
    match env::var("RUST_LOG") {
        Ok(config) => {
            builder.parse_filters(&config);
            builder.init();
        }
        Err(_) => {
            builder.parse_filters("libmdns=info,librespot=info,sailify=debug");
            builder.init();
        }
    }
}

fn main() {
    setup_logging();

    let app = SailfishApp::new_from_env_args();

    register_librespot();

    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}
