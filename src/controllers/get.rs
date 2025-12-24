use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>, key: &str)  {
    let value = store.get(key).cloned().unwrap_or_default();
    println!("Value for {} is {}", key, value)
}
