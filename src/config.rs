//! Main functionality for operating on alternative registries in `.cargo/config`.

use std::collections::HashMap;

use toml_edit::{self, Document};
use failure::Error;

use errors::*;

const REGISTRIES_ENTRY: &str = "registries";

/// Registries view: <ALIAS> => <INDEX_URL>
pub type Registries = HashMap<String, String>;

/// Representaion of `.cargo/config`.
/// All the modifications are performed in the memory, so the `Config` struct
/// should be written into a file afterwards using `config_location::write_config()`.
#[derive(Debug)]
pub struct Config {
    data: Document,
}

impl Config {
    /// Construct `Config` from content of the config file
    pub fn new(data: String) -> Result<Config, Error> {
        match data.parse() {
            Ok(data) => Ok(Config { data }),
            Err(_) => Err(RegistryError::BrokenConfig.into())
        }
    }

    /// Return current state as String
    pub fn data(&self) -> String {
        self.data.to_string()
    }

    /// Add a new alias and URL
    pub fn add(&mut self, name: &str, url: &str) -> Result<(), Error> {
        match self.check_table() {
            Ok(_) => {
                if !self.data[REGISTRIES_ENTRY][name].is_none() {
                    return Err(RegistryError::AlreadyExist { name: name.into() }.into());
                }
            },
            Err(_) => { self.data[REGISTRIES_ENTRY] = toml_edit::table(); }
        }

        self.data[REGISTRIES_ENTRY][name] = toml_edit::value(url);
        Ok(())
    }

    /// Remove an alias
    pub fn remove(&mut self, name: &str) -> Result<String, Error> {
        self.check_table()?;

        let registries = &mut self.data[REGISTRIES_ENTRY];
        if registries[name].is_none() {
            return Err(RegistryError::NoSuchRegistry { name: name.into() }.into());
        }

        let url = registries[name].as_str().unwrap().to_string();
        registries[name] = toml_edit::Item::None;

        // remove table if empty
        if registries.as_table_like().unwrap().is_empty() {
            *registries = toml_edit::Item::None;
        }

        Ok(url)
    }

    /// Rename an existing alias
    pub fn rename(&mut self, old: &str, new: &str) -> Result<(), Error> {
        let url = self.get(old)?;
        self.add(new, &url)?;
        let _ = self.remove(old)?;
        Ok(())
    }

    /// Get the URL for a given alias
    pub fn get(&self, name: &str) -> Result<String, Error> {
        self.check_table()?;

        let registry = &self.data[REGISTRIES_ENTRY][name];
        if registry.is_none() {
            return Err(RegistryError::NoSuchRegistry { name: name.into() }.into());
        }

        Ok(registry.as_str().unwrap().to_string())
    }

    /// Set a new URL for existing alias
    pub fn set(&mut self, name: &str, url: &str) -> Result<String, Error> {
        self.check_table()?;

        let registry = &mut self.data[REGISTRIES_ENTRY][name];
        if registry.is_none() {
            return Err(RegistryError::NoSuchRegistry { name: name.into() }.into());
        }

        let old = registry.as_str().unwrap().to_string();
        *registry = toml_edit::value(url);

        Ok(old)
    }

    /// List all the alias.
    /// * --global - list ONLY global config
    /// * --local  - list ONLY local config
    /// * both     - error, cannot use both the flags
    /// * nothing  - list global config but overwrite with local value; shows configuration for
    /// current directory
    pub fn list(&self) -> Result<Registries, Error> {
        let mut result = HashMap::new();
        let registries = match &self.data[REGISTRIES_ENTRY].as_table() {
            &Some(r) => r,
            &None => return Ok(result),
        };

        for (s, v) in registries.iter() {
            if let Some(v) = v.as_str() {
                result.insert(s.to_string(), v.to_string());
            }
        }

        Ok(result)
    }

    /// Helper that checks an existence of the table
    fn check_table(&self) -> Result<(), Error> {
        let reg = &self.data[REGISTRIES_ENTRY];
        if !reg.is_table_like() {
            return Err(RegistryError::TableAbsent.into());
        }

        Ok(())
    }
}
