use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    giputils::build::git_submodule_update()?;
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| "Environmental variable `OUT_DIR` not defined.".to_string())?;

    let minisat_path = PathBuf::from("./bindings");
    println!("cargo:rerun-if-changed=./bindings");
    println!("cargo:rerun-if-changed=./minisat");
    let mut cfg = Config::new(minisat_path);

    cfg.build();

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(out_dir).join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=bindings");
    println!("cargo:rustc-link-lib=static=minisat");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
    Ok(())
}
