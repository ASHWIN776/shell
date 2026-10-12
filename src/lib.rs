#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;
use commands::{Command, parse_input};

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to get input");
        let command = parse_input(command.trim());

        match command {
            Command::Exit => commands::execute_exit(),
            Command::Echo(arg) => commands::execute_echo(arg),
            Command::Type(arg) => commands::execute_type(arg),
            Command::Executable { name, args } => commands::execute_external_program(&name, args),
            Command::Pwd => commands::execute_pwd(),
            Command::Invalid(invalid_command) => eprintln!("{}: command not found", invalid_command)
        };
    }
}



