use clap::{error::ErrorKind, Command, Error};


use serde::{Deserialize, Serialize};

use super::utils::{read_data_file, write_data_file};


#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    id: String,
    name: String,
    endpoints: Vec<String>
}

pub fn read (command: &mut Command) -> Result<Vec<Collection>, Error> {
   let buff = read_data_file(command, String::from("collections.json"))?;
   let collections: Vec<Collection> = match serde_json::from_str(&buff) {
    Ok(collections) => {collections},
    Err(_) => {
        return Ok(Vec::new())
    }
    }; 
    Ok(collections)
}


pub fn write (command: &mut Command, collections: Vec<Collection>) -> Result<Vec<Collection>, Error> {

    let buff = match serde_json::to_string(&collections) {
        Ok(buff) => buff,
        Err(_) => return Err(command.error(ErrorKind::Io, "Unable to save collections.")),
    };

    write_data_file(command, String::from("collections.json"), buff)?;
    Ok(collections)
}