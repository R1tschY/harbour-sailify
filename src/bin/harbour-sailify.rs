use log::warn;
use qt5qml::core::{QApplicationFactory, QObjectRef};
use sailfishapp::SailfishApp;
use sailify::player::qobject::Librespot;
use sailify::player::LibrespotThread;
use std::env;

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

fn main() {
    setup_logging(true);

    let mut app = SailfishApp::new_from_env_args();

    let _librespot = Librespot::new(app.as_qobject_mut());

    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}
