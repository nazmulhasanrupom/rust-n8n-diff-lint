/// input : 2 workflows.json or 1 workflow.json
/// output : It finds difference and tells us whats the difference  if it has 2 workflows
/// output : It tells us bad things in the workflow


use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Diff {
        old: String,
        new: String,
    },
    Lint {
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {

        Command::Diff{old, new} => {
            println!("Comparing {} to {}", old,new);
        }
        Command::Lint{file} => {
            println!("Linting {}", file);
        }
    }

}

