use clap::{App, Arg, ArgMatches};

pub fn args<'a>() -> ArgMatches<'a> {
    let path = Arg::with_name("path")
        .default_value(".")
        .takes_value(true)
        .required(false)
        .multiple(true)
        .help("Paths to look for files in")
        .long_help("Select zero, one or several directories for which to look for files in. If no value is give, the application will default to current directory.");

    let limit = Arg::with_name("limit")
        .takes_value(true)
        .short("l")
        .long("limit")
        .help("Limit how many files to list")
        .long_help("Only list the first N files found given by this limit. If no value is set for this option, the application will not stop until it has gone through all files in the directory and subdirectories.");

    let verbosity = Arg::with_name("verbosity")
        .takes_value(true)
        .default_value("1")
        .validator(|n: String| {
            let range = 0u8..=5u8;
            let n: u8 = n.parse::<u8>().unwrap();
            if range.contains(&n) {
                Ok(())
            } else {
                Err("Invalid value".to_string())
            }
        })
        .short("v")
        .long("verbosity")
        .help("Set verbosity level, 0 - 5")
        .long_help("Set the verbosity level, from 0 (least amount of output) to 5 (most verbose). Note that logging level configured via RUST_LOG overrides this setting.");

    let debug = Arg::with_name("debug")
        .takes_value(false)
        .short("D")
        .long("debug")
        .help("Print debug information")
        .long_help("Print debug information about current build for binary, useful for when an issue is encountered and reported");

    let args: ArgMatches = App::new(crate_name!())
        .about("Command line tool for finding large files")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(path)
        .arg(limit)
        .arg(verbosity)
        .arg(debug)
        .get_matches();

    args
}
