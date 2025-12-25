use std::collections::HashMap;

pub fn run(store: &mut HashMap<String, String>)  {
    for (key, value) in store.iter() {
        println!("{key} => {value}");
    }
}
