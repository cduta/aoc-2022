use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

type Int = usize;
type Point = (Int,Int);

enum Push { Left, Right }

impl Push {
  fn apply(&self, x: Int, piece_width: Int, board_width: Int) -> Int {
    match self {
      Push::Left  if x > 0                       => x-1,
      Push::Right if x+piece_width < board_width => x+1,
      _                                          => x
    }
  }

  fn push_no_collision(&self, x: Int, piece_width: Int, board_width: Int, push: Int, push_count: usize) -> (Int, Int) {
    (self.apply(x, piece_width, board_width), (push+1)%push_count)
  }

  fn push(&self, pos@(x,_): (Int,Int), piece: &Piece, board: &Board, push: Int, push_count: usize) -> (Int, Int) {
    (match self {
       Push::Left  if !board.left_collision(pos, piece)  => x-1,
       Push::Right if !board.right_collision(pos, piece) => x+1,
       _                                                 => x
     }, (push+1)%push_count)
  }

}

impl Display for Push {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", 
      match self {
        Push::Left  => '<', 
        Push::Right => '>'  
      }
    )
  }
}

#[derive(Debug,Clone)]
struct Piece { parts: HashSet<Point>, left: Vec<Int>, right: Vec<Int>, bottom: Vec<Int> }

impl Piece {
  fn new(parts: HashSet<Point>) -> Piece {
    let width  = parts.iter().max_by(|(x1,_), (x2,_)| x1.cmp(x2)).unwrap().0+1;
    let height = parts.iter().max_by(|(_,y1), (_,y2)| y1.cmp(y2)).unwrap().1+1;

    let (left,right) = (0..height).into_iter().fold(
      (vec![],vec![]),
      |(mut left, mut right), i| {
         left.push(parts.iter().filter(|(_,y)| *y == i).min_by(|(x1,_), (x2,_)| x1.cmp(x2)).unwrap().0);
        right.push(parts.iter().filter(|(_,y)| *y == i).max_by(|(x1,_), (x2,_)| x1.cmp(x2)).unwrap().0);
        (left, right)
      }
    );
    let bottom = (0..width).into_iter().fold(
      vec![],
      |mut bottom, i| {
        bottom.push(parts.iter().filter(|(x,_)| *x == i).min_by(|(_,y1), (_,y2)| y1.cmp(y2)).unwrap().1);
        bottom
      }
    );

    return Piece { parts, left, right, bottom };
  }

  fn width(&self)  -> Int { self.bottom.len() as Int }
  fn height(&self) -> Int { self.left.len() as Int }
}


fn prepare(lines: &Vec<String>) -> Vec<Push> {
  assert_eq!(lines.len(), 1);
  return lines[0].chars().map(
    |c| {
      match c {
        '<' => Push::Left,
        '>' => Push::Right,
         _  => panic!("Read something other than `<` or `>` in input")
      }
    }
  ).collect();
}

fn pieces() -> Vec<Piece> {
  vec![
    Piece::new(vec![(0,0),(1,0),(2,0),(3,0)].into_iter().collect()),
    Piece::new(vec![(1,0),(0,1),(1,1),(2,1),(1,2)].into_iter().collect()),
    Piece::new(vec![(0,0),(1,0),(2,0),(2,1),(2,2)].into_iter().collect()),
    Piece::new(vec![(0,0),(0,1),(0,2),(0,3)].into_iter().collect()),
    Piece::new(vec![(0,0),(1,0),(0,1),(1,1)].into_iter().collect())
  ]
}

struct Board { pieces: Vec<Vec<bool>>, width: Int } 

impl Board {
  fn new(width: Int) -> Board { Board { pieces: vec![], width: width } }
  fn height(&self) -> Int { self.pieces.len() as Int }
  fn collision(&self, x: Int, y: Int) -> bool { y < self.pieces.len() && x < self.width && self.pieces[y][x] }

  fn down_collision(&self, (x,y): (Int,Int), piece: &Piece) -> bool {
    if y == 0 { return true; }
    return !piece.bottom.iter().enumerate().all(|(dx,dy)| !self.collision(x+dx,y+dy-1));
  }

  fn left_collision(&self, (x,y): (Int,Int), piece: &Piece) -> bool {
    if x == 0 { return true; }
    return !piece.left.iter().enumerate().all(|(dy,dx)| !self.collision(x+dx-1,y+dy));
  }

  fn right_collision(&self, (x,y): (Int,Int), piece: &Piece) -> bool {
    if x+piece.width() == self.width { return true; }
    return !piece.right.iter().enumerate().all(|(dy,dx)| !self.collision(x+dx+1,y+dy));
  }

  fn place_at(&mut self, (x,y): (Int,Int)) {
    assert!(y < self.pieces.len());
    assert!(x < self.pieces[y].len());
    self.pieces[y][x] = true;
  }

  fn place_piece(&mut self, (x,y): (Int,Int), piece: &Piece) {
    while self.height() < y + piece.height() {
      self.pieces.push(vec![false; self.width]);
    }

    piece.parts.iter().for_each(
      |(dx,dy)| {
        self.place_at((x+dx,y+dy));
      }
    );
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\n{}", self.pieces.iter().fold(
      String::new(),
      |acc, line| {
        format!("{}\n{acc}", line.iter().fold(
          String::new(), 
          |acc, b| {
            format!("{acc}{}", if *b {'#'} else {'.'})
          }
        ))
      }
    ))
  }
}

fn run(pushes: &Vec<Push>, pieces: &Vec<Piece>, pieces_required: usize) -> Board {
  let (spawn_x, spawn_height) = (2,3);
  let mut board = Board::new(7);
  let mut p: usize = 0;
  let mut push = 0;

  while p < pieces_required {
    let piece = &pieces[p%pieces.len()];
    let (mut x, mut y) = (spawn_x, board.height() + spawn_height);

    // First steps without collision
    while y > board.height() {
      (x, push) = pushes[push].push_no_collision(x,piece.width(),board.width,push,pushes.len());
      y -= 1; 
    }
    (x, push) = pushes[push].push_no_collision(x,piece.width(),board.width,push,pushes.len());

    // Now, repeat down and push until a collision is ahead
    while !board.down_collision((x,y), piece) {
      y -= 1;
      (x, push) = pushes[push].push((x,y),piece,&board,push,pushes.len());
    }

    // Place piece
    board.place_piece((x,y), piece);
    p += 1;

    //trace!("{board}");
  }

  return board;
}

fn one(input: &Input) -> String {
  let pushes = prepare(&input.lines);
  let pieces = pieces();
  let board  = run(&pushes, &pieces, 2022);

  return board.height().to_string();
}

fn two(input: &Input) -> String {
  // let pushes = prepare(&input.lines);
  // let pieces = pieces();
  // let board  = run(&pushes, &pieces, 1000000000000);

  // return board.height().to_string();

  /*
    Idea: Encode falling piece (.fffffff........) and (.........bbbbbbb) in a u16.
          Find cycle, by taking each decimal representation (00-63) dc = bbbbbbb to represent board as:
            dc1
            dc2
            dc3
            ...
          Take each dci, extract di and ci up to n (dynamic?) and concatenate them (c = 1*c1 + 10*c2 + 100*c3 + ... and d = 1*d1 + 10*d2 + 100*d3 + ...)
          Then, compare hash(c) and hash(d) with previous hash values of the same length. 
          If there is a match, this is the cycle. 
          Fast forward to the last cycle and continue till finished.
  */
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
