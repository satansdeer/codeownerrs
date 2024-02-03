use clap::{Parser, Subcommand};
use codeownerrs::paths::list;


/// A tool for interacting with GitHub's
/// [CODEOWNERS](https://help.github.com/articles/about-codeowners/) files.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
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
        depth: Option<i32>,

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
    let args = Args::parse();

    list();    
}
