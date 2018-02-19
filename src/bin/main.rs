extern crate toml_edit;
extern crate url;
extern crate clap;
extern crate console;
#[macro_use] extern crate failure;
extern crate cargo_reg;

use std::process;

use failure::Error;
use url::Url;
use clap::{App, Arg, SubCommand};
use console::style;

use cargo_reg::config::Config;
use cargo_reg::config_location::{ConfigLocation, locate_config, open_config, write_config};


fn handle(args: clap::ArgMatches) -> Result<(), Error> {
    let verbose = args.is_present("verbose");
    let global = args.is_present("global");
    let local = args.is_present("local");

    let config_location = ConfigLocation::new(global, local)?;
    let location = locate_config(&config_location)?;
    let (data, mut file) = open_config(&location)?;
    let mut config = Config::new(data)?;

    if verbose {
        println!("{} {} {}",
                 style("==>").green(),
                 style("Config:").bold(),
                 location.display());
    }

    match args.subcommand() {
        ("add", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = sub.value_of("url").unwrap();
            check_url(&url)?;

            config.add(name, url)?;
            if verbose {
                println!("{} {}\n{} => {}",
                         style("==>").green(),
                         style("Registry added:").bold(),
                         &name, &url);
            }
        }

        ("rm", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = config.remove(name)?;
            if verbose {
                println!("{} {}\n{} => {}",
                         style("==>").green(),
                         style("Registry removed:").bold(),
                         &name, &url);
            }
        }

        ("rename", Some(sub)) => {
            let old = sub.value_of("old").unwrap();
            let new = sub.value_of("new").unwrap();

            config.rename(old, new)?;
            if verbose {
                println!("{} {}\n{} => {}",
                         style("==>").green(),
                         style("Registry renamed:").bold(),
                         old, new);
            }
        }

        ("get", Some(sub)) => {
            println!("{}", config.get(sub.value_of("name").unwrap())?);
        }

        ("set", Some(sub)) => {
            let name = sub.value_of("name").unwrap();
            let url = sub.value_of("url").unwrap();
            check_url(&url)?;

            let old = config.set(name, url)?;
            if verbose {
                println!("{} {}\n{}: {{ {} => {} }}",
                         style("==>").green(),
                         style("Registry updated:").bold(),
                         &name, &old, &url);
            }
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
                println!("{} {}",
                         style("==>").green(),
                         style("Registries' list is empty").bold());
            }

            return Ok(());
        }

        (some, Some(_)) => {
            return Err(format_err!("wrong subcommand: {}", some));
        }

        (some, None) => {
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
    let args = App::new("cargo-reg")
        .about("This command allows you to manage alternative registries in .cargo/config file")
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
        .get_matches();

    if let Err(err) = handle(args) {
        eprintln!("{} {} {}",
                  style("==>").green(),
                  style("Command failed due to unhandled error:").red(),
                  err);
        for e in err.causes().skip(1) {
            eprintln!("{} {} {}",
                      style("==>").green(),
                      style("Caused by:").bold(),
                      e);
        }

        process::exit(1);
    }
}
