mod command;
mod builtin;

use std::error::Error;
use crate::command::Command;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>>{
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // Get command and args
        let mut parts = input.split_whitespace();
        let command = parts.next().expect("No command");

        match Command::from(command) {
            Command::Builtin(builtin_command) => builtin_command.run(parts, &mut stdout)?,
            Command::Executable(path) => {
                std::process::Command::new(path)
                    .args(parts)
                    .status()
                    .expect("could not execute command");
            }
            Command::NotFound(command) => println!("{command}: command not found"),
        }
    }
}
