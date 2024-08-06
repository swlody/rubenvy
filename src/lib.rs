//! Support for [Ruby-style](https://github.com/bkeepers/dotenv/blob/c6e583a/README.md#what-other-env-files-can-i-use)
//! dotenv loading priorities utilizing [dotenvy](https://github.com/allan2/dotenvy).

use std::path::PathBuf;

use dotenvy::{from_filename, Result};

pub enum Environment {
    Test,
    Development,
    Production,
}

/// Load environment variables from .env files based on the provided environment.
/// Will prioritize files in the order described [here](https://github.com/bkeepers/dotenv/blob/c6e583a/README.md#should-i-commit-my-env-file).
pub fn rubenvy(environment: Environment) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::with_capacity(4);

    let mut add_to_env = |filename: &str| -> Result<()> {
        match from_filename(filename) {
            Ok(path) => Ok(paths.push(path)),
            Err(e) if e.not_found() => Ok(()),
            Err(e) => Err(e),
        }
    };

    match environment {
        Environment::Test => {
            add_to_env(".env.test.local")?;
            add_to_env(".env.test")?;
        }
        Environment::Development => {
            add_to_env(".env.development.local")?;
            add_to_env(".env.local")?;
            add_to_env(".env.development")?;
        }
        Environment::Production => {
            add_to_env(".env.production.local")?;
            add_to_env(".env.local")?;
            add_to_env(".env.production")?;
        }
    }
    add_to_env(".env")?;

    Ok(paths)
}

/// Automatically load environment variables based on the current build configuration.
///
/// `#cfg(test)` implies test environment.
/// `#cfg(not(debug_assertions))` implies production environment.
/// `#cfg(debug_assertions)` implies development environment.
pub fn rubenvy_auto() -> Result<Vec<PathBuf>> {
    let environment = if cfg!(test) {
        Environment::Test
    } else if cfg!(debug_assertions) {
        Environment::Development
    } else {
        Environment::Production
    };

    rubenvy(environment)
}

/// Load the test environment files.
pub fn rubenvy_test() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Test)
}

/// Load the development environment files.
pub fn rubenvy_development() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Development)
}

/// Load the production environment files.
pub fn rubenvy_production() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Production)
}
