use crate::keyvaluestore::KeyValueStore;

pub struct Parser {
    cache: KeyValueStore,
}

impl Parser {
    pub fn new() -> Self {
        Parser { cache: KeyValueStore::new() }
    }

    pub fn process_command(&mut self, raw_cmd: &str) -> Result<&str, &str> {
        // Get first command
        let cmd_words: Vec<&str> = raw_cmd.trim().split(' ').collect();
    
        if cmd_words.len() == 0 {
            println!("Error: Empty Command");
            return Err("Empty Command");
        }
    
        match cmd_words[0] {
            "GET"  => {
                         if cmd_words.len() != 2 {
                             println!("Invalid number of arguments. Required: 1, Found: {}", cmd_words.len() - 1);
                             return Err("Incorrect number of arguments");
                         }
                        
                         match self.get(cmd_words[1]) {
                             Some(val) => println!("Value: {}", val),
                             None            => println!("Key not found")
                         }
                      },
            
            "SET"  => {
                        if cmd_words.len() != 3 {
                            println!("Invalid number of arguments. Required: 2, Found: {}", cmd_words.len() - 1);
                            return Err("Incorrect number of arguments");
                        }

                        self.set(cmd_words[1], cmd_words[2]);
                      }
    
            "PING" => println!("PONG"),
    
            _      => {
                        println!("Error: {} is an invalid comand", cmd_words[0]);
                        return Err("Invalid Command");
                      }
        }
    
        return Ok("Success");
    }

    fn get(&self, key: &str) -> Option<&str> {
        println!("GET");
        return self.cache.get(key);
    }

    fn set(&mut self, key: &str, val: &str) {
        println!("SET");
        self.cache.insert(key.to_string(), val.to_string());
    }
}