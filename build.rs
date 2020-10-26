use qobject_compiler::moc::MocConfig;
use qobject_compiler::qobject::TypeRefTrait;
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
            &QObjectProp::new(&TypeRef::qstring(), "username")
                .read("username")
                .write("setUsername"),
        )
        .property(
            &QObjectProp::new(&TypeRef::qstring(), "password")
                .read("password")
                .write("setPassword"),
        )
        .property(
            &QObjectProp::new(&TypeRef::primitive::<bool>(), "active")
                .read("isActive")
                .notify("activeChanged"),
        )
        // username
        .method(&QObjectMethod::new("username").const_().ret::<QString>())
        .method(&QObjectMethod::new("setUsername").arg::<&QString>("value"))
        // password
        .method(&QObjectMethod::new("password").const_().ret::<QString>())
        .method(&QObjectMethod::new("setPassword").arg::<&QString>("value"))
        // active
        .method(&QObjectMethod::new("isActive").const_().ret::<bool>())
        .signal(&QObjectSignal::new("activeChanged").arg("value", &TypeRef::primitive::<bool>()))
        // slots
        .slot(&QObjectMethod::new("start"))
        .slot(&QObjectMethod::new("stop"))
        // private slots
        .slot(&QObjectMethod::new("_onPlayerEvent").arg::<&QString>("event"))
        .build(&cpp, &moc);

    QObjectBuild::new("LibrespotGateway")
        .inherit(TypeRef::qobject())
        .signal(
            &QObjectSignal::new("playerEvent").arg("event", &TypeRef::qstring().with_const_ref()),
        )
        .build(&cpp, &moc);
}
