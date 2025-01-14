use std::fmt::Debug;
use std::path::Path;
use std::string::String;

use async_trait::async_trait;

use crate::bootstrap::BootstrapError;

#[derive(Default)]
pub struct Pacstrap {
	#[allow(private_interfaces)]
	packages: Vec<String>,
}
#[async_trait]
impl super::Bootstrap for Pacstrap {
	type Repository = ();

	const MANAGER_NAME: &'static str = "Pacman";
	const MANAGER_COMMAND: &'static str = "pacstrap";

	fn bootstrap<P: AsRef<Path> + ToString>(&self, directory: P) -> Result<(), BootstrapError> {
		if !directory.as_ref().exists() {
			return Err(BootstrapError::IoFailure(std::io::Error::new(std::io::ErrorKind::NotADirectory,
			"The requested directory did not exist"
			)))
		}
		let mut binding = std::process::Command::new("sudo");
		binding.arg(Self::MANAGER_COMMAND);


		binding.arg("-K");
		binding.arg(directory.to_string());
		let cmd = binding.args(&self.packages);

		let output = cmd.output();
		match output {
			Ok(out) => {
				let stdout_bytes = out.stdout;
				let stderr_bytes = out.stderr;
				let stdout_str = String::from_utf8(stdout_bytes);
				let stderr_str = String::from_utf8(stderr_bytes);
				println!("command output: {}", stdout_str.unwrap());
				println!("Command err: {}", stderr_str.unwrap())
			}
			Err(e) => match e.kind() {
				_ => todo!(),
			},
		}
		Ok(())
	}

	async fn add_package<S: AsRef<str> + std::marker::Send>(
		&mut self,
		name: S,
	) -> Result<(), BootstrapError> {
		let pkg_string = String::from(name.as_ref());
		if self.packages.contains(&pkg_string) {
			return Ok(());
		}
		println!("adding {}", pkg_string);
		self.packages.push(pkg_string);
		Ok(())
	}

	async fn add_packages<Ps: AsRef<str> + Send>(
		&mut self,
		packages: Vec<Ps>,
	) -> Result<(), BootstrapError> {
		todo!()
	}

	async fn add_repository(&self, repo: Self::Repository) -> Result<(), BootstrapError> {
		todo!()
	}
}
