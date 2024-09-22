use catalyst::commands::collections::args::CollectionsCommands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Run,
    Collections {
        #[command(subcommand)]
        cmd: CollectionsCommands,   
    }
}

fn main() {
    let args = Args::parse();
    
    println!("{:?}", args);
}