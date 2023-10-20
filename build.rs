extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={:?}", out_path);

    // build a static lib
    cc::Build::new()
        .file("src/upstream/mailbox.c")
        .file("src/upstream/ws2811.c")
        .file("src/upstream/pwm.c")
        .file("src/upstream/pcm.c")
        .file("src/upstream/dma.c")
        .file("src/upstream/rpihw.c")
        // create a static lib to make cross-compiling
        // and uploading easier.
        .shared_flag(false)
        .compile("libws2811.a");

    // link to the created static lib
    println!("cargo:rustc-link-lib=static=ws2811");
}
