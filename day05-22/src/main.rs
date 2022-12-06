use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use std::fmt;
use std::cmp::{min,max};

#[derive(Debug)]
struct Crate { label: char } 

impl fmt::Display for Crate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", &self.label)
  }
}

type Stack = Vec<Crate>;

#[derive(Debug)]
struct Cargo { stacks: Vec<Stack> }

impl Cargo {
  fn height(&self) -> usize {
    (&self).stacks.iter().fold(0,|acc,stack| max(acc,stack.len()))
  }
}

impl fmt::Display for Cargo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = "".to_string();
    for i in (0..(&self).height()).rev() {
      result = format!("{result}\n{}",&self.stacks.iter().fold("".to_string(), |acc, stack| format!("{acc} {}",if let Some(c) = stack.get(i) {c.to_string()} else {"   ".to_string()})));
    }
    let mut i = 0;
    write!(f,"{result}\n{}", &self.stacks.iter().fold(" ".to_string(),|acc,_| { i += 1; format!("{acc} {i}  ") }))
  }
}

#[derive(Debug)]
struct Move { count: i32, from: usize, to: usize }

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"move {} from {} to {}", &self.count, &self.from, &self.to)
  }
}

#[derive(Debug)]
struct Harbor { cargo: Cargo, moves: Vec<Move> }

impl fmt::Display for Harbor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n{}\n", &self.cargo.to_string(), &self.moves.iter().fold("".to_string(),|acc,mv| format!("{acc}\n{mv}")))
  }
}

fn prepare(lines: &Vec<String>) -> Harbor {
  let (cargo_lines, move_lines_option) = lines.iter().fold(
    (vec![], None::<Vec<String>>), 
    |(mut cargo,moves_option), line| 
      match moves_option {
        Some(mut moves) => {
          moves.push(line.to_string());
          (cargo, Some(moves))
        },
        None            => 
          match line.len() {
            0 => (cargo, Some(vec![])),
            _ => { 
              cargo.push(line.to_string()); 
              (cargo, moves_option) 
            }
          }
      }
  );

  let mut stacks: Vec<Stack> = (0..cargo_lines.last().unwrap().split_whitespace().last().unwrap().parse::<i32>().unwrap())
    .fold(vec![], |mut acc,_| { acc.push(vec![]); acc });

  (0..cargo_lines.len()-1).rev().fold(
    (), 
    |_, i| {
      let mut line = cargo_lines[i].to_string();
      for j in 0..stacks.len() {

        let rest = line.split_off(min(3,line.len()));
        let curr = line;
        line = rest;
        match curr.chars().nth(1) {
          Some(' ') => {},
          Some( c ) => stacks[j].push(Crate { label: c }),
          None      => panic!("Error when parsing stacks on {line}")
        }
        if line.len() > 0 { line.remove(0); }
      }
    }
  );

  let move_lines = if let Some(move_lines) = move_lines_option {move_lines} else {vec![]};
  let moves: Vec<Move> = move_lines.iter().map(
    |line| {
      let tokens: Vec<&str> = line.split_whitespace().collect();
      Move { 
        count: tokens[1].parse::<i32>().unwrap(), 
        from : tokens[3].parse::<usize>().unwrap(), 
        to   : tokens[5].parse::<usize>().unwrap()}
    }
  ).collect();  

  Harbor { cargo: Cargo { stacks: stacks }, moves: moves }  
}

fn one(input: &Input) -> String {
  let mut harbor = prepare(&input.lines);

  for Move { count, from, to } in harbor.moves.iter() {
    for _ in 0..*count {
      let c = harbor.cargo.stacks[*from-1].pop().unwrap();
      harbor.cargo.stacks[*to-1].push(c);
    }
  }

  harbor.cargo.stacks.iter().fold("".to_string(), |acc, stack| format!("{acc}{}", stack.last().unwrap().label))
}

fn two(input: &Input) -> String {
  let mut harbor = prepare(&input.lines);
  
  for Move { count, from, to } in harbor.moves.iter() {
    let mut crates = vec![];
    for _ in 0..*count {
      crates.push(harbor.cargo.stacks[*from-1].pop().unwrap());
    }
    crates = crates.into_iter().rev().collect();
    harbor.cargo.stacks[*to-1].append(&mut crates);
  }

  harbor.cargo.stacks.iter().fold("".to_string(), |acc, stack| format!("{acc}{}", stack.last().unwrap().label))
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
