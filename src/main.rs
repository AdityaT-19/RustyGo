use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod watch;
use watch::watch;
fn main() {
    let cli = Cli::parse();
    if cli.htmx {
        println!("Htmx support enabled");
    } else {
        println!("Htmx support disabled");
    }
    //
    match cli.command {
        Commands::Run => {
            println!("Running the Go server");
            watch();
        }
        Commands::Init => {
            println!("Initializing the config file");
        }
    }
}
