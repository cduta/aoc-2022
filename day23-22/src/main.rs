use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Int = i64;
type Pos = (Int,Int);

enum Dir { N, S, W, E }

impl Dir {
  fn delta(&self) -> [Pos; 3] {
    match self {
      Dir::N => [(-1,-1),(0,-1),( 1,-1)],
      Dir::S => [(-1, 1),(0, 1),( 1, 1)],
      Dir::W => [(-1,-1),(-1,0),(-1, 1)],
      Dir::E => [( 1,-1),( 1,0),( 1, 1)]
    }
  }
  fn go(&self, (x,y): Pos) -> Pos {
    match self {
      Dir::N => (x,y-1),
      Dir::S => (x,y+1),
      Dir::W => (x-1,y),
      Dir::E => (x+1,y)
    }
  }
}

struct Elf { next: Vec<Dir> }

impl Elf {
  fn new() -> Elf { Elf { next: vec![Dir::N,Dir::S,Dir::W,Dir::E] } }

  fn propose(&mut self, pos@(x,y): Pos, occupied: &HashSet<Pos>) -> Option<Pos> {
    let (proposal_pos_option, has_neighbor) = self.next.iter().enumerate().fold(
      (None, false), 
      |(proposal_pos_option, has_neighbor), (i,dir)| {
        if dir.delta().iter().all(|(dx,dy)| { !occupied.contains(&(x+dx,y+dy)) }) {
          if proposal_pos_option.is_none() {
            (Some(i), has_neighbor)
          } else {
            (proposal_pos_option, has_neighbor)
          }
        } else {
          (proposal_pos_option, true)
        }
      }
    );
    return if has_neighbor {
      if let Some(proposal_pos) = proposal_pos_option {
        let proposal = self.next.remove(proposal_pos);
        self.next.push(proposal);
        Some(self.next.last().unwrap().go(pos))
      } else {
        let proposal = self.next.remove(0);
        self.next.push(proposal);
        None
      }
    } else {
      None
    }
  }
}

fn prepare(lines: &Vec<String>) -> HashMap<Pos, Elf> {
  let mut elves = HashMap::new();
  lines.iter().enumerate().for_each(
    |(y,line)| {
      line.chars().enumerate().for_each(
        |(x,c)| {
          match c {
            '.' => (),
            '#' => { elves.insert((x as Int,y as Int), Elf::new()); },
             _  => panic!("Malformed input")
          };
        }
      )
    }
  );
  return elves;
}

fn dims(elves: &HashMap<Pos,Elf>) -> (Int,Int,Int,Int) {
  (*elves.keys().map(|(x,_)| x).min_by(|x1,x2| x1.cmp(&x2)).unwrap(),  
   *elves.keys().map(|(x,_)| x).max_by(|x1,x2| x1.cmp(&x2)).unwrap(),
   *elves.keys().map(|(_,y)| y).min_by(|y1,y2| y1.cmp(&y2)).unwrap(),  
   *elves.keys().map(|(_,y)| y).max_by(|y1,y2| y1.cmp(&y2)).unwrap())
}

fn trace_board(elves: &HashMap<Pos,Elf>) {
  let (min_x,max_x,min_y,max_y) = dims(&elves);

  trace!("{}", (min_y..=max_y).fold(
    String::new(), 
    |acc, y| {
      format!("{acc}\n{}",
        (min_x..=max_x).fold( 
          String::new(),
          |acc, x| {
            format!("{acc}{}", 
              if elves.contains_key(&(x,y)) {
                '#'
              } else {
                '.'
              }
            )
          }
        )
      )
    }
  ));
}

fn one(input: &Input) -> String {
  let mut elves = prepare(&input.lines);
  let mut proposed = false;
  let mut proposals: HashMap<Pos, Pos> = HashMap::new();

  while !proposed || !proposals.is_empty() {
    proposals.clear();

    trace_board(&elves);

    let occupied: HashSet<Pos> = elves.keys().map(|pos| *pos).collect();
    let mut clash: HashSet<Pos> = HashSet::new();
    elves.iter_mut().for_each(
      |(pos, elf)| {
        let proposed_pos_option = elf.propose(*pos, &occupied); // Also moves proposals
        if let Some(proposed_pos) = proposed_pos_option {
          if !clash.contains(&proposed_pos) {
            if proposals.contains_key(&proposed_pos) {
              proposals.remove(&proposed_pos);
              clash.insert(proposed_pos);
            } else {
              proposals.insert(proposed_pos, *pos);
            }
          }
        }
      }
    );

    proposals.iter().for_each(
      |(to, from)| {
        let elf = elves.remove(&from).unwrap();
        elves.insert(*to, elf);
      }
    );

    proposed = true;
  }  

  trace_board(&elves);

  let (min_x,max_x,min_y,max_y) = dims(&elves);

  return (min_x.abs_diff(max_x+1)*min_y.abs_diff(max_y+1)).to_string();
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
