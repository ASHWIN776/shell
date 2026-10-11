use std::{env, fs, process};
use is_executable::IsExecutable;

pub enum Command {
    Exit,
    Echo(Option<String>),
    Type(Option<String>),
    Invalid(String)
}

pub fn parse_input(input: &str) ->  Command {
    match input {
        "exit" => Command::Exit,

        "echo" => Command::Echo(None),
        input if input.starts_with("echo ") => Command::Echo(Some(input[5..].trim().to_string())),

        "type" => Command::Type(None),
        input if input.starts_with("type ") => Command::Type(Some(input[5..].trim().to_string())),

        other => Command::Invalid(other.to_string())
    }
}

pub fn execute_exit() {
    process::exit(0)
}

pub fn execute_echo(arg: Option<String>) {
    if let Some(arg) = arg {
        println!("{}", arg);
    } else {
        println!();
    }
}

pub fn execute_type(arg: Option<String>) {
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
    } else {
        println!();
    }
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



