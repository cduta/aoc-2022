/// The input required to solve a day at AoC
#[derive(Debug)]
pub struct Input {
  /// Print verbose information
  pub verbose: bool,
  /// Which part of the day to solve
  pub part   : args::Part,
  /// The input file as a vector of lines of strings
  pub lines  : Vec<String>
}

pub mod helper {
  use std::str::FromStr;
  use std::fmt::{Debug,Display};

  /// Takes a string vector and parses each element into T
  /// 
  /// Example:
  /// ```
  /// # use common::helper::from_strings;
  /// assert_eq!(from_strings::<i32>(vec!["1".to_string(),"2".to_string(),"-3".to_string()]), vec![1,2,-3]);
  /// let empty: Vec<i32> = vec![];
  /// assert_eq!(from_strings::<i32>(vec![]), empty);
  /// ```
  pub fn from_strings<T>(strings: Vec<String>) -> Vec<T> 
  where T: FromStr,
        <T as FromStr>::Err: Debug,
        <T as FromStr>::Err: Display {
    let depths_result: Vec<Result<T,<T as FromStr>::Err>> = strings.iter().map(|d| d.parse::<T>()).collect();
  
    if let Some(Err(err)) = depths_result.iter().find(|l| l.is_err()) {
      panic!("Failed to parse all lines of input to {}: {err}", std::any::type_name::<T>());
    }
    
    depths_result.into_iter().map(|d| d.unwrap()).collect()
  }
}

pub mod args {
  use std::fmt;
  use clap::Parser;  

  /// Which part of the AoC day
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
    /// The input file path
    #[arg(short, long)]
    pub input: String,

    /// Print verbose information, if true
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Which part of the AoC to run
    #[arg(short, long, default_value = "one")]
    pub part: Part,

    /// Which logger config file to use
    #[arg(short, long, default_value = "log-config.yml")]
    pub log: String
  }

  /// Parses the command-line arguments, then populates and returns the Args struct
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

  /// Initialize log4rs from file_path
  pub fn initialize(file_path: String) -> Result<(),anyhow::Error> {
    log4rs::init_file(&file_path, Default::default()).or_else(
      |err| {
        println!("Failed to read {}: {}", &file_path, err.to_string());
        Err(err)})
  }
}

pub mod reader {
  use std::fs::File;
  use std::io::{BufReader,BufRead};

  /// Read the input from file at file_path line by line, then add each line to a vector in order and return it
  pub fn from_file(file_path: String) -> Result<Vec<String>,std::io::Error> {
    let file_handle = File::open(&file_path);
    if let Err(err) = file_handle {
      return Err(err);
    }
    let lines_result :Vec<Result<String,std::io::Error>> = BufReader::new(file_handle.unwrap()).lines().collect();

    if let Some(Err(err)) = lines_result.iter().find(|l| l.is_err()) {
      return Err(std::io::Error::from(err.kind()));
    }

    Ok(lines_result.into_iter().map(|l| l.unwrap()).collect())
  }
}

pub mod init {
  use super::args::{Args,Part};
  use super::logger::initialize;
  use super::reader::from_file;
  use log::{trace,info};  
  use std::fmt::Display;

  /// Parse command-line arguments, 
  /// initialize log4rs, and 
  /// populate the Input struct and return it
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

  /// Print the result of the part of a day
  pub fn print<T>(day: String, part: Part, result: T) 
  where T: Display {
    info!("The result of {} (Part {}) is: {}", day, part, result);
  }

  /// Shut down operations
  pub fn shutdown() {
    trace!("Shutting down");
  }
}