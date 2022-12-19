use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashSet;
use std::time::Instant;

type Int = i64;
type Square = (Int,Int);
type Cube = (Int,Int,Int);
type Dims = (Int,Int,Int,Int,Int,Int);

enum Axis {X, Y, Z}

fn prepare(lines: &Vec<String>) -> HashSet<Cube> {
  lines.iter().map(
    |line| {
      let pos_list: Vec<Int> = line.split(',').map(|v| v.parse::<Int>().unwrap()).collect();
      assert_eq!(pos_list.len(), 3);
      (pos_list[0],pos_list[1],pos_list[2])
    }
  ).collect()
}

fn scan_surface(cubes: &HashSet<Cube>, start: Int, end: Int, reversed: bool, axis: Axis) -> usize {
  let mut steps: Vec<Int> = (start..=end).collect();
  if reversed { steps.reverse(); }

  let mut prev = HashSet::new();
  let mut result = 0;

  for c in steps {
    let curr: HashSet<Square> = cubes.iter().filter(
      |(x,y,z)| {
        match axis {
          Axis::X => *x == c,
          Axis::Y => *y == c,
          Axis::Z => *z == c
        }
      }
    ).map(
      |(x,y,z)| {
        match axis {
          Axis::X => (*y,*z),
          Axis::Y => (*x,*z),
          Axis::Z => (*x,*y)
        }
      }
    ).collect();
    result += (&curr - &prev).len();
    prev = curr;
  }

  return result;
}


fn dimensions(cubes: &HashSet<Cube>) -> Dims {
  let back   = cubes.iter().min_by(|(_,_,z1), (_,_,z2)| z1.cmp(z2)).unwrap().2;
  let front  = cubes.iter().max_by(|(_,_,z1), (_,_,z2)| z1.cmp(z2)).unwrap().2;
  let bottom = cubes.iter().min_by(|(_,y1,_), (_,y2,_)| y1.cmp(y2)).unwrap().1;
  let top    = cubes.iter().max_by(|(_,y1,_), (_,y2,_)| y1.cmp(y2)).unwrap().1;
  let left   = cubes.iter().min_by(|(x1,_,_), (x2,_,_)| x1.cmp(x2)).unwrap().0;
  let right  = cubes.iter().max_by(|(x1,_,_), (x2,_,_)| x1.cmp(x2)).unwrap().0;

  return (back, front, bottom, top, left, right);
}

fn one(input: &Input) -> String {
  let cubes = prepare(&input.lines);
  let (back, front, bottom, top, left, right) = dimensions(&cubes);

  let mut total = 0;

  total += scan_surface(&cubes, left, right, false, Axis::X);
  total += scan_surface(&cubes, left, right, true , Axis::X);
  total += scan_surface(&cubes, bottom, top, false, Axis::Y);
  total += scan_surface(&cubes, bottom, top, true , Axis::Y);
  total += scan_surface(&cubes, back, front, false, Axis::Z);
  total += scan_surface(&cubes, back, front, true , Axis::Z);

  return total.to_string();
}

fn two(input: &Input) -> String {
  fn neighbors((x,y,z): &Cube, (back, front, bottom, top, left, right): &Dims) -> Vec<Cube> {
    let mut result = vec![];
    if x >= left {
      result.push((*x-1,*y,*z));
    }  
    if x <= right {
      result.push((*x+1,*y,*z));
    }  
    if y >= bottom {
      result.push((*x,*y-1,*z));
    }  
    if y <= top {
      result.push((*x,*y+1,*z));
    }  
    if z >= back {
      result.push((*x,*y,*z-1));
    }  
    if z <= front {
      result.push((*x,*y,*z+1));
    }  
    return result;
  }

  let cubes = prepare(&input.lines);
  let dims@(back, _, bottom, _, left, _) = dimensions(&cubes);

  let mut total = 0;
  let mut to_visit = vec![(left-1, bottom-1, back-1)];
  let mut visited  = vec![(left-1, bottom-1, back-1)].into_iter().collect::<HashSet<Cube>>();

  while let Some((x,y,z)) = to_visit.pop() {
    for neighbor in neighbors(&(x,y,z), &dims) {
      if cubes.contains(&neighbor) {
        total += 1;
      } else if !visited.contains(&neighbor) {
        visited.insert(neighbor);
        to_visit.push(neighbor);
      }
    }
  }

  return total.to_string();
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
