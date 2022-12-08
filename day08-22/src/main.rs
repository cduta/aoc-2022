use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::hash_map::RandomState;
use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::max;

type Row    = Vec<u8>;
type Matrix = Vec<Row>;

fn prepare(lines: &Vec<String>) -> (Matrix,usize,usize) {
  let mut width  = 0;
  let height = lines.len();
  (lines.iter().fold(
    Matrix::new(), 
    |mut acc, line| {
      acc.push(line.chars().fold(
        Row::new(),
        |mut acc, c| {
          acc.push(c.to_string().parse::<u8>().unwrap());
          acc
        }
      ));
      width = max(width, line.len());
      acc
    }
  ), width, height)
}

fn one(input: &Input) -> String {
  let (trees, width, height) = prepare(&input.lines);
  let mut visible: HashSet<(usize,usize), RandomState> = HashSet::new();
  
  vec![(vec![0],      (0..height).collect()),
       (vec![width-1],(0..height).collect()),
       ((0..width).collect(),vec![0]       ),
       ((0..width).collect(),vec![height-1])].into_iter().for_each(|(start_xs, start_ys)| {
         if let [x] = start_xs[..] {
           start_ys.into_iter().for_each(|y| {
             let mut x_iter = if x == 0 { 
                                (0..width).collect::<Vec<usize>>().into_iter() 
                              } else { 
                                (0..width).rev().collect::<Vec<usize>>().into_iter() 
                              };
             let start = x_iter.next().unwrap();
             visible.insert((start,y));
             x_iter.fold(trees[y][start], |tree, i| {
              let curr = trees[y][i];
              if curr > tree {
                visible.insert((i,y));
                curr
              } else {
                tree 
              }
             });
           });
         } else if let [y] = start_ys[..] {
           start_xs.into_iter().for_each(|x| {
             let mut y_iter = if y == 0 { 
                                (0..height).collect::<Vec<usize>>().into_iter() 
                              } else { 
                                (0..height).rev().collect::<Vec<usize>>().into_iter() 
                              };
             let start = y_iter.next().unwrap();
             visible.insert((x,start));
             y_iter.fold(trees[start][x], |tree, i| {
              let curr = trees[i][x];
              if curr > tree {
                visible.insert((x,i));
                curr
              } else {
                tree 
              }
             });
           });
         }
       });
       
  return visible.len().to_string();
}

fn two(input: &Input) -> String {
  let (trees, width, height) = prepare(&input.lines);
  let mut scores: HashMap<(usize,usize), u64, RandomState> = HashMap::new();

  #[derive(PartialEq)]
  enum Dir {Horizontal, Vertical}

  fn view(trees: &Matrix, start_tree: u8, fix: usize, range: std::ops::Range<usize>, rev: bool, dir: Dir) -> u64 {
    let mut result = 0;
    for i in if rev {range.rev().collect::<Vec<usize>>()} else {range.collect::<Vec<usize>>()} {
      result += 1;
      if start_tree <= (if dir == Dir::Horizontal {trees[fix][i]} else {trees[i][fix]}) {
        break;
      }
    }
    return result;
  }

  (1..width-1).into_iter().for_each(|start_x|
    (1..height-1).into_iter().for_each(|start_y| {
      let start_tree = trees[start_y][start_x];
      let right = view(&trees, start_tree, start_y, start_x+1..width , false, Dir::Horizontal);
      let left  = view(&trees, start_tree, start_y, 0..start_x       , true , Dir::Horizontal);
      let down  = view(&trees, start_tree, start_x, start_y+1..height, false, Dir::Vertical  );
      let up    = view(&trees, start_tree, start_x, 0..start_y       , true , Dir::Vertical  );

      scores.insert((start_x+1,start_y+1), right*left*up*down);
    }
  ));

  let ((x,y),score) = scores.iter().max_by(|(_,s1),(_,s2)| s1.cmp(s2)).unwrap(); 
  trace!("Best position is at ({x},{y}) with a score of {score}");

  return score.to_string();
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
