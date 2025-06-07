mod keyvaluestore;

use std::io;
use keyvaluestore::KeyValueStore;

fn main() {
    let mut new_key:     String = String::new();
    let mut new_val:     String = String::new();
    let mut key_to_find: String = String::new();
    let mut cache:       KeyValueStore = KeyValueStore::new();

    loop {
        println!("Enter Key:");
        io::stdin().read_line(&mut new_key).expect("Failed to read");
        
        println!("Enter Value:");
        io::stdin().read_line(&mut new_val).expect("Failed to read");

        cache.insert(new_key.trim().to_string(), new_val.trim().to_string());
        println!("Inserted to cache");

        println!("Enter Key to find:");
        io::stdin().read_line(&mut key_to_find).expect("Failed to read");

        match cache.get(&key_to_find.trim()) {
            Some(val ) => println!("Found: {}", val),
            None             => println!("Key not found")
        }

        new_key.clear();
        new_val.clear();
        key_to_find.clear();
    }
}

