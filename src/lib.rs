//! This tool extends [cargo](http://doc.crates.io/) to allow you manage
//! alternative registries by modifying `.cargo/config` file from
//! the command line.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate toml_edit;
extern crate url;
extern crate clap;
extern crate console;
#[macro_use] extern crate failure;

pub mod config;
pub mod config_location;
pub mod errors;
