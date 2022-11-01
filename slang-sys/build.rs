use std::{
    env,
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use curl::easy::Easy;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=wrapper.h");

    // Download Slang release
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let file_name = out_dir.join("slang.zip");
    let file = File::create(&file_name)?;
    let mut buf = BufWriter::new(file);
    let mut handle = Easy::new();
    handle.follow_location(true)?;
    handle.url("https://github.com/shader-slang/slang/releases/download/v0.24.38/slang-0.24.38-linux-x86_64.zip")?;
    handle.write_function(move |data| Ok(buf.write(data).unwrap()))?;
    handle.perform()?;

    // Unzip release
    let output_folder = out_dir.join("slang");
    eprintln!("{:?}", output_folder);
    if !output_folder.exists() {
        let file = File::open(&file_name)?;
        let mut zip = ZipArchive::new(file)?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let out_path = match file.enclosed_name() {
                Some(p) => output_folder.join(p),
                None => continue,
            };

            if file.name().ends_with('/') {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut out = File::create(&out_path)?;
                io::copy(&mut file, &mut out)?;
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
                }
            }
        }
    }

    // Generate bindings
    // Link slang
    let lib_path = output_folder.join("bin/linux-x64/release/");
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=slang");
    println!("cargo:rustc-link-lib=dylib=slang-glslang");

    let bindings = bindgen::Builder::default()
        .header(output_folder.join("slang.h").to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-v")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .enable_cxx_namespaces()
        .vtable_generation(true)
        // .allowlist_function("sp.*")
        // .allowlist_function("slang_.*")
        // .allowlist_type("I?Slang.*")
        // .allowlist_type("slang.*")
        // .allowlist_type("SLANG.*")
        .generate()?;

    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
