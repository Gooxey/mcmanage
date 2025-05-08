use std::{
    error::Error,
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    // add an icon
    if cfg!(target_os = "windows") {
        println!("Build Script | Adding an icon to the executable...");

        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile()?;
    }

    println!("Build Script | Compiling the frontend application...");

    // Change to the frontend application directory
    let yew_app_dir = "bin/frontend";
    if let Err(e) = std::env::set_current_dir(yew_app_dir) {
        println!("Build Script | Failed to change directory to {}: {}", yew_app_dir, e);
        std::process::exit(1);
    }

    // Run trunk build --release
    let trunk_build = Command::new("trunk")
        .args(["build", "--release"])
        .status()
        .expect("failed to execute trunk build --release");
    if !trunk_build.success() {
        println!("Build Script | Failed to build the frontend application!");
        panic!();
    }

    println!("Build Script | Done!");

    Ok(())
}
