extern crate toml_edit;
extern crate url;
extern crate clap;
extern crate console;
#[macro_use] extern crate failure;
extern crate cargo_reg;

use std::process;

use failure::Error;
use url::Url;
use clap::{App, AppSettings, Arg, SubCommand};
use console::style;

use cargo_reg::config::Config;
use cargo_reg::config_location::{ConfigLocation, locate_config, open_config, write_config};


struct VerboseLogger(bool);
impl VerboseLogger {
    fn verbose(&self, action: &str, msg: &str) {
        if self.0 {
            println!("{} {}", style(action).bold(), msg);
        }
    }
}

fn handle(args: &clap::ArgMatches) -> Result<(), Error> {
    let logger = VerboseLogger(args.is_present("verbose"));
    let global = args.is_present("global");
    let local = args.is_present("local");

    let config_location = ConfigLocation::new(global, local)?;
    let location = locate_config(&config_location)?;
    logger.verbose("Config", location.to_str().unwrap());

    let (data, mut file) = open_config(&location)?;
    let mut config = Config::new(data)?;

    match args.subcommand() {
        ("add", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = sub.value_of("url").unwrap();
            check_url(&url)?;

            config.add(name, url)?;
            logger.verbose("Added", &format!("{} => {}", &name, &url));
        }

        ("rm", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = config.remove(name)?;
            logger.verbose("Removed", &format!("{} => {}", &name, &url));
        }

        ("rename", Some(sub)) => {
            let old = sub.value_of("old").unwrap();
            let new = sub.value_of("new").unwrap();

            config.rename(old, new)?;
            logger.verbose("Renamed", &format!("{} => {}", &old, &new));
        }

        ("get", Some(sub)) => {
            println!("{}", config.get(sub.value_of("name").unwrap())?);
        }

        ("set", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = sub.value_of("url").unwrap();
            check_url(&url)?;

            let old = config.set(name, url)?;
            logger.verbose("Updated", &format!("{}: {{ {} => {} }}",
                                                          &name, &old, &url));
        }

        ("list", _) | ("", _) => {
            let mut registries = config.list()?;
            if let ConfigLocation::System = config_location {
                let location = locate_config(&ConfigLocation::Global)?;
                let (data, _) = open_config(&location)?;
                let global_config = Config::new(data)?;

                let mut global_regestries = global_config.list()?;
                global_regestries.extend(registries);
                registries = global_regestries;
            }

            if !registries.is_empty() {
                for (k, v) in registries {
                    println!("{} => {}", k, v);
                }
            } else {
                println!("Registries' list is empty");
            }

            return Ok(());
        }

        (some, Some(_)) | (some, None) => {
            return Err(format_err!("failed to parse arguments for: {}", some));
        }
    }

    write_config(&config, &mut file)?;
    Ok(())
}

fn check_url(s: &str) -> Result<Url, Error> {
    Url::parse(s).map_err(|e| {
        format_err!("invalid url `{}`: {}", s, e)
    })
}

fn main() {
    const CMD_ABOUT: &str = "This command allows you to manage alternative \
                            registries in .cargo/config file";

    let args = App::new("cargo-reg")
        .about(CMD_ABOUT)
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo, this trick learned from cargo-cook
        .bin_name("cargo")
        // We use a subcommand because parsed after `cargo` is sent to the third party plugin
        // which will be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("reg")
                        .about(CMD_ABOUT)
                        .settings(&[AppSettings::SubcommandRequiredElseHelp])

                        .arg(Arg::with_name("config")
                                .help("Path to the config to add a dependency to"))
                        .arg(Arg::with_name("global")
                                .short("g")
                                .long("global")
                                .help("Operate on a global config only"))
                        .arg(Arg::with_name("local")
                            .short("l")
                            .long("local")
                            .help("Operate on a local config only"))
                        .arg(Arg::with_name("verbose")
                                .short("v")
                                .long("verbose")
                                .help("Verbose mode"))
                        .subcommand(SubCommand::with_name("add")
                                        .arg(Arg::with_name("name").required(true))
                                        .arg(Arg::with_name("url").required(true))
                                        .about("Add a new `<ALIAS> => <INDEX_URL>`"))
                        .subcommand(SubCommand::with_name("rm")
                                        .arg(Arg::with_name("name").required(true))
                                        .about("Remove an existing <ALIAS>"))
                        .subcommand(SubCommand::with_name("rename")
                                        .arg(Arg::with_name("old").required(true))
                                        .arg(Arg::with_name("new").required(true))
                                        .about("Rename an existing <ALIAS>"))
                        .subcommand(SubCommand::with_name("get")
                                        .arg(Arg::with_name("name").required(true))
                                        .about("Get <INDEX_URL> by <ALIAS>"))
                        .subcommand(SubCommand::with_name("set")
                                        .arg(Arg::with_name("name").required(true))
                                        .arg(Arg::with_name("url").required(true))
                                        .about("Set a new <INDEX_URL> by <ALIAS>"))
                        .subcommand(SubCommand::with_name("list")
                                        .about("Print the current configuration"))
        ).get_matches();

    if let Some(args) = args.subcommand_matches("reg") {
        if let Err(err) = handle(args) {
            eprintln!("{} {}", style("error:").red(), err);
            for e in err.causes().skip(1) {
                eprintln!("{} {}", style("Caused by:").bold(), e);
            }

            process::exit(1);
        }
    } else {
        panic!("cargo-reg is intended to be invoked as a cargo subcommand");
    }
}
