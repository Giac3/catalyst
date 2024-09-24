use std::{fs::File, io::{Read, Write}};

use clap::{error::ErrorKind, Command, Error};
use directories::BaseDirs;


pub fn read_data_file (command: &mut Command, file_name: String) -> Result<String, Error> {
    let base_dirs = match BaseDirs::new() {
        Some(dirs) => dirs,
        None => {
            return Err(command.error(ErrorKind::Io, "Unable to locate data directory."))
        }
    };

    let collections_file = base_dirs.data_dir().join(file_name);


    let mut file = match File::open(collections_file) {
        Ok(f) => f,
        Err(_) => {
            return Err(command.error(ErrorKind::Io, "Unable to open data file."));
        }
    };

    let mut buff = String::new();

    match file.read_to_string(&mut buff) {
        Ok(_) => {},
        Err(_) => {
            return Err(command.error(ErrorKind::Io, "Unable to read data file."));
        }
    };

    return Ok(buff)
}

pub fn write_data_file (command: &mut Command, file_name: String, buff: String) -> Result<(), Error> {
    let base_dirs = match BaseDirs::new() {
        Some(dirs) => dirs,
        None => {
            return Err(command.error(ErrorKind::Io, "Unable to locate data directory."))
        }
    };

    let collections_file = base_dirs.data_dir().join(file_name);

    println!("{}", collections_file.to_str().unwrap());

    let mut file = match File::create(collections_file) {
        Ok(f) => f,
        Err(_) => {
            return Err(command.error(ErrorKind::Io, "Unable to open data file."));
        }
    };

    match file.write_all(buff.as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            return Err(command.error(ErrorKind::Io, "Unable to save data."));
        }
    };

    return Ok(())
}