use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum RunCommands {
    History,
    Collection {
        #[clap(hide(true))]
        check: Option<String>,
        
        id: Option<String>,
        name: Option<String>
    },
    Endpoint {
        id: String,
    },
}
