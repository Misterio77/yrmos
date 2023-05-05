use std::{env, fs, path::Path};

fn main() {
    let out = env::var_os("OUT_DIR").unwrap();

    let options = grass::Options::default();
    let scss_path = "assets/scss/style.scss";
    let dest_css_file = Path::new(&out).join("style.css");
    let dest_hash_file = Path::new(&out).join("style.hash");

    let compiled_css = grass::from_path(scss_path, &options).expect("Couldn't compile sass");
    let css_hash = blake3::hash(compiled_css.as_bytes()).to_hex();

    fs::write(dest_hash_file, css_hash.as_str()).expect("Couldn't write css hash");
    fs::write(dest_css_file, compiled_css).expect("Couldn't write compiled css");

    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=db");
    println!("cargo:rerun-if-changed=build.rs");
}
