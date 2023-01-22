#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot find file")]
    FileNotFound,

    #[error("invalid config: {0}")]
    InvalidConfig(&'static str),

    #[cfg(feature = "json")]
    #[error("invalid json: {0}")]
    JSON(#[from] serde_json::Error),

    #[cfg(feature = "yaml")]
    #[error("invalid yaml: {0}")]
    YAML(#[from] serde_yaml::Error),

    #[cfg(feature = "toml")]
    #[error("invalid toml: {0}")]
    TOML(#[from] toml::de::Error),

    #[cfg(feature = "toml")]
    #[error("cannot serialize: {0}")]
    TomlWrite(#[from] toml::ser::Error),

    #[error("FileSystem error")]
    FileSystem(#[from] std::io::Error),

    #[error("the theme you are looking for does not exists")]
    ThemeNotFound,

    #[error("{0}")]
    Custom(&'static str),

    #[error("something went wrong: {0}")]
    Generic(#[from] anyhow::Error),
}
