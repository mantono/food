use clap::ArgMatches;
use std::str::FromStr;

pub struct Config {
    pub paths: Vec<String>,
    pub limit: usize,
    pub verbosity_level: u8,
    pub print_dbg: bool
}

impl Config {
    pub fn from_args(args: ArgMatches) -> Config {
        let limit: usize = args
            .value_of("limit")
            .unwrap_or(&std::u64::MAX.to_string())
            .parse()
            .unwrap();
        let paths: Vec<String> = args
            .values_of("path")
            .unwrap()
            .map(|v| v.to_string())
            .collect();
        let verbosity_level: u8 = args.value_of("verbosity").unwrap().parse::<u8>().unwrap();
        let print_dbg: bool = args.is_present("debug");

        Config {
            paths,
            limit,
            verbosity_level,
            print_dbg
        }
    }
}
