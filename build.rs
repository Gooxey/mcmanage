use std::{
    process::Command,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Change to the frontend application directory
    let yew_app_dir = "frontend";
    if let Err(e) = std::env::set_current_dir(yew_app_dir) {
        eprintln!("Failed to change directory to {}: {}", yew_app_dir, e);
        std::process::exit(1);
    }
    
    // Run trunk build --release
    let trunk_build = Command::new("trunk")     // FIXME: At this point the compiler does not proceed if you compile with `cargo build --release`
        .args(["build", "--release"])
        .status()
        .expect("failed to execute trunk build --release");
    assert!(trunk_build.success());

    Ok(())
}