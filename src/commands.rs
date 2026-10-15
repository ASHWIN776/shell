use std::path::{PathBuf};
use std::{env, process};
use std::process::Command as StdCommand;
use crate::utils::search_path_for_command;

const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

pub fn is_builtin(arg: &str) -> bool {
    BUILTINS.contains(&arg)
}

pub enum Command {
    Exit,
    Echo(Option<String>),
    Type(Option<String>),
    Executable { name: String, args: Vec<String> },
    Pwd,
    Cd(Option<String>),
    Invalid(String)
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Exit => execute_exit(),
            Command::Echo(arg) => execute_echo(arg),
            Command::Type(arg) => execute_type(arg),
            Command::Executable { name, args } => execute_external_program(&name, args),
            Command::Pwd => execute_pwd(),
            Command::Cd(arg) => execute_cd(arg),
            Command::Invalid(invalid_command) => eprintln!("{}: command not found", invalid_command)
        }
    }
}

// Execute commands - functions

fn execute_exit() {
    process::exit(0)
}

fn execute_echo(arg: Option<String>) {
    if let Some(arg) = arg {
        println!("{}", arg);
    } else {
        println!();
    }
}

fn execute_type(arg: Option<String>) {
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

fn execute_external_program(commnd: &str, args: Vec<String>) {
    match StdCommand::new(commnd).args(args).status() {
        Ok(_) => {},
        Err(err) => {
            println!("Error in running program: {}", err)
        }
    }
}

fn execute_pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.to_string_lossy().into_owned())
        },
        Err(error) => {
            eprintln!("Failed to run pwd: {}", error);
        }
    }

}

fn execute_cd(arg: Option<String>) {
    if let Some(path) = arg {
        let path = if path.starts_with("~") {
            let home_path = match env::home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("cd: No Home directory found here");
                    return;
                }
            };

            if path == "~" {
                home_path
            } else if path.starts_with("~/") {
                home_path.join(&path[2..])
            } else {
                eprintln!("cd: {}: No such file or directory", path);
                return;
            }
        } else {
            PathBuf::from(path)
        };
        
        if path.is_dir() {
            match env::set_current_dir(path) {
                Ok(_) => {},
                Err(error) => eprintln!("cd: Something went wrong: {}", error)
            }
        } else {
            eprintln!("cd: {}: No such file or directory", path.to_string_lossy());
        }
    }
}