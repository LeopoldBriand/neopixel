
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run(command: &mut Command) {
    let status = command.status().expect("failed to run AVR tool");
    assert!(status.success(), "AVR tool returned an error");
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let object = out_dir.join("neopixel_tx.o");
    let archive = out_dir.join("libneopixel_tx.a");

    let avr_gcc = env::var("AVR_GCC")
        .unwrap_or_else(|_| "avr-gcc".to_string());
    let avr_ar = env::var("AVR_AR")
        .unwrap_or_else(|_| "avr-ar".to_string());

    println!("cargo:rerun-if-changed=src/neopixel_tx.S");

    run(
        Command::new(avr_gcc)
            .arg("-mmcu=atmega328p")
            .arg("-c")
            .arg("-x")
            .arg("assembler-with-cpp")
            .arg("src/neopixel_tx.S")
            .arg("-o")
            .arg(&object),
    );

    run(
        Command::new(avr_ar)
            .arg("crs")
            .arg(&archive)
            .arg(&object),
    );

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=neopixel_tx");
}
