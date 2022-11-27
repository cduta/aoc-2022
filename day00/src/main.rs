use common::args::{Args,Part};
use common::logger::initialize;
use common::reader::from_file;
use log::trace;

pub struct Input {
  part : Part,
  lines: Vec<String>
}

fn main() {
  let args   = Args::populate();
  let result = initialize(args.log);

  if result.is_err() {
    panic!("Preparation failed!");
  }
  trace!("Start logging for {}", env!("CARGO_PKG_NAME"));
  trace!("Parsing input from {}...", &args.input);

  let lines: Vec<String> = from_file(args.input);

  let mut content: Vec<String> = Vec::new();
  lines.into_iter().for_each(|l| content.push(l));
  trace!("Print input:\n{}", content.join("\n"));
}
