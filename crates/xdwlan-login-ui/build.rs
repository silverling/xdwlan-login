extern crate embed_resource;

use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../packages/xdwlan-login");

    embed_resource::compile("app.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();

    // Get the target directory and profile
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    let profile = std::env::var("PROFILE").unwrap();

    for (key, value) in std::env::vars() {
        eprintln!("{}={}", key, value);
    }

    // Define paths
    let example_config_path = Path::new("../../config.yaml");
    let target_config_path = target_dir.join("config.yaml");
    let server_exe_path =
        Path::new("../../packages/xdwlan-login/build/xdwlan-login-windows-server.exe"); // relative to the build.rs
    let target_exe_path = target_dir.join("xdwlan-login-server.exe");

    // Delete the existing server executable
    if server_exe_path.exists() {
        fs::remove_file(&server_exe_path).unwrap();
    }

    // Always build bun program
    println!("cargo:info=Building login server");
    let output = Command::new("bun")
        .args(&["run", "build:windows", &profile])
        .current_dir("../../packages/xdwlan-login")
        .output()
        .expect("Failed to execute bun command");

    if !output.status.success() {
        panic!(
            "Failed to build server: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    println!("cargo:info=Build login server completed");

    // Copy the executable to target directory
    if server_exe_path.exists() {
        fs::copy(&server_exe_path, &target_exe_path).expect("Failed to copy server executable");
        println!("cargo:info=Copied server executable to target directory");
    } else {
        panic!("Server executable still not found after build attempt");
    }

    // Copy an example config file to the target directory if it doesn't exist
    if !target_config_path.exists() {
        fs::copy(&example_config_path, &target_config_path).expect("Failed to copy example config");
        println!("cargo:info=Copied example config to target directory");
    }
}
