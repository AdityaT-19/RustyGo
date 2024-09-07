use clap::Parser;
mod cli;
mod watch;
use cli::{Cli, Commands};
use watch::watch;
fn main() {
    let cli = Cli::parse();
    if cli.htmx {
        println!("Htmx support enabled");
    } else {
        println!("Htmx support disabled");
    }
    match cli.command {
        Commands::Run { path } => {
            println!("Watching the Go Files");
            watch(&path, cli.htmx);
        }
        Commands::Init => {
            println!("Initializing the config file");
        }
    }
}
