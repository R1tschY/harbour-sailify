use qt5qml::core::QApplicationFactory;
use sailfishapp::SailfishApp;

fn main() {
    let app = SailfishApp::new_from_env_args();
    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}