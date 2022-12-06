use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;

fn start(lines: &String, length: usize) -> i32 {
  let (_, pos) = lines.chars().enumerate().fold(
    (vec![], None),
    | (mut seq, mut pos), (i,c)| {
      //trace!("{seq:?},{pos:?}");
      match pos {
        None    => {
          if let Some(j) = seq.iter().position(|&_c| _c == c) {
            seq.push(c);
            seq = seq.split_off(j+1);
          } else {
            seq.push(c);
            pos = if seq.len() == length {Some(i+1)} else {pos};
          }
        },
        Some(_) => {}
      }
      (seq, pos)
    }
  );
  return pos.unwrap() as i32;
}

fn one(input: &Input) -> String {
  return start(&input.lines[0], 4).to_string();
}

fn two(input: &Input) -> String {
  return start(&input.lines[0], 14).to_string();
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
