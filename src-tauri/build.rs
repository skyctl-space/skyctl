use std::{
    env,
    fs,
    path::{Path, PathBuf}
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os.as_str() == "macos" {
        // Probe CFITSIO via pkg-config
        let lib = match pkg_config::Config::new()
            .probe("cfitsio")
        {
            Ok(lib) => lib,
            Err(_) => {
                println!("cargo:warning=Failed to find CFITSIO via pkg-config. Falling back to platform-specific paths.");
                return fallback_copy_libraries();
            }
        };

        // Output location inside the Tauri project
        let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bins");
        fs::create_dir_all(&out_dir).expect("Could not create bins/ directory");

        for lib_path in lib.link_paths {
            // println!("cargo:warning=Library path: {:?}", lib_path);

            // Try to find and copy the library
            if let Some(lib_file) = find_cfitsio_lib(&lib_path) {
                let filename = lib_file.file_name().unwrap();
                let dst = out_dir.join(filename);
                fs::copy(&lib_file, &dst).expect("Failed to copy CFITSIO binary");
                println!("cargo:info=Copied {:?} to {:?}", lib_file, dst);
            }
        }
    }

    tauri_build::build()
}

fn find_cfitsio_lib(dir: &Path) -> Option<PathBuf> {
    let candidates = [
        "libcfitsio.dylib", // macOS
    ];

    for name in candidates {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn fallback_copy_libraries() {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bins");
    fs::create_dir_all(&out_dir).expect("Could not create bins/ directory");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os.as_str() == "macos" {
        // Manually copy the macOS .dylib
        let src = Path::new("/opt/homebrew/lib/libcfitsio.10.dylib");
        if src.exists() {
            let dst = out_dir.join("libcfitsio.10.dylib");
            fs::copy(src, dst).expect("Failed to copy CFITSIO dylib");
            println!("cargo:warning=Copied macOS CFITSIO dylib");
        }
    }
}
