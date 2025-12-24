use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>, key: String, value: String) {
    store.insert(key.clone(), value.clone());
    println!("Filled {} with the value of {}", key, value)
}
