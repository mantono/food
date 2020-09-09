#[macro_use]
extern crate clap;
extern crate lazy_static;
mod args;
mod cfg;
mod dbg;
mod logger;
mod qty;
mod recipe;

use crate::cfg::Config;
use crate::dbg::dbg_info;
use crate::logger::setup_logging;
use crate::recipe::{divide_unit, merge, Recipe};
use fwalker::Walker;
use lazy_static::lazy_static;
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use regex::Regex;
use std::path::PathBuf;
use std::process;

lazy_static! {
    pub static ref ITEM_PATTERN: Regex = Regex::new(r"^\s*-\s+").unwrap();
}

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
        .filter(|f: &PathBuf| !f.ends_with("README.md"))
        .collect();

    let mut all_files: Vec<PathBuf> = [found_files, files].concat();
    let mut rand = StdRng::seed_from_u64(cfg.seed);
    all_files.shuffle(&mut rand);

    let recipes: Vec<Recipe> = select_recipes(all_files, cfg.limit, cfg.simple);
    let output = merge(recipes);

    output
        .iter()
        .map(divide_unit)
        .for_each(|i| println!("{}", i))
}

fn select_recipes(mut files: Vec<PathBuf>, limit: usize, only_simple: bool) -> Vec<Recipe> {
    if only_simple {
        let recipes: Vec<Recipe> = files
            .iter()
            .filter_map(|f| Recipe::from_file(f.to_path_buf()))
            .inspect(|f| println!("{} => {}", f.title, f.size()))
            .collect();

        let sizes: Vec<usize> = recipes.iter().map(|r: &Recipe| r.size()).collect();
        let median_ingredients: usize = median(&sizes);
        log::debug!("Will parition on median size: {}", median_ingredients);

        let (under, over): (Vec<_>, Vec<_>) = recipes
            .iter()
            .partition(|r: &&Recipe| r.size() <= median_ingredients);

        let recipes = [&under[..], &over[..]].concat();

        recipes
            .iter()
            .take(limit)
            .inspect(|f| println!("{}", f))
            .map(|r| r.to_owned().to_owned())
            .collect()
    } else {
        files
            .iter_mut()
            .take(limit)
            .filter_map(|f| Recipe::from_file(f.to_path_buf()))
            .inspect(|f| println!("{}", f))
            .collect()
    }
}

fn median(list: &[usize]) -> usize {
    let len: usize = list.len();
    let mid_index: usize = len / 2;
    if len % 2 == 0 {
        let range = (mid_index - 1)..(mid_index + 1);
        mean(&list[range])
    } else {
        list[mid_index]
    }
}

fn mean(list: &[usize]) -> usize {
    let sum: usize = Iterator::sum(list.iter());
    sum / list.len()
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
