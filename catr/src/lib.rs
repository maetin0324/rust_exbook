use std::error::Error;
use clap::{Arg, App};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
  for filename in config.files {
    match open(&filename) {
      Err(err) => eprintln!("Failed to open{}: {}", filename, err),
      Ok(_) => {
        let mut line_number = 1;
        for line in BufReader::new(File::open(&filename)?).lines() {
          let line = line?;
          if config.number_nonblank_lines && line.trim().is_empty() {
            println!("{}", line);
          } else if config.number_lines {
            println!("{} {}", line_number, line);
            line_number += 1;
          } else {
            println!("{}", line);
          }
        }
      }
    }
  }
  Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("maetin")
    .about("Rust cat")
    .arg(Arg::with_name("files")
      .value_name("FILE")
      .help("Files to read")
      .multiple(true)
      .default_value("-")
    )
    .arg(Arg::with_name("number_lines")
      .short("n")
      .long("number")
      .help("Number the output lines, starting at 1")
      .takes_value(false)
      .conflicts_with("number_nonblank_lines")
    )
    .arg(Arg::with_name("number_nonblank_lines")
      .short("b")
      .long("number-nonblank")
      .help("Number the output lines, starting at 1, omitting blank lines")
      .takes_value(false)
    )
    .get_matches();

  if matches.is_present("number_lines") && matches.is_present("number_nonblank_lines") {
    return Err("The argument '--number-non-blank' cannot be used with '--number'".into());
  }

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    number_lines: matches.is_present("number_lines"),
    number_nonblank_lines: matches.is_present("number_nonblank_lines"),
  })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}
