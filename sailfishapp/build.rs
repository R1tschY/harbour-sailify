fn apply_pkg_config(build: &mut cpp_build::Config, config: &pkg_config::Library) {
    for include in &config.include_paths {
        build.include(include);
    }

    for (var, value) in &config.defines {
        match value {
            Some(ref value) => build.define(var, Some(value)),
            None => build.define(var, None),
        };
    }
}


fn main() {
    let sailfishapp = pkg_config::Config::new()
        .probe("sailfishapp")
        .unwrap();
    let qt5quick = pkg_config::Config::new()
        .atleast_version("5.6")
        .probe("Qt5Quick")
        .unwrap();

    let mut cpp = cpp_build::Config::new();
    apply_pkg_config(&mut cpp, &sailfishapp);
    apply_pkg_config(&mut cpp, &qt5quick);
    cpp.build("src/lib.rs");
}