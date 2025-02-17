//! Support for managing global configuration, as well as loading it from TOML

mod configurable;
mod overrides;
mod reader;

pub use self::{configurable::Configurable, overrides::Override, reader::Reader};
#[doc(hidden)]
pub use abscissa_derive::Config;

use crate::{
    error::{FrameworkError, FrameworkErrorKind::ConfigError},
    path::AbsPath,
};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, fs::File, io::Read};

/// Trait for Abscissa configuration data structures
pub trait Config: Debug + Default + DeserializeOwned {
    /// Load the configuration from the given TOML string
    fn load_toml<T: AsRef<str>>(toml_string: T) -> Result<Self, FrameworkError> {
        Ok(toml::from_str(toml_string.as_ref())?)
    }

    /// Load the global configuration from the TOML file at the given path.
    /// If an error occurs reading or parsing the file, print it out and exit.
    fn load_toml_file<P>(path: &P) -> Result<Self, FrameworkError>
    where
        P: AsRef<AbsPath>,
    {
        let mut file = File::open(path.as_ref()).map_err(|e| {
            err!(
                ConfigError,
                "couldn't open {}: {}",
                path.as_ref().display(),
                e
            )
        })?;

        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        Self::load_toml(toml_string)
    }
}
