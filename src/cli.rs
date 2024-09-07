use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RustyGo🚀")]
#[command(version = "0.0.1")]
#[command(about = "A Rust based auto-reloader for Go! 🚀", long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable htmx support
    #[arg(short = 'x', long, global = true)]
    pub htmx: bool,
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Run the Go server
    ///, r is an alias for run
    #[command(alias = "r")]
    Run {
        /// The path to the Go files
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Initialize a config file
    // to be implemented later
    Init,
}
