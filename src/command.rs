use std::path::PathBuf;
use crate::builtin::Builtin;

#[derive(Debug, PartialEq, Eq)]
pub enum Command<'a> {
    Builtin(Builtin),
    Executable(PathBuf),
    NotFound(&'a str),
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(builtin) = Builtin::try_from(value) {
            return Self::Builtin(builtin);
        }

        if let Some(executable) = std::env::var("PATH")
            .expect("PATH not set")
            .split(':')
            .map(PathBuf::from)
            .find_map(|path| {
                let executable = path.join(value);
                if executable.exists() && executable.is_file() {
                    Some(executable)
                } else {
                    None
                }
            })
        {
            return Self::Executable(executable);
        }

        Self::NotFound(value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use tempfile::tempdir;

    #[test]
    fn command_returns_correct_builtins() {
        let command = Command::from("exit");
        assert_eq!(command, Command::Builtin(Builtin::Exit));

        let command = Command::from("echo");
        assert_eq!(command, Command::Builtin(Builtin::Echo));

        let command = Command::from("type");
        assert_eq!(command, Command::Builtin(Builtin::Type));
    }

    #[test]
    fn command_returns_executable_if_on_path() {
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

        // Test the Command::from function
        let command = Command::from("test_executable");

        // Check the result
        assert_eq!(command, Command::Executable(executable_path));
    }

    #[test]
    fn command_returns_not_found_when_executable_not_found() {
        // Save the current PATH
        let old_path = env::var("PATH").unwrap();

        // Set a new empty PATH
        env::set_var("PATH", "");

        // Test the Command::from function
        let command = Command::from("nonexistent_executable");

        // Restore the original PATH
        env::set_var("PATH", old_path);

        // Check the result
        assert_eq!(command, Command::NotFound("nonexistent_executable"));
    }


}
