extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2rc1")
        .probe("speexdsp")
        .unwrap();
    println!("cargo:rustc-link-lib=speexdsp");
}
