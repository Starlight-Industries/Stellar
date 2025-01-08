pub mod chroot;
pub mod pacstrap;

trait BootstrapBuilder {
    type Error;

    fn package(&mut self, pkg_name: String) -> Result<&mut Self, Self::Error>;
    fn build(&self) -> Result<(), Self::Error>;
}

struct Pacstrap {
    packages: Vec<String>,
}

impl Pacstrap {
    fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }
}

impl BootstrapBuilder for Pacstrap {
    type Error = String;

    fn package(&mut self, pkg_name: String) -> Result<&mut Self, Self::Error> {
        if pkg_name.is_empty() {
            Err("Package name cannot be empty".to_string())
        } else {
            println!("Adding package: {}", pkg_name);
            self.packages.push(pkg_name);
            Ok(self)
        }
    }
    fn build(&self) -> Result<(), Self::Error> {
        if self.packages.is_empty() {
            Err("No packages specified. Cannot build.".to_string())
        } else {
            println!("Building bootstrap environment with packages:");
            for pkg in &self.packages {
                println!("- {}", pkg);
            }
            Ok(())
        }
    }
}

fn is_package(pkg_name: &str) -> bool {
    pkg_name.starts_with("pkg")
}

fn test_packages(packages: Vec<String>) {
    let mut builder = Pacstrap::new();

    for pkg_name in packages {
        if is_package(&pkg_name) {
            println!("{} is a valid package", pkg_name);
            if let Err(err) = builder.package(pkg_name) {
                eprintln!("Error adding package: {}", err);
            }
        } else {
            println!("{} is not a valid package", pkg_name);
        }
    }
    if let Err(err) = builder.build() {
        eprintln!("Error building bootstrap environment: {}", err);
    }
}
