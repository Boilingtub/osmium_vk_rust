macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}
use std::process::Command;
fn main() {
    p!("BUILD.rs -> Starting ...");
    let out_dir = &(format!("{}/../../..", std::env::var("OUT_DIR").unwrap()));
    let _r = Command::new("cp")
        .args(["-r", "./resources/", out_dir])
        .output()
        .expect("Failed to run command `cp`");
    p!("Copy ./resources/ to {}", out_dir);
}
