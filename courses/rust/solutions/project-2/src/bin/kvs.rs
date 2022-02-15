use clap::Parser;
pub use kvs::KvStore;
use std::process;

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

fn set(store: &mut KvStore, key: String, value: String) {
    let ret = store.set(key.clone(), value.clone());
    match ret {
        Err(err) => println!("{}", err.msg),
        _ => (),
    }
    // println!("set {} to {}", &key, &value)
}

fn get(store: &KvStore, key: String) {
    let ret = store.get(key.clone());
    match ret {
        Ok(res) => match res {
            Some(value) => println!("get {}: {}", key, value),
            None => println!("{} not found", key),
        },
        Err(err) => println!("{}", err.msg),
    }
}

fn remove(store: &mut KvStore, key: String) {
    let ret = store.remove(key.clone());
    match ret {
        Err(err) => {
            println!("{}", err.msg);
            process::exit(1);
        }
        _ => (),
    }
}

fn main() {
    let mut store = KvStore::new();

    let args = Args::parse();
    // println!("{}", args.action);
    // println!("{:?}", args.action::Get);
    match args.action {
        Action::Set { key, value } => set(&mut store, key, value),
        Action::Get { key } => get(&store, key),
        Action::RM { key } => remove(&mut store, key),
    }
}
