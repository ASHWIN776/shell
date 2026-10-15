use std::{env, fs};
use is_executable::IsExecutable;
use crate::commands::Command;

pub fn search_path_for_command(command: &str) -> Result<String, String> {
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

pub fn parse_input(input: &str) ->  Command {
    match input {
        "exit" => Command::Exit,

        "echo" => Command::Echo(None),
        input if input.starts_with("echo ") => Command::Echo(Some(input[5..].trim().to_string())),

        "type" => Command::Type(None),
        input if input.starts_with("type ") => Command::Type(Some(input[5..].trim().to_string())),

        "pwd" => Command::Pwd,

        "cd" => Command::Cd(None),
        input if input.starts_with("cd ") => Command::Cd(Some(input[3..].trim().to_string())), 

        other => {
            let args: Vec<&str> = other.split_whitespace().collect();
            let command = &args[0];
            if let Ok(_path) = search_path_for_command(command) {
                if args.len() > 1 {
                    let args = args[1..].iter().map(|a| a.to_string()).collect();
                    Command::Executable { name: command.to_string(), args }
                } else {
                    Command::Executable { name: command.to_string(), args: Vec::new() }
                }
            } else {
                Command::Invalid(other.to_string())
            }
        }
    }
}