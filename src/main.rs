#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to get command");
        let command = command.trim();

        match command {
            "exit" => process::exit(0),
            other => eprintln!("{}: command not found", other)
        };
    }
}
