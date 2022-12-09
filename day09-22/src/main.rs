use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;
use std::str::FromStr;

#[derive(Debug)]
enum Direction { Up, Down, Left, Right }

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match &self {
      Direction::Up    => "U", 
      Direction::Down  => "D",
      Direction::Left  => "L",
      Direction::Right => "R"
    })
  }
}

impl Direction {
  fn mv(&self, (x,y): Position) -> Position {
    match &self {
      Direction::Up    => (x,y+1), 
      Direction::Down  => (x,y-1),
      Direction::Left  => (x-1,y),
      Direction::Right => (x+1,y)
    }
  }
}

impl FromStr for Direction {
  type Err = ();

  fn from_str(input: &str) -> Result<Direction, Self::Err> {
      match input {
          "U" => Ok(Direction::Up),
          "D" => Ok(Direction::Down),
          "L" => Ok(Direction::Left),
          "R" => Ok(Direction::Right),
          _   => Err(()),
      }
  }
}

#[derive(Debug)]
struct Move { dir: Direction, steps: i32 }

impl Display for Move {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", &self.dir, &self.steps)
  }
}

type Position = (i32, i32);

fn pull((hx,hy): Position, (tx,ty): Position) -> Position {
  let mut result = (tx,ty);
  let (dx,dy) = (hx-tx,hy-ty);
  if dx.abs() > 1 {
    result = (tx+dx.signum(),ty+dy.signum());
  } else if dy.abs() > 1 {
    result = (tx+dx.signum(),ty+dy.signum());
  }
  return result;
}

#[derive(Debug)]
struct Rope { head: Position, tails: Vec<Position> }

fn prepare(lines: &Vec<String>) -> Vec<Move> {
  lines.iter().map(|line| { 
    if let [dir_string, steps_string] = line.split_whitespace().collect::<Vec<&str>>()[..] {
      Move { dir: dir_string.parse().unwrap(), steps: steps_string.parse().unwrap() }
    } else {
      panic!("Failed parsing move: {line}")
    }
  }).collect()
}


fn move_rope(moves: Vec<Move>, length: usize) -> HashSet<Position> {
  let mut visited: HashSet<Position> = HashSet::new();

  moves.into_iter().fold(
    Rope { head: (0,0), tails: std::iter::repeat((0,0)).take(length-1).collect() },
    | rope, Move { dir, steps } | {
      (0..steps).fold( 
        rope, 
        | Rope { head, mut tails }, _ | {
          let next_head = dir.mv(head);
          let next_tail = tails.iter_mut().enumerate().fold(
            next_head,
            |prev, (_, curr)| {
              *curr = pull(prev, *curr);
              *curr
            } 
          );
          visited.insert(next_tail);
          Rope { head: next_head, tails: tails }
        }
      )
    }
  );

  return visited;
}

fn one(input: &Input) -> String {
  return move_rope(prepare(&input.lines), 2).len().to_string();
}

fn two(input: &Input) -> String {
  return move_rope(prepare(&input.lines), 10).len().to_string();
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
