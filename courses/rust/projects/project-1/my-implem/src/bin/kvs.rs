use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(about = "Retrieve value from the key-value store")]
    Get(Get),

    #[clap(about = "Insert a value into the key-value store")]
    Set(Set),

    #[clap(about = "Remove a value from the key-value store")]
    Rm(Rm),
}

#[derive(Args)]
struct Get {
    #[arg(index = 1)]
    key: String,
}

#[derive(Args)]
struct Set {
    #[arg(index = 1)]
    key: String,

    #[arg(index = 2)]
    value: String,
}

#[derive(Args)]
struct Rm {
    #[arg(index = 1)]
    key: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommand::Get(_get) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        SubCommand::Set(_set) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        SubCommand::Rm(_rm) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
    }
}
