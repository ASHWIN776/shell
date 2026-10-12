use std::path::Path;
use std::{env, fs, process};
use std::process::Command as StdCommand;
use is_executable::IsExecutable;

const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

pub enum Command {
    Exit,
    Echo(Option<String>),
    Type(Option<String>),
    Executable { name: String, args: Vec<String> },
    Pwd,
    Cd(Option<String>),
    Invalid(String)
}

pub fn is_builtin(arg: &str) -> bool {
    BUILTINS.contains(&arg)
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

// Execute commands - functions

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
        if is_builtin(&arg) {
            println!("{} is a shell builtin", arg)
        } else {
            match search_path_for_command(&arg) {
                Ok(path) => println!("{} is {}", arg, path),
                Err(_error) => eprintln!("{}: not found", arg)
            }
        }
    } else {
        println!();
    }
}

pub fn execute_external_program(commnd: &str, args: Vec<String>) {
    match StdCommand::new(commnd).args(args).status() {
        Ok(_) => {},
        Err(err) => {
            println!("Error in running program: {}", err)
        }
    }
}

pub fn execute_pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.to_string_lossy().into_owned())
        },
        Err(error) => {
            eprintln!("Failed to run pwd: {}", error);
        }
    }

}

pub fn execute_cd(arg: Option<String>) {
    if let Some(path) = arg {
        let root = Path::new(&path);
        
        if root.is_dir() {
            match env::set_current_dir(path) {
                Ok(_) => {},
                Err(error) => eprintln!("cd: Something went wrong: {}", error)
            }
        } else {
            eprintln!("cd: {}: No such file or directory", path);
        }
    }
}