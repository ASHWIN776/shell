#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to get input");
        let command = command.trim();

        match command {
            "exit" => process::exit(0),
            "echo" | "type" => println!(),
            command if command.starts_with("echo ") => {
                println!("{}", &command[5..]);
            },
            command if command.starts_with("type ") => {
                let arg = &command[5..];
                match arg {
                    "exit" | "echo" | "type" => println!("{} is a shell builtin", arg),
                    other => eprintln!("{}: not found", other)
                }
            }
            other => eprintln!("{}: command not found", other)
        };
    }
}
