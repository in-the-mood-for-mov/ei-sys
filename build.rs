use std::{env, path, process};

#[cfg(not(windows))]
const NAME: &'static str = "ei";

#[cfg(windows)]
const NAME: &'static str = "ei_md";

fn resolve_library_search_path_from_erlang() -> Option<path::PathBuf> {
  let output = process::Command::new("erl")
    .arg("-noinput")
    .arg("-eval")
    .arg(r#"io:format("~s", [code:root_dir()]), init:stop()."#)
    .output()
    .unwrap();
  if !output.status.success() {
    return None;
  }

  let code_root_dir = path::PathBuf::from(String::from_utf8(output.stdout).unwrap());
  Some(code_root_dir.join("usr/lib"))
}

fn resolve_library_search_path_from_env() -> Option<path::PathBuf> {
  env::var_os("EI_LINK_SEARCH").map(path::PathBuf::from)
}

fn resolve_library_search_path() -> Option<path::PathBuf> {
  resolve_library_search_path_from_env().or_else(resolve_library_search_path_from_erlang)
}

fn main() {
  let library_search_path = resolve_library_search_path().expect(
    "You need to either have Erlang in your path or set the environment variable \
     EI_LINK_SEARCH to compile this crate",
  );
  println!("cargo:rustc-link-lib=static={}", NAME);
  println!(
    "cargo:rustc-link-search=native={}",
    library_search_path.as_os_str().to_str().unwrap()
  );
}
