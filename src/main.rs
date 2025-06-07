mod keyvaluestore;
mod parser;

use std::io;
use parser::Parser;

fn main() {
    let mut cmd:     String = String::new(); 
    let mut parser:  Parser = Parser::new();
    // let mut new_key:     String = String::new();
    // let mut new_val:     String = String::new();
    // let mut key_to_find: String = String::new();
    // let mut cache:       KeyValueStore = KeyValueStore::new();

    loop {
        // println!("Enter Key:");
        // io::stdin().read_line(&mut new_key).expect("Failed to read");
        
        // println!("Enter Value:");
        // io::stdin().read_line(&mut new_val).expect("Failed to read");

        // cache.insert(new_key.trim().to_string(), new_val.trim().to_string());
        // println!("Inserted to cache");

        // println!("Enter Key to find:");
        // io::stdin().read_line(&mut key_to_find).expect("Failed to read");

        // match cache.get(&key_to_find.trim()) {
        //     Some(val) => println!("Found: {}", val),
        //     None            => println!("Key not found")
        // }

        // new_key.clear();
        // new_val.clear();
        // key_to_find.clear();

        io::stdin().read_line(&mut cmd).expect("Failed to read");
        let res: Result<&str, &str> = parser.process_command(cmd.as_str());

        match res {
            Ok(_)        => println!("Command ran successfully."),
            Err(e) => println!("{}", e)
        }

        cmd.clear();
    }
}
