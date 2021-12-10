use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let target = env::var("TARGET").unwrap();

    let v2 = target == "thumbv7em-none-eabihf";

    if v2 {
        println!("cargo:rustc-cfg=v2");
    }

    let (flash_size, mem_size) = if v2 {
        ("512K", "128K")
    } else {
        ("256K", "16K")
    };

    let mut file = File::create(out.join("memory.x")).unwrap();
    write!(
        file,
        "MEMORY
{{
    FLASH : ORIGIN = 0x00000000, LENGTH = {}
    RAM : ORIGIN = 0x20000000, LENGTH = {}
}}
",
        flash_size, mem_size
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    if target != "thumbv6m-none-eabi" && target != "thumbv7em-none-eabihf" {
        println!("cargo:warning={} is not a valid target for the micro:bit. The 'thumbv6m-none-eabi' target should be used for the v1, and the 'thumbv7em-none-eabihf' target should be used for the v2.", target);
    }
}
