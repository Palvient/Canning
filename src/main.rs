use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: can <command> [args]");
        return;
    }

    match args[1].as_str() {
        "set" => {
            println!("Feature still in development")
        }
        "get" => {
            println!("Feature still in development")
        }
        "remove" => {
            println!("Feature still in development")
        }
        "list" => {
            println!("Feature still in development")
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
