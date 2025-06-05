fn main() {
    let mut s: String = String::new();

    loop {
        println!("Enter string:");
        io::stdin().read_line(&mut s).expect("Failed to read");
    
        println!("You input: {}", s);   
    }
}

