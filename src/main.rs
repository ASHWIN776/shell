#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input");
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        let command = &input[0];


        match *command {
            "exit" => process::exit(0),
            "echo" => {
                if input.len() > 1 {
                    println!("{}", &input[1..].join(" "));
                }
            },
            other => eprintln!("{}: command not found", other)
        };
    }
}
