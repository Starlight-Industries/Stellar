use std::path::{Path, PathBuf};
use thiserror::Error;
pub struct Chroot {
    directory: PathBuf 
}

#[derive(Debug, Error)]
pub enum ChrootError {

	#[error("The directory you provided was not considered valid")]
	InvalidDirectory {
		got: String
	},
	#[error("An IO faliure has occured: {0}")]
	IoFailure(#[from] std::io::Error)
}

impl Chroot {
    pub fn new<P: AsRef<Path> + ToString>(path: P) -> Result<Self,ChrootError> {
        let directory = path.as_ref();
        if !directory.exists() {
            return Err(ChrootError::InvalidDirectory { got: path.to_string() })
        }
        let directory = directory.to_path_buf();
        let val = Self {
            directory
        };
        
        Ok(val)
    }
}