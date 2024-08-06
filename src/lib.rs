use std::path::PathBuf;

use dotenvy::{dotenv, from_filename, Result};

pub enum Environment {
    Development,
    Test,
    Production,
}

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
        Environment::Development => {
            add_to_env(".env.development.local")?;
            add_to_env(".env.local")?;
            add_to_env(".env.development")?;
        }
        Environment::Test => {
            add_to_env(".env.test.local")?;
            add_to_env(".env.test")?;
        }
        Environment::Production => {
            add_to_env(".env.production.local")?;
            add_to_env(".env.local")?;
            add_to_env(".env.production")?;
        }
    }
    paths.push(dotenv()?);

    Ok(paths)
}

pub fn rubenvy_development() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Development)
}

pub fn rubenvy_test() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Test)
}

pub fn rubenvy_production() -> Result<Vec<PathBuf>> {
    rubenvy(Environment::Production)
}
