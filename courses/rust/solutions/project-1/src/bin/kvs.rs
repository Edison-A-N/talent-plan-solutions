use clap::Parser;
pub use kvs::KvStore;

#[derive(clap::Parser)]
#[clap(name = "kvs", version = "0.1.0")]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    ///Get a value by key, default is None
    Get { key: String },
    ///Set a value by key
    Set { key: String, value: String },
    /// Remove a value by key
    RM { key: String },
}

fn main() {
    let args = Args::parse();
    // println!("{}", args.action);
    match args.action {
        _ => panic!("unimplemented"),
    }
}
