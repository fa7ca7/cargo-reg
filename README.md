# cargo-reg

This tool extends [cargo](http://doc.crates.io/) to allow you manage
alternative registries by modifying `.cargo/config` file from
the command line.

## Install

```sh
$ cargo install cargo-reg
```

## Usage

```plain
cargo-reg
This command allows you to manage alternative registries in .cargo/config file.

USAGE:
    cargo-reg [FLAGS] [SUBCOMMAND]

FLAGS:
    -g, --global     Operate on a global config
    -h, --help       Prints help information
    -l, --local      Operate on a local config only
    -V, --version    Prints version information
    -v, --verbose    Verbose mode.

SUBCOMMANDS:
    add       Add a new `<ALIAS> => <INDEX_URL>`
    get       Get <INDEX_URL> by <ALIAS>
    help      Prints this message or the help of the given subcommand(s)
    list      Print the current configuration
    rename    Rename an existing <ALIAS>
    rm        Remove an existing <ALIAS>
    set       Set a new <INDEX_URL> by <ALIAS>
```

## Example

```sh
$ cargo-reg --global add test1 https://test1-global.io
$ cargo-reg --global add test2 https://test2-global.io
$ cargo-reg --global list
test1 => https://test1-global.io
test2 => https://test2-global.io
$ cat ~/.cargo/config
[registries]
test1 = "https://test1-global.io"
test2 = "https://test2-global.io"
$
$ cd $SOME_DIR
$ cargo-reg add test1 https://test1-local.io
$ cargo-reg --local list
test1 => https://test1-local.io
$ cat $PWD/.cargo/config
[registries]
test1 = "https://test1-local.io"
$
$ cargo-reg list
test1 => https://test1-local.io
test2 => https://test2-global.io
```

## License
Licensed at your option under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)