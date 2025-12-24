use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>, key: String) {
    if !store.contains_key(&key) {
        eprintln!("Error, {} does not exist", key)
    } else {
        store.insert(key, String::new());
    }
}