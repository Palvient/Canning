use std::env;
use std::collections::HashMap;

mod controllers {        
    pub mod fill;         
}

use controllers::fill; 

fn main() {
    let mut cans: HashMap<String, String> = HashMap::new();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {   
        eprintln!("Usage: can fill <key> <value>");
        return;
    }

    match args[1].as_str() {
        "fill" => {
            let key = args[2].clone();
            let value = args[3].clone();
            fill::run(&mut cans, key, value);
        }
        "get" => {
            println!("Feature still in development")
        }
        "empty" => {
            println!("Feature still in development")
        }
        "delete" => {
            println!("Feature still in development")
        }
        "list" => {
            println!("Feature still in development")
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
