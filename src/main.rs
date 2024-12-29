use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod fs_helper;

#[derive(Subcommand)]
enum Commands {
    /// adds a new alias
    Add {
        /// the alias for the app
        #[arg(short, long)]
        name: String,

        /// the abs path to the app
        #[arg(short, long)]
        path: PathBuf,
    },
    /// crawl from a dir to add aliases
    Crawl {
        /// the base dir to start crawl from
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,

        /// rather or not to crawl recursivly
        #[arg(short, long, default_value_t = true)]
        recursive: bool,
    },
}

#[derive(Parser)]
#[command(name = "Shell-Alias-Maker")]
#[command(version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, path } => {
            println!("got name: {}, path: {}", name, path.display())
        }
        Commands::Crawl { dir, recursive } => {
            println!("got dir: {}, recursive: {}", dir.display(), recursive)
        }
    }
}
