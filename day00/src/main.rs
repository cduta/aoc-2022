use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::trace;

fn one(_input: &Input) -> String {
  return "42".to_string();
}

fn two(_input: &Input) -> String {
  return "42".to_string();
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

  print(day, input.part.clone(), f(&input));

  shutdown();
}
