use thiserror::Error;
struct Architecture {}
struct Pacstrap {
    packages: Vec<String>,
}
#[derive(Debug, Error, PartialEq)]
enum PacstrapError {
    #[error("Package {got} Was not found. Did you mean {}",
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
}

// impl super::Bootstrap for Pacstrap {
//     fn bootstrap() {
//         todo!()
//     }
//     fn add_package() -> Result<(), Self::Error> {
//     Ok(())
//
//     }
// }
