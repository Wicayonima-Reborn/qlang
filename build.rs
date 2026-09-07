use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Only re-run this build script if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = manifest_dir.join("tcc");
    let tcc_exe = out_dir.join("tcc.exe");

    if !tcc_exe.exists() {
        println!("cargo:warning=[QLC BUILD] TinyCC executable not found. Downloading cnlohr installer...");

        let installer_url = "https://github.com/cnlohr/tinycc-win64-installer/releases/download/v0_0.9.27/tcc-0.9.27-win64-installer.exe";
        let temp_installer = manifest_dir.join("temp_installer.exe");

        let client = reqwest::blocking::Client::builder()
            .user_agent("QLang-Compiler-BuildScript")
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .expect("[QLC BUILD ERROR] Failed to create HTTP client");

        let response = client
            .get(installer_url)
            .send()
            .expect("[QLC BUILD ERROR] Failed to connect to GitHub releases");

        if !response.status().is_success() {
            panic!(
                "[QLC BUILD ERROR] HTTP Request failed with status: {}",
                response.status()
            );
        }

        let bytes = response
            .bytes()
            .expect("[QLC BUILD ERROR] Failed to fetch installer bytes");

        fs::write(&temp_installer, bytes)
            .expect("[QLC BUILD ERROR] Failed to write temporary installer file");

        println!("cargo:warning=[QLC BUILD] Extracting installer payload into tcc/...");

        fs::create_dir_all(&out_dir).unwrap();

        let status = Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Start-Process -FilePath '{}' -ArgumentList '/S', '/D={}' -Verb RunAs -Wait",
                temp_installer.display(),
                out_dir.display()
            ))
            .status();

        let _ = fs::remove_file(&temp_installer);

        if status.is_err() || !status.unwrap().success() {
            panic!("[QLC BUILD ERROR] Installation failed or UAC was declined");
        }

        println!("cargo:warning=[QLC BUILD] TinyCC successfully installed into tcc/!");
    }
}