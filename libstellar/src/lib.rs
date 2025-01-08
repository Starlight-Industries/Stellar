pub mod utils;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The system command you are attempting to run has failed: {command} - {reason}")]
    CommandFaliure { command: String, reason: String },
    #[error("The action you are trying to perform has been denied by the system: {0}")]
    PermissionDenied(String),
    #[error("An IO faliure has occured: {0}")]
    IoError(#[from] std::io::Error),
    #[error("An Unknown error has occured: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
