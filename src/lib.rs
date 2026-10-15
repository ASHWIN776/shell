#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;
mod utils;
use utils::parse_input;

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to get input");
        let command = parse_input(command.trim());
        command.run();
    }
}



