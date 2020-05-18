#[macro_use]
extern crate clap;
mod args;
mod cfg;
mod dbg;
mod logger;
mod recipe;

use crate::cfg::Config;
use crate::dbg::dbg_info;
use crate::logger::setup_logging;
use fwalker::Walker;
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::borrow::BorrowMut;
use std::path::PathBuf;
use std::process;
use std::process::exit;

fn main() {
    let cfg: Config = Config::from_args(args::args());
    setup_logging(cfg.verbosity_level);

    if cfg.print_dbg {
        println!("{}", dbg_info());
        process::exit(0);
    }

    let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = cfg
        .paths
        .iter()
        .map(PathBuf::from)
        .inspect(check_path)
        .partition(|p| p.is_dir());

    let found_files: Vec<PathBuf> = dirs
        .iter()
        .flat_map(|path: &PathBuf| Walker::from(path).unwrap())
        .filter(accept_file_ext)
        .collect();

    let mut all_files: Vec<PathBuf> = [found_files, files].concat();
    let mut rand = StdRng::seed_from_u64(cfg.seed);
    all_files.shuffle(&mut rand);
    all_files.iter().take(cfg.limit).for_each(|f| println!("{:?}", f));
}

fn check_path(path: &PathBuf) {
    if !path.exists() {
        log::error!("Path does not exist: {:?}", path);
        process::exit(1);
    }
    if !path.is_dir() && !accept_file_ext(path) {
        log::error!("File does not have a supported file extension: {:?}", path);
        process::exit(2);
    }
}

const ACCEPTED_EXTENSIONS: [&str; 2] = ["md", "txt"];

fn accept_file_ext(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => {
            let ext: &str = &ext.to_str().unwrap_or("").to_lowercase();
            ACCEPTED_EXTENSIONS.contains(&ext)
        }
        None => false,
    }
}
