#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    input = input.trim().to_string();

    eprintln!("{}: command not found", input);
    process::exit(1);
}
