use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use std::fmt;
use common::helper::{from_strings};

struct Range {
  from: i32,
  to  : i32
}

impl fmt::Display for Range {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}-{}",&self.from, &self.to)
  }
}

struct Assignment {
  ranges: Vec<Range>
}

impl fmt::Display for Assignment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", &self.ranges[0], &self.ranges[1..].iter().fold("".to_string(), |acc,range| format!("{acc},{range}")))
  }
}

fn prepare(lines: &Vec<String>) -> Vec<Assignment> {
  lines.iter().map(
    |line|
      Assignment { 
        ranges: line.split(',').map(
                  |assignment_string| {
                    let bounds: Vec<i32> = from_strings(assignment_string.split('-').map(|str| str.to_string()).collect());
                    Range {from: bounds[0], to: bounds[1]}
                  }
                ).collect()
      }
  ).collect()
}

fn one(input: &Input) -> String {
  prepare(&input.lines).iter().fold(
    0,
    |acc, assignment| acc 
                       + 
                      ((assignment.ranges[0].from <= assignment.ranges[1].from && assignment.ranges[0].to >= assignment.ranges[1].to ||
                        assignment.ranges[1].from <= assignment.ranges[0].from && assignment.ranges[1].to >= assignment.ranges[0].to) as i32) 
  ).to_string()
}

fn two(input: &Input) -> String {
  (input.lines.len() as i32 - prepare(&input.lines).iter().fold(
    0,
    |acc, assignment| acc 
                       + 
                      ((assignment.ranges[0].to < assignment.ranges[1].from || assignment.ranges[0].from > assignment.ranges[1].to) as i32) 
  )).to_string()
}

fn main() {
  let day  : String = env!("CARGO_PKG_NAME").to_string();
  let input: Input = startup();

  if input.verbose {
    trace!("Running {} (Part {}) with input:\n{}", env!("CARGO_PKG_NAME"), input.part, input.lines.join("\n"));
  }

  let f = match input.part {
    Part::One => one,
    Part::Two => two
  };

  let start = Instant::now();
  print(day, input.part.clone(), f(&input));
  info!("Time elapsed: {} ms", start.elapsed().as_millis());

  shutdown();
}
