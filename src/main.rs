use catalyst::{commands::{collections::args::CollectionsCommands, endpoints::args::{handle_endpoints_commands, EndpointsCommands}, run::args::{handle_run_commands, RunCommands}}, styles::get_styles};
use clap::{Error, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles=get_styles())]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Run {
        #[command(subcommand)]
        cmd: RunCommands
    },
    Collections {
        #[command(subcommand)]
        cmd: CollectionsCommands,
    },
    Endpoints {
        #[command(subcommand)]
        cmd: EndpointsCommands,
    }
}

fn main()  {
    let args = Cli::parse();
    
    match handle_command(args.cmd) {
        Ok(_) => {
            return
        },
        Err(e) => {
            print!("\n");
            e.exit();
        }
    }
    
    
}

fn handle_command (cmd: Commands) -> Result<(), Error> {


    match cmd {
        Commands::Run { cmd } => {
            handle_run_commands(cmd)?;
            Ok(())
        },
        Commands::Collections { cmd } => {
            Ok(())
        },
        Commands::Endpoints { cmd } => {
            handle_endpoints_commands(cmd)?;
            Ok(())
        },
        
    }
}