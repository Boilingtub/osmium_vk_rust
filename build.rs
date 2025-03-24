macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}
use std::{io::stdout, process::Command};
fn main() {
    p!("BUILD.rs -> Starting ...");
    let out_dir = &(format!("{}/../../..", std::env::var("OUT_DIR").unwrap()));
    let r = Command::new("cp")
        .args(["-r", "./resources/", out_dir])
        .output()
        .expect("Failed to run command `cp`");
    p!("Copy ./resources/ to {}", out_dir);
}
