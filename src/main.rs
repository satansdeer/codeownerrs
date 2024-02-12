use clap::{Parser, Subcommand};
use codeownerrs::code_owners::CodeOwners;
use codeownerrs::paths::list;
use std::path::Path;

/// A tool for interacting with GitHub's
/// [CODEOWNERS](https://help.github.com/articles/about-codeowners/) files.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a list of files in the current repo, followed by its owner
    Audit {
        /// Specify a non-standard CODEOWNERS filename
        #[arg(short, long, default_value_t = String::from("CODEOWNERS"))]
        file: String,

        /// Max traversal depth
        #[arg(short, long)]
        depth: Option<usize>,

        /// List the files not covered by the `CODEOWNERS`
        #[arg(short, long)]
        unowned: bool,
    },
    /// Verify that users/teams own a specific path
    Verify {
        /// Path to verify
        path: Option<String>,

        /// A space-separated list of users/teams
        users: Vec<String>,

        /// Specify a non-standard CODEOWNERS filename
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Audit {
            file,
            depth,
            unowned,
        } => {
            let code_owners = CodeOwners::new(file).unwrap();

            for entry in code_owners.get_owners("/src") {
                println!("Owner of /src: {}", entry)
            }

            let result = list(*depth);
            for entry in result {
                println!("{}", entry.path().display())
            }
        }
        Commands::Verify { .. } => {
            println!("Verify!");
        }
    }
}
