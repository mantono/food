use chrono::Datelike;
use clap::ArgMatches;
use rand::Rng;
use std::str::FromStr;

pub struct Config {
    pub paths: Vec<String>,
    pub limit: usize,
    pub seed: u64,
    pub verbosity_level: u8,
    pub print_dbg: bool,
}

impl Config {
    pub fn from_args(args: ArgMatches) -> Config {
        let limit: usize = args.value_of("limit").unwrap_or(&std::u64::MAX.to_string()).parse().unwrap();
        let paths: Vec<String> = args.values_of("path").unwrap().map(|v| v.to_string()).collect();
        let default_seed: String = gen_seed().to_string();
        let seed: u64 = args.value_of("seed").unwrap_or(&default_seed).parse().unwrap();
        let verbosity_level: u8 = args.value_of("verbosity").unwrap().parse::<u8>().unwrap();
        let print_dbg: bool = args.is_present("debug");

        Config {
            paths,
            limit,
            seed,
            verbosity_level,
            print_dbg,
        }
    }
}

fn gen_seed() -> u64 {
    chrono::Utc::now().num_days_from_ce() as u64
}
