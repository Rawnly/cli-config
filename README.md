# CLI Config

A simple library that provides utilities for managing configuration files in command-line applications.

## Installation

To use this library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cli_config = "0.1"
```

By default this crate supports JSON. However, you can enable support for other file formats by using one or more of the following feature flags:

- `toml`: Enables support for TOML files
- `yaml`: Enables support for YAML files

If you need a custom implementation you can always implement the `File` trait yourself and adapt it to your needs.

```rust
use cli_config::fs::File;

impl File for MyConfig {
  // ...
}
```

## Usage

Here's an example of how to use this crate in order to manage config files:

```rust
use cli_config::{Result, fs::JSONFile};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
  first_run: bool
}

impl cli_config::fs::JSONFile for MyConfig {}

impl Default for MyConfig {
    fn default() -> Self {
        MyConfig {
          first_run: true
        }
    }
}


fn main() -> Result<()> {
    let config_file = cli_config::init("my_cli_tool", "config.json")
        .ok_or("Could not locate config file")?;

    let mut config = MyConfig::load(&config_file)?;

    if config.first_run {
        // do your stuff on first run
        println!("Please login:");

        // update the config
        config.first_run = false;
        config.write(&config_file)?;
    }

    Ok(())
}
```

In this example, we use the `init` function to find the location of a configuration file named `config.json` in a directory called my_cli_tool.
If the file doesn't exist, the function will create it and populate it with some default values.

We then load the contents of the file into a `MyConfig` struct using the `JSONFile` trait, and check to see if the user has run the program before.

## License

This library is distributed under the terms of the MIT license. See [LICENSE](./LICENSE.md) for details
