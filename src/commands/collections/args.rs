use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum CollectionsCommands {
    List {
        #[arg(short, long)]
        collection: Option<String>,
    },
    Create {
        #[arg(short, long)]
        name: String,
    },
    Delete {
        #[clap(hide(true))]
        check: Option<String>,

        name: Option<String>,
        id: Option<String>,
    },
}
