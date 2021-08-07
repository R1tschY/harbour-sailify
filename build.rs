use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, QObjectSignal, TypeRef};
use qt5qml::core::QString;

fn main() {
    // Qt
    let core = pkg_config::probe_library("Qt5Core").unwrap();
    let qml = pkg_config::probe_library("Qt5Qml").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    cpp.flag("-std=gnu++11");
    for include in &core.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }
    for include in &qml.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("Librespot")
        .inherit(TypeRef::qobject())
        .property(
            QObjectProp::new::<QString>("username")
                .read("username")
                .write("setUsername"),
        )
        .property(
            QObjectProp::new::<QString>("password")
                .read("password")
                .write("setPassword"),
        )
        .property(
            QObjectProp::new::<bool>("active")
                .read("isActive")
                .notify("activeChanged"),
        )
        .property(
            QObjectProp::new::<QString>("errorString")
                .read("errorString")
                .notify("errorOccurred"),
        )
        .property(
            QObjectProp::new::<QString>("errorKind")
                .read("errorKind")
                .notify("errorOccurred"),
        )
        .property(
            QObjectProp::new::<i32>("mediaStatus")
                .read("mediaStatus")
                .notify("mediaStatusChanged"),
        )
        .property(
            QObjectProp::new::<i32>("connectionStatus")
                .read("connectionStatus")
                .notify("connectionStatusChanged"),
        )
        .property(
            QObjectProp::new::<QString>("trackUri")
                .read("trackUri")
                .notify("trackUriChanged"),
        )
        .property(
            QObjectProp::new::<QString>("playbackStatus")
                .read("playbackStatus")
                .notify("playbackStatusChanged"),
        )
        .property(
            QObjectProp::new::<u32>("position")
                .read("position")
                .notify("positionChanged"),
        )
        .property(
            QObjectProp::new::<u32>("duration")
                .read("duration")
                .notify("durationChanged"),
        )
        .property(
            QObjectProp::new::<QString>("accessToken")
                .read("accessToken")
                .notify("accessTokenChanged"),
        )
        .property(QObjectProp::new::<i32>("accessTokenExpiresIn").read("accessTokenExpiresIn"))
        .property(
            QObjectProp::new::<QString>("deviceId")
                .read("deviceId")
                .const_(),
        )
        .property(
            QObjectProp::new::<QString>("deviceName")
                .read("deviceName")
                .const_(),
        )
        // username
        .method(QObjectMethod::new("username").const_().ret::<QString>())
        .method(QObjectMethod::new("setUsername").arg::<&QString>("value"))
        // password
        .method(QObjectMethod::new("password").const_().ret::<QString>())
        .method(QObjectMethod::new("setPassword").arg::<&QString>("value"))
        // active
        .method(QObjectMethod::new("isActive").const_().ret::<bool>())
        .signal(QObjectSignal::new("activeChanged").arg::<bool>("value"))
        // error
        .method(QObjectMethod::new("errorString").const_().ret::<QString>())
        .method(QObjectMethod::new("errorKind").const_().ret::<QString>())
        .signal(QObjectSignal::new("errorOccurred"))
        // status
        .method(QObjectMethod::new("mediaStatus").const_().ret::<i32>())
        .signal(QObjectSignal::new("mediaStatusChanged").arg::<i32>("status"))
        // connection status
        .method(QObjectMethod::new("connectionStatus").const_().ret::<i32>())
        .signal(QObjectSignal::new("connectionStatusChanged").arg::<i32>("status"))
        // track uri
        .method(QObjectMethod::new("trackUri").const_().ret::<QString>())
        .signal(QObjectSignal::new("trackUriChanged").arg::<&QString>("value"))
        // paused
        .method(
            QObjectMethod::new("playbackStatus")
                .const_()
                .ret::<QString>(),
        )
        .signal(QObjectSignal::new("playbackStatusChanged"))
        // position
        .method(QObjectMethod::new("position").const_().ret::<u32>())
        .signal(QObjectSignal::new("positionChanged").arg::<u32>("value"))
        // duration
        .method(QObjectMethod::new("duration").const_().ret::<u32>())
        .signal(QObjectSignal::new("durationChanged").arg::<u32>("value"))
        // token
        .method(QObjectMethod::new("accessToken").const_().ret::<QString>())
        .method(
            QObjectMethod::new("accessTokenExpiresIn")
                .const_()
                .ret::<i32>(),
        )
        .signal(QObjectSignal::new("accessTokenChanged"))
        .slot(QObjectMethod::new("refreshAccessToken"))
        // device id
        .method(QObjectMethod::new("deviceId").const_().ret::<QString>())
        // device name
        .method(QObjectMethod::new("deviceName").const_().ret::<QString>())
        // slots
        .slot(QObjectMethod::new("login"))
        .slot(QObjectMethod::new("logout"))
        .slot(QObjectMethod::new("shutdown"))
        .slot(QObjectMethod::new("play"))
        .slot(QObjectMethod::new("pause"))
        .slot(QObjectMethod::new("next"))
        .slot(QObjectMethod::new("previous"))
        .slot(QObjectMethod::new("updatePosition"))
        // private slots
        .slot(QObjectMethod::new("_onPlayerEvent"))
        .build(&cpp, &moc);

    QObjectBuild::new("LibrespotGateway")
        .inherit(TypeRef::qobject())
        .signal(QObjectSignal::new("playerEvent"))
        .qml(false)
        .build(&cpp, &moc);
}
