pub mod bad_git;
pub mod command;

use crate::bad_git::BadGitError;
use command::Command;
use std::env;


fn main() -> Result<(), BadGitError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(BadGitError::NoArgumentsProvided);
    }

    let command = Command::from_args(&args)?;

    command.execute()?;

    Ok(())
}
