#![cfg(test)]


use std::{
    fs,
    io::{
        ErrorKind,
        self
    },
    path::Path
};


pub fn cleanup() {
    if let Err(_) = cleanup_dir("./servers/") {}
    if let Err(_) = cleanup_dir("./config/") {}
    if let Err(_) = cleanup_dir("./logs/") {}
}
pub fn cleanup_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            cleanup_dir(&path)?;
            if let Err(erro) = fs::remove_dir(&path) {
                match erro.kind() {
                    ErrorKind::NotFound => {}
                    _ => {
                        return Err(erro);
                    }
                }
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    fs::remove_dir(path)?;
    Ok(())
}