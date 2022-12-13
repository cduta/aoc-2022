use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::{HashMap, HashSet};
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
    let mut start: Option<Node> = None;
    let mut end  : Option<Node> = None;
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

type Node = (usize,usize);

fn adjacent(curr: Node, height_map: &HeightMap) -> Vec<Node> {
  vec![(1,0),(0,1),(-1,0),(0,-1)].into_iter().map(
    |(dx,dy): (i32,i32)| {
      let (nx,ny) = (curr.0 as i32+dx, curr.1 as i32+dy);
      if 0 <= nx && nx < height_map.width as i32 && 0 <= ny && ny < height_map.height as i32 { 
        let (curr_height, next_height) = (height_map.height_at(&curr), height_map.height_at(&(nx as usize, ny as usize)));
        if next_height == 0 || curr_height >= next_height-1 {
          Some((nx as usize, ny as usize))
        } else {
          None
        }
      } else {
        None
      }
    }
  ).filter_map(|node_option| node_option).collect()
}

fn astar(height_map: &HeightMap, start: Node, end: Node, memo: &mut HashMap<Node, Vec<Node>>) -> Option<Vec<Node>> {
  let mut open_set = HashSet::new();
  let mut came_from = HashMap::new();
  let mut g_score = HashMap::new();
  let mut f_score = DoublePriorityQueue::new();

  open_set.insert(start);
  g_score.insert(start, 0);
  f_score.push(start, height_map.manhattan_distance(&start, &end));

  while !open_set.is_empty() {
    let (current, _) = f_score.pop_min().unwrap();
    if current == end {
      break;
    }
    open_set.remove(&current);

    adjacent(current, height_map).into_iter().for_each(
      |neighbor| {
        let tentatitve_score_option = g_score.get(&current).map(|g| *g+1);
        if let Some(tentative_score) = tentatitve_score_option {
          let g_score_neighbor_option = g_score.get(&neighbor);
          if g_score_neighbor_option.is_none() || tentative_score < *g_score_neighbor_option.unwrap() {
            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_score);
            f_score.push(neighbor, tentative_score + height_map.manhattan_distance(&neighbor, &end));
            if !open_set.contains(&neighbor) {
              open_set.insert(neighbor);
            }
          }
        }
      }
    );
  }

  let mut path = vec![end];
  let mut curr = &end;
  
  while let Some(prev) = came_from.get(curr) {
    //memo.insert(*curr, path.clone().into_iter().rev().collect());
    path.push(*prev);
    curr = prev;
  }

  path.reverse();
  return if came_from.contains_key(&end) {Some(path)} else {None};
}

fn one(input: &Input) -> String {
  let height_map = HeightMap::new(&input.lines);
  let (start, end) = (height_map.start, height_map.end);
  let mut memo = HashMap::new();
  return if let Some(path) = astar(&height_map, start, end, &mut memo) {(path.len()-1).to_string()} else {"Unreachable".to_string()};
}

fn two(input: &Input) -> String {
  let height_map = HeightMap::new(&input.lines);
  let (_, end) = (height_map.start, height_map.end);
  let mut memo = HashMap::new();

  height_map.map.iter().enumerate().for_each(
    |(y,row)| {
      row.iter().enumerate().for_each(
        |(x,height)| {
          if *height == 0 {
            astar(&height_map, (x,y), end, &mut memo);
          }
        }
      );
    }
  );

  let mut paths: Vec<Vec<Node>> = memo.into_iter().filter(|(first, _)| height_map.height_at(first) == 0).map(|(_,path)| path).collect();

  paths.sort_by(|a,b| a.len().cmp(&b.len()));

  return (paths.first().unwrap().len()-1).to_string();
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
