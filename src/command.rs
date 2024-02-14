use crate::bad_git::{self, BadGitError};

#[derive(Debug, PartialEq)]
pub enum Command {
    Init,
    Add(Vec<String>),
    Commit,
    Status,
}

impl Command {
    pub fn from_args(args: &Vec<String>) -> Result<Self, BadGitError> {
        match args.get(1).expect("No arguments provided").as_str() {
            "init" => Ok(Self::Init),
            "add" => {
                let files = args[2..].to_vec();

                if files.len() == 0 {
                    return Err(BadGitError::DidNotProvideFilesToAdd);
                }

                Ok(Self::Add(files))
            }
            "commit" => Ok(Self::Commit),
            "status" => Ok(Self::Status),
            _ => Err(BadGitError::InvalidCommand),
        }
    }

    pub fn execute(&self) -> Result<(), BadGitError> {
        if !bad_git::is_initialized() && *self != Command::Init {
            return Err(BadGitError::HasNotBeenInitialized);
        }

        match self {
            Command::Init => bad_git::init(),
            Command::Add(paths) => bad_git::add(paths)?,
            Command::Commit => todo!(),
            Command::Status => todo!(),
        };

        // println!("{:?}", self);

        Ok(())
    }
}
