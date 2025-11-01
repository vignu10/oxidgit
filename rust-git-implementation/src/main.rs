//! oxid - A Git implementation in Rust
//!
//! Command-line interface for oxid

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "oxid")]
#[command(about = "A Git implementation in Rust for learning purposes", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        /// Path where to initialize the repository
        #[arg(default_value = ".")]
        path: String,
    },

    // Uncomment as you implement each command

    // /// Compute object ID and optionally create a blob
    // HashObject {
    //     /// Write the object to the database
    //     #[arg(short = 'w')]
    //     write: bool,
    //
    //     /// File to hash
    //     file: String,
    // },
    //
    // /// Provide content for repository objects
    // CatFile {
    //     /// Show object type
    //     #[arg(short = 't')]
    //     show_type: bool,
    //
    //     /// Pretty-print object content
    //     #[arg(short = 'p')]
    //     pretty_print: bool,
    //
    //     /// Object hash to display
    //     object: String,
    // },
    //
    // /// Create a tree object from the current index
    // WriteTree,
    //
    // /// Create a new commit object
    // Commit {
    //     /// Commit message
    //     #[arg(short = 'm')]
    //     message: String,
    // },
    //
    // /// Add file contents to the index
    // Add {
    //     /// Files to add
    //     files: Vec<String>,
    // },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            oxid::commands::init::run(&path)?;
        }

        // Uncomment as you implement each command
        // Commands::HashObject { write, file } => {
        //     oxid::commands::hash_object::run(&file, write)?;
        // }
        // Commands::CatFile {
        //     show_type,
        //     pretty_print,
        //     object,
        // } => {
        //     oxid::commands::cat_file::run(&object, show_type, pretty_print)?;
        // }
        // Commands::WriteTree => {
        //     oxid::commands::write_tree::run()?;
        // }
        // Commands::Commit { message } => {
        //     oxid::commands::commit::run(&message)?;
        // }
        // Commands::Add { files } => {
        //     oxid::commands::add::run(&files)?;
        // }
    }

    Ok(())
}
