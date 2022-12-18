use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashSet;
use std::time::Instant;

type Int = i64;
type Square = (Int,Int);
type Cube = (Int,Int,Int);

fn prepare(lines: &Vec<String>) -> HashSet<Cube> {
  lines.iter().map(
    |line| {
      let pos_list: Vec<Int> = line.split(',').map(|v| v.parse::<Int>().unwrap()).collect();
      assert_eq!(pos_list.len(), 3);
      (pos_list[0],pos_list[1],pos_list[2])
    }
  ).collect()
}

fn one(input: &Input) -> String {
  let cubes = prepare(&input.lines);
  let left:  HashSet<Square> = cubes.iter().map(|(_,y,z)| (*y,*z)).collect();
  let down:  HashSet<Square> = cubes.iter().map(|(x,_,z)| (*x,*z)).collect();
  let front: HashSet<Square> = cubes.iter().map(|(x,y,_)| (*x,*y)).collect();

  return (2*(left.len()+down.len()+front.len())).to_string();
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

  let start = Instant::now();
  print(day, input.part.clone(), f(&input));
  info!("Time elapsed: {} ms", start.elapsed().as_millis());

  shutdown();
}
