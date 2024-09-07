use clap::Parser;
pub mod cli;
use cli::{Cli, Commands};
fn main() {
    let cli = Cli::parse();
    if cli.htmx {
        println!("Htmx support enabled");
    } else {
        println!("Htmx support disabled");
    }

    match cli.command {
        Commands::Run => {
            println!("Running the Go server");
        }
        Commands::Init => {
            println!("Initializing the config file");
        }
    }
}
