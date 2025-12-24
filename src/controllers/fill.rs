use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>, key: String, value: String) {
    store.insert(key, value);
}
