#Installer lite
A simple installation app creator for your Windows app.
Takes the bytes that make up your binary and store it inside the installer binary, then it writes the file down in a requested location.
how to use:
- First create a `./installer/installer.rs` file in your crates root directory.
- Add the binary to your `Cargo.toml` as such:
```toml
[package]
name = "demo-app"
version = "1.0.0"
edition = "2021"
# First add the app you want to package as a bin
[[bin]]
name = "demo_app"
path = "src/main.rs"

# Then add the installer as such, must be second so it always
# builds after your main one
[[bin]]
name = "demo_app_installer"
path = "installer/installer.rs"

# and ofcourse add the dependency
[dependencies]
installer_lite = "1.0.0"
```
- inside the `installer.rs`:
```rs
use installer_lite::Installer;
use std::{env, path::PathBuf};

/* Make sure your app is built first, then include it's bytes */
static EXECUTABLE: &'static [u8] = include_bytes!("../target/release/demo_app.exe");
fn main() {
    let app_name = env!("CARGO_PKG_NAME");

    let mut installer = Installer::new(
        EXECUTABLE,
        None, // Defaults to C:\Program Files (x86)
        app_name.to_string(),
    );
    /* Support for pre and post install custom functions */
    installer.add_pre_install_function(Box::from(|| {
        println!("STARTING INSTALLATION HEHE");
        let console_output = "STARTING INSTALLATION HEHE".to_owned();
        return console_output;
    }));
    /* Start the installer, maybe handle error cases */
    installer.start().expect("Installation somehow failed");
}
```