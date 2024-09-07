use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RustyGo🚀")]
#[command(version = "0.0.1")]
#[command(about = "A Rust based auto-reloader for Go! 🚀", long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable htmx support
    #[arg(short = 'x', long, global = true)]
    htmx: bool,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Run the Go server
    ///, r is an alias for run
    #[command(alias = "r")]
    Run,
    /// Initialize a config file
    // to be implemented later
    Init,
}
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
