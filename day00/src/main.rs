use common::args::Args;
use common::logger::initialize;
use log::{trace,info};

fn main() {
  let args: Args = Args::populate();
  let result = initialize(args.log.clone());
  if result.is_err() {
    panic!("Preparation failed!");
  }
  trace!("Begin logging for {}", env!("CARGO_PKG_NAME"));
  trace!("Parsing input...");

  info!("Input: {} and Part: {}", args.input, args.part);
}
