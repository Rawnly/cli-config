/*!
  This module contains file utilities
  The following traits should be used to ease
  read/write operation in the fs.

  ## Example
  ```rust
    use cli_config::fs::*;
    use cli_config::config;
    use cli_config::error::Error;

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    struct MyConfig {
      is_first_install: bool
    }

    impl JSONFile for MyConfig {}

    fn main() -> cli_config::Result<()> {
      let mut config_path = config::get_path("hawk", "config.json")?;

      if let None = config_path {
        config_path = config::get_new_path("hawk", "config.json");

        match config::get_new_path("hawk", "config.json") {
            None => return Err(Error::Custom("Could not create config file.")),
            Some(config_path) => {
              Config::default().write(config_path.unwrap().as_path())?;
            }
        }
      }


      Ok(())
    }
  ```
*/

/// Generic trait
pub trait File {
    fn load(path: &Path) -> crate::Result<Self>
    where
        Self: Sized;

    fn write(&self, path: &Path) -> crate::Result<()>;
}

#[cfg(any(feature = "json", feature = "toml", feature = "yaml"))]
use serde::de::DeserializeOwned;

use std::path::Path;
#[cfg(any(feature = "json", feature = "toml", feature = "yaml"))]
use std::{fs, path::Path};

#[cfg(feature = "toml")]
use std::io::Write;

#[cfg(any(feature = "json", feature = "toml", feature = "yaml"))]
use crate::error::Error;

#[cfg(feature = "json")]
pub trait JSONFile
where
    Self: DeserializeOwned + serde::Serialize,
{
    /// Load file content into `Self`
    fn load(path: &Path) -> crate::Result<Self> {
        let file = fs::File::open(path)?;

        serde_json::from_reader(file).map_err(Error::JSON)
    }

    /// Write `Self` into specified file
    fn write(&self, path: &Path) -> crate::Result<()> {
        let file = fs::File::create(path)?;

        serde_json::to_writer_pretty(file, self).map_err(Error::JSON)
    }
}

#[cfg(feature = "yaml")]
pub trait YAMLFile
where
    Self: DeserializeOwned + serde::Serialize,
{
    /// Load file content into `Self`
    fn load(path: &Path) -> crate::Result<Self> {
        let file = fs::File::open(path)?;

        serde_yaml::from_reader(file).map_err(Error::YAML)
    }

    /// Write `Self` into specified file
    fn write(&self, path: &Path) -> crate::Result<()> {
        let file = fs::File::create(path)?;

        serde_yaml::to_writer(file, self).map_err(Error::YAML)
    }
}

#[cfg(feature = "toml")]
pub trait TOMLFile
where
    Self: DeserializeOwned + serde::Serialize,
{
    /// Load file content into `Self`
    fn load(path: &Path) -> crate::Result<Self> {
        let file = fs::read_to_string(path)?;

        toml::from_str(&file).map_err(error::Error::TOML)
    }

    /// Write `Self` into specified file
    fn write(&self, path: &Path) -> crate::Result<()> {
        let mut file = fs::File::create(path)?;
        let str = toml::ser::to_string(&self)?;

        file.write_all(&str.as_bytes())?;

        Ok(())
    }
}
