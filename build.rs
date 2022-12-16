use std::{env, fs, path::Path};

fn main() {
    let out = env::var_os("OUT_DIR").unwrap();

    let options = grass::Options::default();
    let compiled_css =
        grass::from_path("assets/scss/style.scss", &options).expect("Couldn't compile sass");
    let dest_file = Path::new(&out).join("style.css");
    fs::write(dest_file, compiled_css).expect("Couldn't write compiled css");

    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=db");
    println!("cargo:rerun-if-changed=build.rs");
}
