use crate::bad_git;

#[derive(Debug)]
pub enum BadGitError {
    NoArgumentsProvided,
    InvalidCommand,
    DidNotProvideFilesToAdd,
    HasNotBeenInitialized,
}

#[derive(Debug)]
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
        if !bad_git::is_initialized() {
            return Err(BadGitError::HasNotBeenInitialized)
        }

        println!("{:?}", self);
        Ok(())
    }
}

