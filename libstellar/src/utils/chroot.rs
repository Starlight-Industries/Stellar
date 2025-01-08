use std::{
    io::{Read, Write},
    path::PathBuf,
};

use crate::Error;

const CHROOT_EXE: &'static str = "chroot";

pub struct Chroot {
    child_process: std::process::Child,
    pub current_executable: String,
    pub args: ChrootArgs,
}

impl Chroot {
    pub fn execute(&mut self, command: &str) -> anyhow::Result<(), Error> {
        if let Some(mut stderr) = self.child_process.stderr.take() {
            let mut buffer: Vec<u8> = vec![];

            let result = stderr
                .read_to_end(&mut buffer)
                .expect("Failed to read stdout");

            let child_stderr = String::from_utf8(buffer).unwrap();
            if child_stderr.contains("Operation not permitted") {
                let command: String = command.into();
                let reason = "Operation not permitted".into();
                return Err(Error::CommandFaliure { command, reason });
            }
        }

        if let Some(mut stdin) = self.child_process.stdin.take() {
            writeln!(stdin, "echo 'Hello world!'").unwrap();
        }

        Ok(())
    }

    pub fn new(base_dir: String, args: ChrootArgs) -> anyhow::Result<Self, Error> {
        setup_directories(PathBuf::from(&base_dir))?;
        let mut binding = std::process::Command::new("sudo");
        let mut command_binding: &mut std::process::Command = binding.arg(CHROOT_EXE);
        let command = command_binding.stderr(std::process::Stdio::piped());

        let child_process = command.arg(base_dir).spawn()?;

        Ok(Self {
            current_executable: String::new(),
            args,
            child_process,
        })
    }
}

pub struct ChrootArgs {
    pub groups: Vec<String>,
    pub userspec: Vec<String>,
    pub skip_chdir: bool,
}

struct ArgBuilder {
    groups: Vec<String>,
    userspec: Vec<String>,
    skip_chdir: bool,
}

impl ArgBuilder {
    pub fn new() -> Self {
        Self {
            groups: vec![],
            userspec: vec![],
            skip_chdir: false,
        }
    }

    pub fn groups(mut self, groups: Vec<String>) -> Self {
        self.groups = groups;
        self
    }

    pub fn userspec(mut self, userspec: Vec<String>) -> Self {
        self.userspec = userspec;
        self
    }

    pub fn skip_chdir(mut self, skip_chdir: bool) -> Self {
        self.skip_chdir = skip_chdir;
        self
    }

    pub fn build(self) -> ChrootArgs {
        ChrootArgs {
            groups: self.groups,
            userspec: self.userspec,
            skip_chdir: self.skip_chdir,
        }
    }
}

fn setup_directories(path: PathBuf) -> anyhow::Result<(), crate::Error> {
    ArgBuilder::new().groups(vec![String::new()]).build();
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    #[test]
    fn test_chroot() {
        let temp_dir = TempDir::with_prefix("stellar-chroot-").unwrap();
        let temp_dir_name = temp_dir.path().to_str().unwrap();

        let result = std::panic::catch_unwind(|| {
            let mut new_chroot = Chroot::new(
                temp_dir_name.to_string(),
                ChrootArgs {
                    groups: vec![],
                    userspec: vec![],
                    skip_chdir: false,
                },
            )
            .expect("Failed to create new chroot");

            new_chroot.execute("hi").unwrap();
            // new_chroot.execute("f").unwrap();
            // new_chroot.execute("hi").unwrap();
        });
        println!("{}", "Test failed but we unwrapped");

        assert!(result.is_ok());
    }
}
