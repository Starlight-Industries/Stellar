use thiserror::Error;

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
}

pub fn test() {
    let mut test: Vec<String> = Vec::new();
    for i in 0..50 {
        test.push(format!("test #{}",{
            let new = i + 1;
            new
        }));
    }
    let new_string = test.join("\n");
    println!("{}", new_string);
}
impl super::Bootstrap for Pacstrap {
    type Error = String;
    fn bootstrap() {
        todo!()
    }
    fn add_package() -> Result<(), Self::Error> {
        todo!()
    }
}

struct PacstrapBuilder;
