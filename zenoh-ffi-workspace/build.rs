use std::path::PathBuf;

fn main() {
    // If CARGO_LOCK is set, use it as the cargo lock path
    if let Ok(cargo_lock_path) = std::env::var("CARGO_LOCK") {
        // Check if cargo_lock_path is absolute
        if !PathBuf::from(&cargo_lock_path).is_absolute() {
            panic!("\nCARGO_LOCK={} must be an absolute path\n", cargo_lock_path);
        }
        println!("cargo:rustc-env=CARGO_LOCK_PATH={cargo_lock_path}");
        return;
    }
    // Otherwise, check if the crate is in the workspace
    let cargo_home = PathBuf::from(std::env::var("CARGO_HOME").unwrap());
    let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    if cargo_manifest_dir.starts_with(cargo_home) {
        panic!("\n\
        The crate `zenoh-ffi-workspace` located at {}\n\
        is being used as a Cargo dependency.\n\
        Since it is not located inside the workspace, it cannot determine the path to the workspace's `Cargo.lock`.\n\
        \n\
        Two solutions are available:\n\
        1. Explicitly pass the absolute path to Cargo.lock using the CARGO_LOCK environment variable:\n\
        \n\
           CARGO_LOCK=$PWD/Cargo.lock cargo build\n\
        \n\
        2. Add a clone of the `zenoh-ffi-workspace` project to your workspace\n\
           and add a \"patch\" section to your `Cargo.toml` to make other crates use this local version.\n\
           With this setup, the `cargo build` command will work without additional configuration:\n\
           \n\
           git clone https://github.com/eclipse-zenoh/zenoh-ffi-workspace\n\
           rm -rf zenoh-ffi-workspace/.git\n\
           \n\
           [workspace]\n\
           members = [\n\
               \"zenoh-ffi-workspace\",\n\
               # ... other members\n\
           ]\n\
           \n\
           [patch.crates-io]\n\
           \"zenoh-ffi-workspace\" = {{ path = \"zenoh-ffi-workspace\" }}\n\
           \n\
           [patch.'https://github.com/eclipse-zenoh/zenoh-ffi-workspace']\n\
           \"zenoh-ffi-workspace\" = {{ path = \"zenoh-ffi-workspace\" }}\n\
           \n\
        ", cargo_manifest_dir.display());
    }
    let workspace_root = project_root::get_project_root().unwrap();
    let cargo_lock_path = workspace_root.join("Cargo.lock");
    println!("cargo:rustc-env=CARGO_LOCK_PATH={}", cargo_lock_path.display());
}
