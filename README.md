# opensplice-sys

This library is a rust binding to the DCPS C API of Adlink's Vortex Opensplice community edition version **6.9.1.181018**.  
To be able to use this binding you need Vortex Opensplice installed on your machine. [Vortex Opensplice can be downloaded from here.](https://github.com/ADLINK-IST/opensplice)  

The rust bindings were generated using bindgen.

### **Current state of opensplice-sys**  
If you encounter any problems, please create an issue and let me know!

## Building opensplice-sys  
To be able to build the rust bindings you need to have the Vortex Opensplice Community installed on your machine of choice.  
Before building make sure you set the required environment variables by sourcing `release.com` in the Opensplice installation folder on Linux.  

The build script will use the `OSPL_HOME` environment variable to find the required headers and libraries.  
If it cannot find the environment variable the library will not be build.

## Generating bindings using bindgen
If you want to generate the bindings yourself replace `build.rs` with the following piece of code.
**Note:** You will need to add `bindgen` to the `[build-dependencies]` in `Cargo.toml`. 

```rust
extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() {
    let key = "OSPL_HOME";
    let ospl_home = match env::var(key) {
        Ok(val) => val,
        Err(_e) => panic!("OSPL_HOME variable was not set.")
    };

    println!(r"cargo:rustc-link-search={}/lib/", ospl_home);
    println!(r"cargo:include={}/include/dcps/C/SAC", ospl_home);
    println!(r"cargo:rustc-link-lib=dcpssac");

    let bindings = bindgen::Builder::default()
    .header(format!("{}/include/dcps/C/SAC/dds_dcps.h", ospl_home))
    .clang_arg(format!("-I{}/include/dcps/C/SAC/", ospl_home))
    .whitelist_type("DDS_.*")
    .whitelist_var("DDS_.*")
    .whitelist_function("DDS_.*")
    .layout_tests(false)
    .rustfmt_bindings(true)
    .generate()
    .expect("Unable to generate bindings for OpenSplice DDS");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write OpenSplice DDS bindings to bindings.rs!");
}
```

## Contributing

Help is always appreciated! Submit a PR if you have any improvements for the rust binding to Vortex Opensplice.  


