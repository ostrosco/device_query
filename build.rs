extern crate pkg_config;

#[cfg(target_os = "windows")]
fn main() {}

#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::path::Path;

#[cfg(target_os = "linux")]
fn main() {
    let mut config = String::new();
    let libdir = match pkg_config::get_variable("x11", "libdir") {
        Ok(libdir) => format!("Some(\"{}\")", libdir),
        Err(_) => "None".to_string(),
    };
    config.push_str(&format!(
        "pub const {}: Option<&'static str> = {};\n",
        "x11", libdir
    ));

    let config =
        format!("pub mod config {{ pub mod libdir {{\n{}}}\n}}", config);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(&config.into_bytes()).unwrap();

    let target = env::var("TARGET").unwrap();
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dl");
    } else if target.contains("freebsd") || target.contains("dragonfly") {
        println!("cargo:rustc-link-lib=c");
    }
}
