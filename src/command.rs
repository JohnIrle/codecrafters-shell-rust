use std::path::PathBuf;
use std::str::SplitWhitespace;

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

#[derive(Debug, PartialEq, Eq)]
pub enum Builtin {
    Exit,
    Echo,
    Type,
}


impl Builtin {
    pub fn run(&self, mut args: SplitWhitespace) {
        match self {
            Self::Exit => {
                std::process::exit(args.next().map_or(0, |code| {
                    code.parse().expect("could not parse exit code to i32")
                }))
            }
            Self::Echo => {
                println!("{}", args.collect::<Vec<_>>().join(" "))
            }
            Self::Type => {
                let command = args.next().expect("missing arg");
                match Command::from(command) {
                    Command::Builtin(_) => println!("{command} is a shell builtin"),
                    Command::Executable(path) => println!("{command} is {path}", path = path.to_string_lossy()),
                    Command::NotFound(_) => println!("{command}: not found"),
                }
            }
        }
    }
}

impl TryFrom<&str> for Builtin {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "exit" => Ok(Builtin::Exit),
            "echo" => Ok(Builtin::Echo),
            "type" => Ok(Builtin::Type),
            _ => Err(())
        }
    }
}