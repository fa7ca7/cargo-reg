extern crate toml_edit;
extern crate url;
extern crate clap;
extern crate console;
extern crate failure;
extern crate cargo_reg;
#[macro_use] extern crate pretty_assertions;

use std::collections::HashMap;
use cargo_reg::config::Config;

const URL: &str = "http://my-reg.local/";

const CONTENT_ONE_REGISTRY: &str = r##"
[cargo-new]
# By default `cargo new` will initialize a new Git repository.
vcs = "none"

[registries]
my-reg = "http://my-reg.local/"
"##;

const CONTENT_TWO_REGISTRIES: &str = r##"
[cargo-new]
# By default `cargo new` will initialize a new Git repository.
vcs = "none"

[registries]
my-reg = "http://my-reg.local/"
my-reg33 = "http://my-reg.local/"
"##;

#[test]
#[should_panic]
fn corrupted_config_file() {
    let content = r##"
[cargo-new
# By default `cargo new` will initialize a new Git repository.
vcs = "none

[registries
my-reg33 = http://my-reg.local/"
"##;

    let _ = Config::new(content.to_string()).unwrap();
}

#[test]
fn add_empty_file() {
    let mut config = Config::new(String::new()).unwrap();
    config.add("my-reg", URL).unwrap();
    assert_eq!(&config.data(), r#"
[registries]
my-reg = "http://my-reg.local/"
"#);
}

#[test]
fn add_non_empty_file() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.add("my-reg33", URL).unwrap();
    assert_eq!(&config.data(), CONTENT_TWO_REGISTRIES);
}

#[test]
#[should_panic]
fn add_name_exist() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.add("my-reg", URL).unwrap();
}

#[test]
fn remove_non_empty_file() {
    let mut config = Config::new(CONTENT_TWO_REGISTRIES.to_string()).unwrap();
    config.remove("my-reg33").unwrap();
    assert_eq!(&config.data(), CONTENT_ONE_REGISTRY);
}

#[test]
fn remove_last_entry() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.remove("my-reg").unwrap();
    assert_eq!(&config.data(), r##"
[cargo-new]
# By default `cargo new` will initialize a new Git repository.
vcs = "none"
"##);
}

#[test]
#[should_panic]
fn remove_absent_name() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.remove("my-reg123").unwrap();
}

#[test]
fn rename_non_empty_file() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.rename("my-reg", "my-reg123").unwrap();
    assert_eq!(&config.data(), r##"
[cargo-new]
# By default `cargo new` will initialize a new Git repository.
vcs = "none"

[registries]
my-reg123 = "http://my-reg.local/"
"##);
}

#[test]
#[should_panic]
fn rename_absent_name() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.rename("my-reg123", "my-reg").unwrap();
}

#[test]
fn get_non_empty_file() {
    let config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    assert_eq!(config.get("my-reg").unwrap().as_str(), URL);
}

#[test]
#[should_panic]
fn get_absent() {
    let config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.get("my-reg123").unwrap();
}

#[test]
fn set_correct() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.set("my-reg", "http://my-reg.net/").unwrap();
    assert_eq!(&config.data(), r##"
[cargo-new]
# By default `cargo new` will initialize a new Git repository.
vcs = "none"

[registries]
my-reg = "http://my-reg.net/"
"##)
}

#[test]
#[should_panic]
fn set_absent() {
    let mut config = Config::new(CONTENT_ONE_REGISTRY.to_string()).unwrap();
    config.set("my-reg33", "http://my-reg.net/").unwrap();
}

#[test]
fn list() {
    let config = Config::new(CONTENT_TWO_REGISTRIES.to_string()).unwrap();

    let mut data = HashMap::new();
    data.insert("my-reg".to_string(), URL.to_string());
    data.insert("my-reg33".to_string(), URL.to_string());

    assert_eq!(config.list().unwrap(), data);
}
