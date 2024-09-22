use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum CollectionsCommands {
    List {
        collection: Option<String>,
    },
    Create {
        name: String,
    },
    Delete {
        name: String,
        id: String,
    },
}
