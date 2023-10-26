use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=examples/res/*");

    let cargo_path = env::var("CARGO_MANIFEST_DIR").unwrap().replace("\\", "/");
    let profile_path = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!(
        "{}/target/{}/{}",
        cargo_path, profile_path, "examples/examples"
    ));

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("examples/res/");

    if !out.exists() {
        fs::create_dir(&out).unwrap();
    }

    println!("cargo:warning=COPYING FILES TO TARGET FOLDER {:?}", out);
    copy_items(&paths_to_copy, out, &copy_options)?;

    Ok(())
}
