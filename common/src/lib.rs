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
