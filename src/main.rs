pub mod command;
pub mod bad_git;

use command::{BadGitError, Command};
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
