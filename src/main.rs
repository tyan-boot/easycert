extern crate clap;

#[cfg(windows)]
extern crate gdi32;
#[cfg(windows)]
extern crate user32;

use std::fs::File;

use clap::{App, Arg, ArgMatches, SubCommand};

mod cert;
mod install;
mod utils;

use install::install;

fn main() {
    let args = App::new("mkcert")
        .version("0.1")
        .author("tyanboot <tyanboot@outlook.com>")
        .about("create cert quickly")
        .subcommand(
            SubCommand::with_name("init")
                .about("init root ca")
                .arg(
                    Arg::with_name("common name")
                        .long("cn")
                        .short("n")
                        .help("common name of cert")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("length")
                        .long("len")
                        .short("l")
                        .help("length of key, default 2048")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("force init, discard previous ca."),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("issue a new cert")
                .arg(
                    Arg::with_name("separate")
                        .long("separate")
                        .short("s")
                        .help("issue multiple cert into separate cert file (no SAN"),
                )
                .arg(Arg::with_name("names").multiple(true).required(true))
                .arg(
                    Arg::with_name("output")
                        .long("out")
                        .short("o")
                        .help("output file name of cert, without extension")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("length")
                        .long("length")
                        .short("l")
                        .help("lengh of key, default 2048")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("install").about("install ca to trusted store"));

    let get_init_args = |matches: &ArgMatches| {
        let length: u32 = matches
            .value_of("length")
            .unwrap_or("2048")
            .parse()
            .unwrap();
        let cn = matches.value_of("common name");

        let cn = match cn {
            Some(cn) => cn.to_string(),
            None => utils::hostname(),
        };

        let force = if matches.is_present("force") {
            true
        } else {
            false
        };

        return (cn, length, force);
    };

    let matches = args.get_matches();

    let mut c = cert::Cert::new();

    if let Some(matches) = matches.subcommand_matches("init") {
        let (cn, length, force) = get_init_args(&matches);

        c.init(&cn, length, force);
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        c.load_ca();

        if let Some(names) = matches.values_of("names") {
            let names: Vec<&str> = names.collect();

            let out_name = matches.value_of("output");

            let length: u32 = matches
                .value_of("length")
                .unwrap_or("2048")
                .parse()
                .unwrap();
            c.new_cert(names, length, out_name);
        } else {
            eprintln!("must provide a name!");
        }
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        let ca = File::open(utils::ca_path());
        let key = File::open(utils::key_path());

        if let (Ok(_ca), Ok(_key)) = (ca, key) {
        } else {
            let (cn, length, force) = get_init_args(&matches);

            c.init(&cn, length, force);
        }

        install();
    }
}
