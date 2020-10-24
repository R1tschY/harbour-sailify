use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, TypeRef};
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
        .method(&QObjectMethod::new("username").const_().ret::<QString>())
        .method(&QObjectMethod::new("setUsername").arg::<&QString>("value"))
        .build(&cpp, &moc);
}
