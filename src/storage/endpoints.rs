use std::collections::HashMap;

use clap::{error::ErrorKind, Command, Error};


use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::utils::{read_data_file, write_data_file};


#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub id: String,
    pub method: String,
    pub url: String,
    pub params: Option<Vec<String>>,
    pub body: Option<Value>,
    pub headers: Option<Vec<String>>,
}

pub fn read (command: &mut Command) -> Result<(Vec<Endpoint>), Error> {
   let buff = read_data_file(command, String::from("endpoints.json"))?;
   let endpoints: Vec<Endpoint> = match serde_json::from_str(&buff) {
    Ok(endpoints) => {endpoints},
    Err(_) => {
        return Ok(Vec::new())
    }
}; 
    Ok(endpoints)
}

pub fn write (command: &mut Command, endpoints: Vec<Endpoint>) -> Result<Vec<Endpoint>, Error> {

    let buff = match serde_json::to_string(&endpoints) {
        Ok(buff) => buff,
        Err(_) => return Err(command.error(ErrorKind::Io, "Unable to save endpoints.")),
    };

    write_data_file(command, String::from("endpoints.json"), buff)?;
    Ok(endpoints)
}