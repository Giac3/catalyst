
use clap::{error::ErrorKind, Command, Error, Subcommand};

use serde_json::{json, Value};
use json_to_table::{json_to_table, Orientation};
use url::Url;
use crate::{storage::endpoints::{read, write, Endpoint}, utils::id::{CIDStore, CIDTypes}};


#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Method {
   Get,
   Post,
   Patch,
   Put,
   Delete,
}

impl Method {
    fn get_str(&self) -> &str {
        match self {
            Method::Get => "get",
            Method::Post => "post",
            Method::Patch => "patch",
            Method::Put => "put",
            Method::Delete => "delete",
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum EndpointsCommands {
    List {
        id: Option<String>,
    },
    Create {
            #[arg(short, long, value_enum)]
            method: Method,
            #[arg(short, long)]
            url: String,
            #[arg(short, long)]
            params: Option<Vec<String>>,
            
            #[arg(long)]
            headers: Option<Vec<String>>,
            #[arg(short, long)]
            body: Option<Value>,
    },
    Delete {
        #[clap(hide(true))]
        check: Option<String>,

        name: Option<String>,
        id: Option<String>,
    },
}

use tabled::settings::Style;

pub fn handle_endpoints_commands (cmd: EndpointsCommands) -> Result<(), Error> {
    

    match cmd {
        EndpointsCommands::List { id } => {
            let mut command = Command::new("Endpoints");
            let endpoints = read(&mut command)?;

            let display_endpoints = match id {
                Some(id) => {
                    let endpoints = endpoints.into_iter().filter(|item| item.id == id).collect();
                    endpoints
                },
                None => endpoints
            };
            
            let table = if display_endpoints.len() >0 {
                json!(display_endpoints)
            } else {
                json!("No Endpoints")
            };
            let t = json_to_table(&table)
                .array_orientation(Orientation::Column).object_orientation(Orientation::Row).with(Style::modern_rounded())
                .to_string();
            println!("{}", t);
            Ok(())
        }
        EndpointsCommands::Create { method, url, params, headers, body }=> {
            let mut command = Command::new("\n\ncatalyst endpoints create -m get -u https://google.com");
            let mut endpoints = read(&mut command)?;

            let mut cid_store =  CIDStore::new();     

            let cids: Vec<String> = endpoints.iter().map(|endpoint|  endpoint.id.clone()).collect();
            cid_store.load_cids(cids);

            let cid = cid_store.new_cid(CIDTypes::Endpoint);

            let url = match Url::parse(&url) {
                Ok(url) => url,
                Err(_) => {
                    return Err(command.error(ErrorKind::InvalidValue, "Invalid url"));
                }
            };

            let new_endpoint = Endpoint {
                id: cid,
                method: method.get_str().to_string(),
                url: url.to_string(),
                headers,
                params,
                body
            };

            endpoints.push(new_endpoint);

            write(&mut command, endpoints)?;
            
            println!("Endpoint Created");

            Ok(())
        }
        _ => {
            Ok(())
        }
    }
}
