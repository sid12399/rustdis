mod keyvaluestore;
mod parser;

use std::io;
use parser::Parser;

fn main() {
    let mut cmd:     String = String::new(); 
    let mut parser:  Parser = Parser::new();

    loop {
        io::stdin().read_line(&mut cmd).expect("Failed to read");
        let res: Result<&str, &str> = parser.process_command(cmd.as_str());

        match res {
            Ok(_)        => println!("Command ran successfully."),
            Err(e) => println!("{}", e)
        }

        cmd.clear();
    }
}
