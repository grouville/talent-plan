use clap::{Args, Parser, Subcommand};
use kvs::KvStore;
use kvs::MyError;

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
    let mut kv_store = KvStore::new();

    match cli.subcommand {
        SubCommand::Get(get) => {
            // Call the get method with the given key
            match kv_store.get(get.key) {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => println!("Key not found"),
                Err(e) => {
                    eprintln!("Error getting key: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        SubCommand::Set(set) => {
            // Call the set method with the given key and value
            match kv_store.set(set.key, set.value) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Error setting key: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        SubCommand::Rm(rm) => {
            if let Err(e) = kv_store.remove(rm.key) {
                match e {
                    MyError::KeyNotFound => println!("Key not found"),
                    _ => {
                        eprintln!("Error removing key: {:?}", e);
                    }
                }
                std::process::exit(1);
            }
        }
    }
}
