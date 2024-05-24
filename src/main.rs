#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    let builtin_commands = vec!["exit", "echo", "type"];
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
            command if command.starts_with("type") => {
                match command.split_once(' ') {
                    None => continue,
                    Some((_, arg)) => {
                        if builtin_commands.contains(&arg) {
                            println!("{arg} is a shell builtin")
                        } else {
                            println!("{arg} not found")
                        }
                    }
                }
            }
            command => println!("{command}: command not found")
        }
    }
}
