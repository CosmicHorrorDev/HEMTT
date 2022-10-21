use std::path::{Path, PathBuf};

use clap::{arg, value_parser, ArgMatches, Command};
use hemtt_error::AppError;
use hemtt_preprocessor::{preprocess_file, LocalResolver, Processed};

pub fn cli() -> Command {
    Command::new("preprocessor")
        .about("Run the preprocessor on files")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run the preprocessor")
                .arg_required_else_help(true)
                .arg(
                    arg!(<source> "Path to preprocess")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(<dest> "Path to destination")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), AppError> {
    match matches.subcommand() {
        Some(("run", matches)) => run(
            matches.get_one::<PathBuf>("source").unwrap(),
            matches.get_one::<PathBuf>("dest").unwrap(),
        ),
        _ => unreachable!(),
    }
}

fn run(source: &Path, dest: &Path) -> Result<(), AppError> {
    if !source.exists() {
        panic!("Source file does not exist");
    }
    if dest.exists() {
        panic!("Destination file already exists");
    }
    let mut resolver = LocalResolver::new();
    let tokens = preprocess_file(source, &mut resolver)?;
    let processed = Processed::from(tokens);
    std::fs::write(dest, processed.output()).unwrap();
    Ok(())
}
