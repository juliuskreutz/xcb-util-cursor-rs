use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=xcb-cursor");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("wrapper.h")
        .allowlist_function("xcb_cursor_.*")
        .allowlist_type("xcb_cursor_.*")
        .allowlist_var("xcb_cursor_.*")
        .blocklist_type("xcb_connection_t")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
