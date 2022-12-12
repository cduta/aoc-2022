use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashMap;
use std::time::Instant;
use priority_queue::DoublePriorityQueue;

type Height = usize;

fn to_height(c: char) -> Height {
  match c {
    'S'                                           => to_height('a'),
    'E'                                           => to_height('z'),
     c if 97 <= c as Height && c as Height <= 122 => c as Height-97,
     _                                            => panic!("Malformed height character: {c}")
  }
}

struct HeightMap { map: Vec<Vec<Height>>, width: usize, height: usize, start: (usize,usize), end: (usize,usize) }

impl HeightMap {
  fn new(lines: &Vec<String>) -> HeightMap {
    let mut width  = 0;
    let     height = lines.len();
    let mut start: Option<(usize, usize)> = None;
    let mut end  : Option<(usize, usize)> = None;
    let map = lines.iter().enumerate().map(
      |(y,line)| {
        let row: Vec<Height> = line.chars().enumerate().map(
          |(x,c)| {
            match c { 
              'S' => start = Some((x,y)),
              'E' => end   = Some((x,y)),
               _  => ()
            }
            to_height(c)
          }
        ).collect();
        width = std::cmp::max(width,row.len());
        row
      }
    ).collect();
    if start.is_none() { panic!("No start specified") }
    if end.is_none()   { panic!("No end specified") }
    HeightMap { map: map, width: width, height: height, start: start.unwrap(), end: end.unwrap() }
  }

  fn height_at(&self, (x,y): &(usize,usize)) -> Height { (&self).map[*y][*x] }

  fn manhattan_distance(&self, here@(x1,y1): &(usize,usize), there@(x2,y2): &(usize,usize)) -> usize {
    x1.abs_diff(*x2) + y1.abs_diff(*y2) + (&self).height_at(here).abs_diff((&self).height_at(there))
  }
}

#[derive(Debug,Hash,Eq)]
struct Node { pos: (usize,usize), steps: u32 }

impl Node {
  fn new(pos: (usize,usize), steps: u32) -> Node { Node { pos: pos, steps: steps } }

  fn adjacent(&self, width: usize, height: usize) -> Vec<Node> {
    vec![(1,0),(0,1),(-1,0),(0,-1)].into_iter().map(
      |(dx,dy): (i32,i32)| {
        let (nx,ny) = ((&self).pos.0 as i32+dx, (&self).pos.1 as i32+dy);
        if 0 <= nx && nx < width as i32 && 0 <= ny && ny < height as i32 { 
          Some(Node { pos: (nx as usize, ny as usize), steps: (&self).steps+1 })
        } else {
          None
        }
      }
    ).filter_map(|node_option| node_option).collect()
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
      (&self).pos == other.pos
  }
}

fn one(input: &Input) -> String {
  let height_map = HeightMap::new(&input.lines);

  let mut unvisited = DoublePriorityQueue::new();
  unvisited.push(Node::new(height_map.start, 0), height_map.manhattan_distance(&height_map.start, &height_map.end));

  let mut visited = HashMap::new();

  while !unvisited.is_empty() && !visited.contains_key(&height_map.end) {
    let (curr,steps) = unvisited.pop_min().unwrap();
    visited.insert(curr.pos, curr.steps);
    curr.adjacent(height_map.width,height_map.height).into_iter().for_each(
      |next| {
        let curr_height = height_map.height_at(&curr.pos);
        let next_height = height_map.height_at(&next.pos);
        if !visited.contains_key(&next.pos) && (next_height == 0 || curr_height >= next_height-1) {
          let heuristic = steps+1;//= steps+height_map.manhattan_distance(&next.pos, &height_map.end)+1;
          unvisited.push(next, heuristic);
        }
      }
    );
  }  

  trace!("Visited:\n{visited:?}");

  trace!("How many steps from start {:?} to end {:?}: {}", 
           height_map.start, 
           height_map.end, 
           if let Some(steps) = visited.get(&height_map.end) {steps.to_string()} else {"Unreachable".to_string()});

  return if let Some(steps) = visited.get(&height_map.end) {steps.to_string()} else {"Unreachable".to_string()};
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
