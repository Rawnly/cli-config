/*!
  This module contains file utilities
  The following traits should be used to ease
  read/write operation in the fs.
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

#[cfg(any(feature = "json", feature = "toml", feature = "yaml"))]
use std::fs;
use std::path::Path;

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

        toml::from_str(&file).map_err(Error::TOML)
    }

    /// Write `Self` into specified file
    fn write(&self, path: &Path) -> crate::Result<()> {
        let mut file = fs::File::create(path)?;
        let str = toml::ser::to_string(&self)?;

        file.write_all(&str.as_bytes())?;

        Ok(())
    }
}

mod test_utils {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct TestConfig {
        pub foo: String,
        pub bar: bool,
        pub baz: u32,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                foo: "foo".to_string(),
                bar: true,
                baz: 42,
            }
        }
    }
}

#[cfg(feature = "toml")]
#[cfg(test)]
mod toml_tests {
    use super::*;
    use tempdir::TempDir;
    use test_utils::TestConfig;

    impl TOMLFile for TestConfig {}

    #[test]
    fn test_file_trait() {
        let dir = TempDir::new("test_config").unwrap();
        let config_file = dir.path().join("test-config.json");
        let config = TestConfig::default();

        // test write and load
        config.write(&config_file).unwrap();
        let loaded_config = TestConfig::load(&config_file).unwrap();
        assert_eq!(config, loaded_config);
    }
}

#[cfg(feature = "yaml")]
#[cfg(test)]
mod yaml_tests {
    use super::*;
    use tempdir::TempDir;
    use test_utils::TestConfig;

    impl YAMLFile for TestConfig {}

    #[test]
    fn test_file_trait() {
        let dir = TempDir::new("test_config").unwrap();
        let config_file = dir.path().join("test-config.json");
        let config = TestConfig::default();

        // test write and load
        config.write(&config_file).unwrap();
        let loaded_config = TestConfig::load(&config_file).unwrap();
        assert_eq!(config, loaded_config);
    }
}

#[cfg(feature = "json")]
#[cfg(test)]
mod json_tests {
    use super::*;
    use tempdir::TempDir;
    use test_utils::TestConfig;

    impl JSONFile for TestConfig {}

    #[test]
    fn test_file_trait() {
        let dir = TempDir::new("test_config").unwrap();
        let config_file = dir.path().join("test-config.json");
        let config = TestConfig::default();

        // test write and load
        config.write(&config_file).unwrap();
        let loaded_config = TestConfig::load(&config_file).unwrap();
        assert_eq!(config, loaded_config);
    }
}
