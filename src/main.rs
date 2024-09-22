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
    Create {
        #[arg(short, long, default_value = "test")]
        key: String,
        #[arg(short, long, default_value = "test")]
        value: String,
        
    }
}

fn main() {
    let args = Args::parse();
  
    println!("{:?}", args);
}