use std::{path, process};

#[cfg(not(windows))]
const NAME: &'static str = "ei";

#[cfg(windows)]
const NAME: &'static str = "ei_md";

fn main() {
    let output = process::Command::new("erl")
        .arg("-noinput")
        .arg("-eval")
        .arg(r#"io:format("~s", [code:root_dir()]), init:stop()."#)
        .output()
        .unwrap();
    assert!(output.status.success());
    let code_root_dir = path::PathBuf::from(String::from_utf8(output.stdout).unwrap());
    let library_search_path = code_root_dir.join("usr/lib");

    println!("cargo:rustc-link-lib=static={}", NAME);
    println!(
        "cargo:rustc-link-search=native={}",
        library_search_path.as_os_str().to_str().unwrap()
    );
}
