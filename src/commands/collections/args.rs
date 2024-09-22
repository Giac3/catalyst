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
        #[clap(hide(true))]
        check: Option<String>,

        name: Option<String>,
        id: Option<String>,
    },
}
