#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};
use is_executable::IsExecutable;

enum Command {
    Exit,
    Echo(Option<String>),
    Type(Option<String>)
}

pub fn run() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input");
        let command = parse_input(input.trim());

        match command {
            Some(Command::Exit) => process::exit(0),
            Some(Command::Echo(arg)) => {
                if let Some(arg) = arg {
                    println!("{}", arg);
                } else {
                    println!();
                }
            },
            Some(Command::Type(arg)) => {
                if let Some(arg) = arg {    
                    match &arg[..] {
                        "exit" | "echo" | "type" => println!("{} is a shell builtin", arg),
                        other => {
                            match search_path_for_command(other) {
                                Ok(path) => println!("{} is {}", other, path),
                                Err(_error) => eprintln!("{}: not found", other)
                            }
                        }
                    }
                }
            }
            None => eprintln!("{}: command not found", input)
        };
    }
}

fn parse_input(input: &str) ->  Option<Command> {
    let command = match input {
        "exit" => Command::Exit,

        "echo" => Command::Echo(None),
        input if input.starts_with("echo ") => Command::Echo(Some(input[5..].trim().to_string())),

        "type" => Command::Type(None),
        input if input.starts_with("type ") => Command::Type(Some(input[5..].trim().to_string())),

        _ => return None
    };

    Some(command)
}

fn search_path_for_command(command: &str) -> Result<String, String> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() && entry.file_name() == command && entry.path().is_executable() {
                            let hit_path = entry.path().to_string_lossy().into_owned();
                            return Ok(hit_path)
                        }
                    }
                }
            }       
        }
        Err(String::from("No Match"))
    } else {
        Err(String::from("$PATH is not defined in the environment"))
    }
}
