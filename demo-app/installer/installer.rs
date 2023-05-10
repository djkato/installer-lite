#![windows_subsystem = "windows"]
use installer_lite::Installer;
use std::{env, path::PathBuf};

static EXECUTABLE: &'static [u8] = include_bytes!("../target/release/demo_app.exe");
fn main() {
    let app_name = env!("CARGO_PKG_NAME");
    let executable_name = env!("CARGO_BIN_NAME");
    let dir = PathBuf::from("demo_app.exe");
    println!(
        "
    App name: {app_name}\n
    executable name: {executable_name}\n
    Path to executable: {}",
        dir.display()
    );
    let mut installer = Installer::new(
        EXECUTABLE,
        Some(r"C:\Users\djkato\Code PF\installer-lite\demo-app\install location".to_string()),
        app_name.to_string(),
    );

    installer.add_pre_install_function(Box::from(|| {
        println!("STARTING INSTALLATION HEHE");
        let console_output = "STARTING INSTALLATION HEHE".to_owned();
        return console_output;
    }));
    let result = installer.start();
    println!("result: {:?}", result);
}
