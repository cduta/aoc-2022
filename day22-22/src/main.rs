use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

type Pos  = (usize,usize);
type Dims = (Pos,Pos);

enum Direction { N, S, E, W }

struct Transition { to: usize, side: Direction, range: (usize,usize) }

struct Room { id: usize, dimensions: Dims, transitions: Vec<Transition>, walls: HashSet<Dims> }

enum Instruction { L, R, Walk(usize) }

struct State { position: Pos, facing: Direction }

fn prepare(lines: &Vec<String>) { //-> (Vec<Room>, VecDeque<Instruction>, State) {
  let rest: Vec<&[String]> = lines.split(|line| line.is_empty()).collect();
  assert_eq!(rest.len(),2);
  let (map_strings, inst_string) = (rest[0], rest[1]);
  //let mut state;

  assert_eq!(inst_string.len(), 1);
  let mut instructions: VecDeque<Instruction> = VecDeque::new();
  assert!(inst_string[0].chars().fold(
    String::new(),
    |mut acc, c| {
      match c {
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => acc.push(c),
        'L'                                     => { 
          instructions.push_back(Instruction::Walk(acc.parse::<usize>().unwrap())); 
          instructions.push_back(Instruction::L); 
          acc.clear();
        },
        'R'                                     => { 
          instructions.push_back(Instruction::Walk(acc.parse::<usize>().unwrap())); 
          instructions.push_back(Instruction::R); 
          acc.clear();
        },
         _                                      => panic!("Malformed instruction input")
      }
      acc
    }
  ).is_empty());

  //return (..., instructions, state)
}

fn one(input: &Input) -> String {
  //let (rooms, instructions, mut state) = prepare(&input.lines);

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

  let start = Instant::now();
  print(day, input.part.clone(), f(&input));
  info!("Time elapsed: {} ms", start.elapsed().as_millis());

  shutdown();
}
