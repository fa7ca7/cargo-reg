//! Errors for cargo-reg

/// Errors related to alternative registries
#[derive(Debug, Fail)]
pub enum RegistryError {
    /// Indicates an absence of the registry
    #[fail(display = "no such registry: {}", name)]
    NoSuchRegistry {
        /// Registry
        name: String
    },

    /// The registry already exists
    #[fail(display = "this registry already exists: {}", name)]
    AlreadyExist {
        /// Registry
        name: String
    },

    /// `registries` table is absent in the root of config file
    #[fail(display = "registries table is absent")]
    TableAbsent,

    /// Config file is invalid or non TOML
    #[fail(display = "config file is broken")]
    BrokenConfig,
}
