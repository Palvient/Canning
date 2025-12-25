use std::env;
use std::collections::HashMap;

mod controllers {        
    pub mod fill;   
    pub mod get;    
    pub mod empty;
    pub mod remove;
    pub mod list;
}

use controllers::{fill, get, empty, remove, list};

fn main() {
    let mut cans: HashMap<String, String> = HashMap::new();

    let args: Vec<String> = env::args().collect();

    let help_msg: String = "Usage: \n canning fill <key> <value> \n canning get <key> \n canning empty <key> \n canning remove <key> \n canning list".to_string();

    if args.len() < 3 {   
        eprintln!("{}", help_msg);
        return;
    }

    match args[1].as_str() {
        "fill" => {
            let key = args[2].clone();
            let value = args[3].clone();
            fill::run(&mut cans, key, value);
        }
        "get" => {
            let key = args[2].clone();
            get::run(&mut cans, &key);
        }
        "empty" => {
            let key = args[2].clone();
            empty::run(&mut cans, key)
        }
        "remove" => {
            let key = args[2].clone();
            remove::run(&mut cans, key);
        }
        "list" => {
            list::run(&mut cans);
        }
        "help" => {
            println!("{}", help_msg)
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
