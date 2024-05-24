use std::error::Error;
use std::str::SplitWhitespace;
use crate::command::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum Builtin {
    Exit,
    Echo,
    Type,
}

impl Builtin {
    pub fn run(&self, mut args: SplitWhitespace, mut writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Exit => std::process::exit(args.next().map_or(0, |code| {
                code.parse().expect("could not parse exit code to i32")
            })),
            Self::Echo => {
                writeln!(writer, "{}", args.collect::<Vec<_>>().join(" "))?;
            }
            Self::Type => {
                let command = args.next().expect("missing arg");
                match Command::from(command) {
                    Command::Builtin(_) => {
                        writeln!(writer, "{command} is a shell builtin")?;
                    }
                    Command::Executable(path) => {
                        writeln!(writer, "{command} is {path}", path = path.to_string_lossy())?;
                    }
                    Command::NotFound(_) => {
                        writeln!(writer, "{command}: not found")?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for Builtin {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "exit" => Ok(Builtin::Exit),
            "echo" => Ok(Builtin::Echo),
            "type" => Ok(Builtin::Type),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::{env, fs};
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn builtin_echo_prints_correct_values() {
        let builtin = Builtin::Echo;
        let args = "hello world how are you".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();
        assert_eq!(result, b"hello world how are you\n")
    }

    #[test]
    fn builtin_type_prints_correct_value_for_exit() {
        let builtin = Builtin::Type;
        let args = "exit".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();
        assert_eq!(result, b"exit is a shell builtin\n");
    }

    #[test]
    fn builtin_type_prints_correct_value_for_echo() {
        let builtin = Builtin::Type;
        let args = "echo".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();
        assert_eq!(result, b"echo is a shell builtin\n");
    }

    #[test]
    fn builtin_type_prints_correct_value_for_type() {
        let builtin = Builtin::Type;
        let args = "type".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();
        assert_eq!(result, b"type is a shell builtin\n");
    }

    #[test]
    fn builtin_type_prints_correct_value_for_executable() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create a temporary executable file
        let executable_path = dir_path.join("test_executable");
        fs::write(&executable_path, "#!/bin/sh\necho Hello").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        // Add the temporary directory to the PATH
        env::set_var("PATH", dir_path);

        let builtin = Builtin::Type;
        let args = "test_executable".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();

        assert_eq!(result, format!("test_executable is {}\n", executable_path.to_string_lossy()).as_bytes());
    }

    #[test]
    fn builtin_type_prints_correct_value_for_not_found() {
        let builtin = Builtin::Type;
        let args = "notfound".split_whitespace();
        let mut result = Vec::new();

        builtin.run(args, &mut result).unwrap();
        assert_eq!(result, b"notfound: not found\n");
    }

    #[test]
    fn builtin_try_from_returns_correct_variant() {
        let exit_builtin = Builtin::try_from("exit");
        assert_eq!(exit_builtin, Ok(Builtin::Exit));

        let echo_builtin = Builtin::try_from("echo");
        assert_eq!(echo_builtin, Ok(Builtin::Echo));

        let type_builtin = Builtin::try_from("type");
        assert_eq!(type_builtin, Ok(Builtin::Type));
    }

    #[test]
    fn builtin_try_from_returns_error() {
        let builtin_err = Builtin::try_from("nonexistent");
        assert_eq!(builtin_err, Err(()))
    }
}