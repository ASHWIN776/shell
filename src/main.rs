#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};
use is_executable::IsExecutable;

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
                    other => {
                        match search_path_for_command(other) {
                            Ok(path) => println!("{} is {}", other, path),
                            Err(_error) => eprintln!("{}: not found", other)
                        }
                    }
                }
            }
            other => eprintln!("{}: command not found", other)
        };
    }
}

fn search_path_for_command(command: &str) -> Result<String, String> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            // println!("Searching items in this PATH: {}", path.display());
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
