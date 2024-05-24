#[allow(unused_imports)]
use std::io::{self, Write};

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
        let command = input.trim();
        match command {
            command if command.starts_with("exit") => {
                match command.split_once(' ') {
                    None => std::process::exit(0),
                    Some((_, arg)) => std::process::exit(arg.parse().unwrap())
                }
            }
            command if command.starts_with("echo") => {
                match command.split_once(' ') {
                    None => println!(),
                    Some((_, arg)) => println!("{arg}")
                }
            }
            command => println!("{command}: command not found")
        }
    }
}
