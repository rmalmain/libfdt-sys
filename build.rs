use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

const LIBFDT_PATH: &str = "libfdt";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path_str = out_path.as_os_str().to_str().unwrap();

    let mut options = HashMap::new();
    options.insert("tools", "false");
    options.insert("yaml", "disabled");
    options.insert("valgrind", "disabled");
    options.insert("python", "disabled");
    options.insert("tests", "false");

    let config = meson_next::Config::new().options(options);

    if cfg!(feature = "static") && cfg!(feature = "shared") {
        panic!("Both static and dynamic features are set.");
    }

    let link_kind = if cfg!(feature = "shared") {
        "dylib"
    } else {
        // fall back to static linking.
        "static"
    };

    println!(
        "cargo:rustc-link-search={}",
        out_path.join(LIBFDT_PATH).display()
    );
    println!("cargo:rustc-link-lib={link_kind}=fdt");
    meson_next::build(LIBFDT_PATH, out_path_str, config);

    println!("cargo:rerun-if-changed=libfdt");

    let bindings = bindgen::Builder::default()
        .header("libfdt/libfdt/libfdt.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-Ilibfdt/libfdt")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
