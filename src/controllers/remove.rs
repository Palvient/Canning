use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>, key: String)  {
    store.remove(&key);
    println!("Removed {}", key)
}
