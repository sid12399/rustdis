mod keyvaluestore;
mod parser;

//use std::io;
use parser::Parser;

// fn main() {
//     let mut cmd: String     = String::new(); 
//     let mut parser: Parser  = Parser::new();

//     loop {
//         io::stdin().read_line(&mut cmd).expect("Failed to read");
//         let res: Result<String, String> = parser.process_command(cmd.as_str());

//         match res {
//             Ok(_)        => println!("Command ran successfully."),
//             Err(e) => println!("{}", e)
//         }

//         cmd.clear();
//     }
// }

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut parser: Parser  = Parser::new();
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    let (mut tcp, _) = server.accept().await?;
    let mut buffer = [0u8; 16];
    loop {
        let n = tcp.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        
        let cmd: String = String::from_utf8(buffer[..n].to_vec())?;

        let res: Result<String, String> = parser.process_command(cmd.as_str());

        let out: String;
        match res {
            Ok(success_msg) => out = format!("{}\n", success_msg),
            Err(e)          => out = format!("{}\n", e)
        }
        let _ = tcp.write(&out.as_bytes()).await?;
    }
    Ok(())
}