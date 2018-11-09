extern crate bindgen;

use std::env;

fn main() {
    let key = "OSPL_HOME";
    let ospl_home = match env::var(key) {
        Ok(val) => val,
        Err(_e) => panic!("OSPL_HOME variable was not set!"),
    };

    println!(r"cargo:rustc-link-search={}/lib/", ospl_home);
    println!(r"cargo:rustc-link-lib=dcpssac");
}
