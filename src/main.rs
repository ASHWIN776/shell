#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input");
        input = input.trim().to_string();
    
        eprintln!("{}: command not found", input);
    }


}
