//! Module contains utils for loading and persisting
//! TOML data from/to the config file.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::env;
use std::path::{Path, PathBuf};

use failure::Error;

use config::Config;

/// Location of config which operate on
#[derive(Debug)]
pub enum ConfigLocation {
    /// $CARGO_HOME/config
    Global,
    /// $PWD/.cargo/config
    Local,
    /// All the fields from global and local
    System,
}

impl ConfigLocation {
    /// Construct `ConfigLocation` based on flags given from CLI
    pub fn new(global: bool, local: bool) -> Result<ConfigLocation, Error> {
        let location = match (global, local) {
            (true, true) => {
                return Err(format_err!("Cannot use both --global and --local. Consider to use one \
        of them or use nothing.\n\nIf one use nothing cargo-reg assumes using --local for all \
        the commands except list(). List will show the merge of global and local configs."))
            }
            (false, false) => ConfigLocation::System,
            (false, true) => ConfigLocation::Local,
            (true, false) => ConfigLocation::Global,
        };

        Ok(location)
    }
}

/// Define a file path to the config file
pub fn locate_config(location: &ConfigLocation) -> Result<PathBuf, Error> {
    let config_dir = match location {
        &ConfigLocation::Global => env::home_dir(),
        &ConfigLocation::System | &ConfigLocation::Local => env::current_dir().ok(),
    }.map(|p| p.join(".cargo"));

    let config_dir = match config_dir {
        Some(c) => c,
        None => bail!("Failed to determine config path"),
    };

    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config"))
}

/// Open or create a config file
pub fn open_config(path: &Path) -> Result<(String, File), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok((data, file))
}

/// Overwrite a config file with TOML data.
pub fn write_config(config: &Config, file: &mut File) -> Result<(), Error> {
    let new_content = config.data();
    let new_content = new_content.trim().as_bytes();
    file.seek(SeekFrom::Start(0))?;
    file.set_len(new_content.len() as u64)?;
    file.write_all(new_content)?;
    Ok(())
}
