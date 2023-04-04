use std::{
    error::Error,
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    // add an icon
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile()?;
    }

    // Change to the frontend application directory
    let yew_app_dir = "frontend";
    if let Err(e) = std::env::set_current_dir(yew_app_dir) {
        eprintln!("Failed to change directory to {}: {}", yew_app_dir, e);
        std::process::exit(1);
    }

    // Run trunk build --release
    let trunk_build = Command::new("trunk")
        .args(["build", "--release"])
        .status()
        .expect("failed to execute trunk build --release");
    assert!(trunk_build.success());

    Ok(())
}
