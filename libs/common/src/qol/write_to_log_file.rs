//! This module provides the [`write_to_log_file`] method which writes a given bytes string to a file at `./logs/<destination>.txt`.


use std::{
    fs::{
        File,
        self
    },
    io::{
        ErrorKind,
        Write
    }
};


/// This function writes a given bytes string to a log file at `./logs/<destination>.txt`.
pub fn write_to_log_file(log: &[u8], destination: &str) {    
    let destination = &format!("logs/{destination}.txt");

    match File::options().append(true).create(true).open(destination) {
        Ok(mut log_file) => {
            if let Err(erro) = log_file.write_all(log) {
                panic!("An error occurred while writing a log message to the file {destination}. Error: {erro}")
            }
        }
        Err(erro) if ErrorKind::NotFound == erro.kind() => {
            fs::create_dir("logs").expect("The error ErrorKind::NotFound only gets returned if the logs dir is missing.");

            let mut log_file;
            match File::options().append(true).create(true).open(destination) {
                Ok(file) => {
                    log_file = file;
                }
                Err(erro) => {
                    panic!("Could not write to the log file {destination}. Error: {erro}")
                }
            }

            if let Err(erro) = log_file.write_all(log) {
                panic!("An error occurred while writing a log message to the file {destination}. Error: {erro}")
            }
        }
        Err(erro) => {
            panic!("Could not write to the log file {destination}. Error: {erro}")
        }
    }
}