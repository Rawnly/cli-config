use home::home_dir;
use std::path::PathBuf;

use crate::fs::File;

/// Get the path for creating a new config file
/// trying to use `$XDG_CONFIG_HOME/{prefix}/{filename}`
///
/// - `prefix` is the name of the folder that will contain the config file
#[cfg(not(windows))]
fn get_new_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    xdg::BaseDirectories::with_prefix(prefix)
        .ok()
        .and_then(|base| base.place_config_file(filename).ok())
}

/// Get the path for creating a new config file on windows
///
/// - `prefix` is the name of the folder that will contain the config file
#[cfg(windows)]
fn get_new_config_path(prefix: &str, filename: &str) -> Option<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(&format!("{}\\{}", prefix, filename)))
        .find(|p| p.exists())
}

/// Try to find the location of the first config file in the following paths:
///
/// 1. $XDG_CONFIG_HOME/{prefix}/{filename}.json
/// 2. $XDG_CONFIG_HOME/{prefix}.json
/// 3. $HOME/.config/{prefix}/{filename}
/// 4. $HOME/.{prefix}
#[cfg(not(windows))]
pub fn locate_config(prefix: &str, filename: &str) -> Option<PathBuf> {
    xdg::BaseDirectories::with_prefix(prefix)
        .ok()
        // Search for case n. 1
        .and_then(|xdg| xdg.find_config_file(filename))
        .or_else(|| {
            xdg::BaseDirectories::new()
                .ok()
                // Search for case n. 2
                .and_then(|fallback| fallback.find_config_file(format!("{prefix}.json")))
        })
        .or_else(|| {
            if let Some(home_path) = home_dir() {
                // Search for case n. 3 ($HOME/.config/{prefix}/{filename})
                let fallback_path = format!(".config/{prefix}");
                let fallback = home_path.join(fallback_path).join(filename);

                if fallback.exists() {
                    return Some(fallback);
                }

                // Search for case n. 4 ($HOME/.{prefix})
                let fallback = home_path.join(format!(".{prefix}.json"));

                if fallback.exists() {
                    return Some(fallback);
                }
            }

            None
        })
}

/// Get the location of the config file on windows
#[cfg(windows)]
pub fn locate_config(prefix: &str, filename: &str) -> Option<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join(&format!("{}\\{}", prefix, filename)))
        .filter(|p| p.exists())
}

/// Initialize the configuration file for the specified type.
///
/// This function returns the path to the configuration file for the specified type. If the file does not exist, it will be created.
///
/// # Arguments
///
/// * `config` - The configuration object to initialize the file with.
/// * `prefix` - The name of the folder that will contain the configuration file.
/// * `filename` - The name of the configuration file.
///
///
/// # Examples
///
/// ```
/// use cli_config::fs::JSONFile;
///
/// #[derive(serde::Serialize, serde::Deserialize, Default)]
/// struct MyConfig {
///     pub is_first_run: bool,
/// }
///
/// impl JSONFile for MyConfig {}
///
/// let config = MyConfig::default();
/// let prefix = "my-app";
/// let filename = "config.json";
///
/// // Initialize the configuration file
/// let config_path = cli_config::init(config, prefix, filename).unwrap();
///
/// // Use the configuration file
/// let loaded_config = MyConfig::load(&config_path).unwrap();
/// println!("Is my first run? {}", loaded_config.is_first_run);
/// ```
pub fn init<T>(config: T, prefix: &str, filename: &str) -> crate::Result<PathBuf>
where
    T: serde::Serialize + Default + File,
{
    let config_path = locate_config(prefix, filename);

    match config_path {
        None => match get_new_config_path(prefix, filename) {
            None => Err(crate::error::Error::Custom("Could not create file")),
            Some(path) => {
                config.write(&path)?;
                Ok(path)
            }
        },
        Some(path) => Ok(path),
    }
}
