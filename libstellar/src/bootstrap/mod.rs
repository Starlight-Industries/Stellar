use std::path::Path;

use thiserror::Error;
#[cfg(feature = "archlinux")]
pub mod arch;
#[cfg(feature = "debian")]
pub mod debian;
use async_trait::async_trait;
#[async_trait]
/// This trait defines the methods required to implement a bootstrapper
/// for a specific package manager.
/// By default, this only includes APT & Pacman for Debian/Arch based
/// distributions respectively.
pub trait Bootstrap
where
	Self: Sized + Send + Sync + Default,
{
	/// Intended to be a struct that can be serialized and deserialized
	/// into a valid repository listing for the implemented
	/// package manager. For example:
	///
	/// ```toml
	/// #[custom]
	/// SigLevel = Optional TrustAll
	/// Server = file:///home/custompkgs
	/// ```
	/// This is Pacman's example of a custom package repository.
	/// Sourced from ```/etc/pacman.conf```
	type Repository;
	/// This constant defines the name of the package manager,
	/// This is different than the command used to invoke the manager, it is
	/// intended to be displayed for the user and not for functionality.
	const MANAGER_NAME: &'static str;
	/// This constant defines the name used to invoke the package manager
	const MANAGER_COMMAND: &'static str;
	fn bootstrap<P: AsRef<Path> + ToString>(&self, directory: P) -> Result<(), BootstrapError>;
	async fn add_package<P: AsRef<str> + Send>(&mut self, name: P) -> Result<(), BootstrapError>;
	async fn add_packages<Ps: AsRef<str> + Send>(
		&mut self,
		packages: Vec<Ps>,
	) -> Result<(), BootstrapError>;
	async fn add_repository(&self, repo: Self::Repository) -> Result<(), BootstrapError>;
}
#[derive(Debug, Error)]
pub enum BootstrapError {
	#[error("Package {got} Was not found. Did you mean {}?",
    closest_matches.join("\n"))]
	UnknownPackage {
		got: String,
		closest_matches: Vec<String>,
		exact_message: String,
	},
	#[error("the name \"{got}\" is not a valid package name")]
	InvalidPackage { got: String, exact_message: String },
	#[error("One or more repositories could not be reached{}",
    match reason {
        Some(reason) =>  ":".to_owned() + reason.as_str(),
        None => "".to_string(),
    })]
	NetworkFailure { reason: Option<String> },
	#[error("The directory you provided was not considered valid")]
	InvalidDirectory {
		got: String
	},
	#[error("An IO faliure has occured: {0}")]
	IoFailure(#[from] std::io::Error)
}