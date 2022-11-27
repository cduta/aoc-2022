#[derive(Debug)]
pub struct Input {
  pub verbose: bool,
  pub part   : args::Part,
  pub lines  : Vec<String>
}

pub mod args {
  use std::fmt;
  use clap::Parser;  

  #[derive(clap::ValueEnum, Clone, Debug)]
  pub enum Part { One, Two }

  impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
          Part::One => write!(f, "One"),
          Part::Two => write!(f, "Two")
        }
    }
  }

  #[derive(Parser)]
  pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long, default_value = "one")]
    pub part: Part,

    #[arg(short, long, default_value = "log-config.yml")]
    pub log: String
  }

  impl Args {
      pub fn populate() -> Args {
          Args::parse()
      }
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn display_part_one() {
          assert_eq!(Part::One.to_string(), "One");
      }

      #[test]
      fn display_part_two() {
          assert_eq!(Part::Two.to_string(), "Two");
      }
  }
}

pub mod logger {
  use log4rs;
  use anyhow;

  pub fn initialize(log: String) -> Result<(),anyhow::Error> {
    log4rs::init_file(&log, Default::default()).or_else(
      |err| {
        println!("Failed to read {}: {}", &log, err.to_string());
        Err(err)})
  }
}

pub mod reader {
  use std::fs::File;
  use std::io::{BufReader,BufRead};

  pub fn from_file(file: String) -> Result<Vec<String>,std::io::Error> {
    let file_handle = File::open(&file);
    if let Err(err) = file_handle {
      return Err(err);
    }
    let lines :Vec<Result<String,std::io::Error>> = BufReader::new(file_handle.unwrap()).lines().collect();

    if let Some(Err(err)) = lines.iter().find(|l| l.is_err()) {
      return Err(std::io::Error::from(err.kind()));
    }

    Ok(lines.into_iter().map(|l| l.unwrap()).collect())
  }
}

pub mod init {
  use super::args::{Args,Part};
  use super::logger::initialize;
  use super::reader::from_file;
  use log::{trace,info};  

  pub fn startup() -> super::Input {
    let args              = Args::populate();
    let initialize_result = initialize(args.log.clone());
    if initialize_result.is_err() {
      panic!("Preparation failed!");
    }

    trace!("Loaded log config from {}", args.log);
    trace!("Start logging");
    trace!("Parsing input from `{}`", &args.input);

    let lines_result: Result<Vec<String>,std::io::Error> = from_file(args.input);
    if lines_result.is_err() {
      panic!("Parsing failed: {}", lines_result.unwrap_err());
    }

    let mut lines: Vec<String> = Vec::new();
    lines_result.unwrap().into_iter().for_each(|l| lines.push(l));

    super::Input { verbose: args.verbose, part: args.part, lines: lines }
  }

  pub fn print(day: String, part: Part, result: String) {
    info!("The result of {} (Part {}) is: {}", day, part, result);
  }

  pub fn shutdown() {
    trace!("Shutting down");
  }
}