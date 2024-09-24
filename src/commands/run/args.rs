use clap::{error::ErrorKind, Command, Error, Subcommand};

use crate::utils::id::validate_cid;

#[derive(Subcommand, Debug, Clone)]
pub enum RunCommands {
    History,
    Collection {
        id_or_name: Option<String>,
    },
    Endpoint {
        id: String,
    },
}



pub fn handle_run_commands (cmd: RunCommands) -> Result<(), Error> {


    match cmd {
        RunCommands::Collection { id_or_name } => {

        let mut command = Command::new("\n\ncatalyst run collection c:313avc1\ncatalyst run collection my_collection");

        let id_or_name = match id_or_name {
            Some(value) => value,
            None => {
                return Err(command.error(ErrorKind::InvalidValue, "Please provide either collection ID or a collection name to start a performance run."))
            }
        };

        let valid_cid = validate_cid(&id_or_name);

        if valid_cid {
            
            return Ok(());
        }
        Ok(())
        }
        _ => {
            Ok(())
        }
    }
}
