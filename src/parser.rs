use crate::keyvaluestore::KeyValueStore;

pub struct Parser {
    cache: KeyValueStore,
}

impl Parser {
    pub fn new() -> Self {
        Parser { cache: KeyValueStore::new() }
    }

    pub fn process_command(&mut self, raw_cmd: &str) -> Result<String, String> {
        // Split the command into a list of tokens
        let success_msg: String;
        let cmd_words: Vec<&str> = raw_cmd.trim().split(' ').collect();
    
        if cmd_words.len() == 0 {
            return Err("Empty Command".to_string());
        }
        
        // Match the first word with available commands
        match cmd_words[0] {
            "GET"       =>  {
                                if cmd_words.len() != 2 {
                                    return Err(
                                            format!("Invalid number of arguments. Required: 1, Found: {}", cmd_words.len() - 1)
                                        );
                                }
                            
                                match self.get(cmd_words[1]) {
                                    Some(val) => success_msg = format!("Value: {}", val),
                                    None            => success_msg = format!("Key not found")
                                }
                            },
            
            "SET"       =>  {
                                if cmd_words.len() != 3 {
                                    return Err(
                                            format!("Invalid number of arguments. Required: 2, Found: {}", cmd_words.len() - 1)
                                        );
                                }

                                self.set(cmd_words[1], cmd_words[2]);
                                success_msg = format!("Successfully set pair.");
                            },

            "EXISTS"    =>  {
                                if cmd_words.len() != 2 {
                                    return Err(
                                            format!("Invalid number of arguments. Required: 1, Found: {}", cmd_words.len() - 1)
                                        );
                                }

                                if self.exists(cmd_words[1]) {
                                    success_msg = format!("Key {} exists.", cmd_words[1]);
                                }
                                else {
                                    success_msg = format!("Key does not exist.");
                                }
                            }, 
    
            "DEL"       =>  {
                                if cmd_words.len() != 2 {
                                    return Err(
                                            format!("Invalid number of arguments. Required: 1, Found: {}", cmd_words.len() - 1)
                                        );
                                }

                                match self.del(cmd_words[1]) {
                                    Ok(val)     => success_msg = format!("Deleted key with value: {}", val.as_str()),
                                    Err(err)    => return Err(err)
                                }
                            }, 
    
            "PING"      =>  {
                                success_msg = format!("PONG");
                            },
    
            _           =>  {
                                return Err(
                                        format!("Error: {} is an invalid comand", cmd_words[0])
                                    );
                            }
        }
    
        return Ok(success_msg);
    }

    fn get(&self, key: &str) -> Option<&str> {
        println!("GET");
        self.cache.get(key)
    }

    fn set(&mut self, key: &str, val: &str) {
        println!("SET");
        self.cache.insert(key.to_string(), val.to_string());
    }

    fn exists(&self, key: &str) -> bool {
        println!("EXISTS");
        self.cache.exists(key)
    }

    fn del(&mut self, key: &str) -> Result<String, String> {
        println!("DEL");
        self.cache.del(key)
    }
}