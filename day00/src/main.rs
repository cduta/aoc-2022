use common::args::Part;
use common::init::{Input, startup, shutdown};
use log::trace;

fn main() {
  let input: Input = startup();

  if input.verbose {
    trace!("Running {} (Part {}) with input:\n{}", env!("CARGO_PKG_NAME"), input.part, input.lines.join("\n"));
  }

  shutdown();
}
