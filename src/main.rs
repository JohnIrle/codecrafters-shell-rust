mod command;

use std::io::{self, Write};
use crate::command::Command;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // Get command
        let mut parts = input.split_whitespace();
        let command = parts.next().expect("No command");
        
        match Command::from(command) {
            Command::Builtin(builtin_command) => builtin_command.run(parts),
            Command::Executable(_) => unimplemented!(),
            Command::NotFound(command) => println!("{command}: command not found"), 
        }
    }
}
