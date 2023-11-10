use clap::{arg, value_parser};
use std::path::PathBuf;
use filmette::run;

mod fs_utils;

fn main() {
    let cmd = clap::Command::new("filmette").bin_name("filmette")
      .arg(
        arg!(-i --input <FILE_PATH> "Path to movie file")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        arg!(-o --output <OUTPUT_PATH> "Path to save output")
          .required(true)
          .value_parser(value_parser!(PathBuf))
      );

    let matches = cmd.get_matches();
    let Some(input_path) = matches.get_one::<PathBuf>("input") else { panic!("No input file provided") };
    let Some(output_path) = matches.get_one::<PathBuf>("output") else { panic!("No output file provided") };
    
    let _ = run(input_path, output_path);
}
