use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::cmp::{min, max};
use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

type Point = (i32,i32);

#[derive(Debug)]
struct Sensor { pos: Point, beacon: Point }

impl Sensor {
  fn new(line: &String) -> Sensor {
    let (_, s) = line.split_at(12);
    let positions: Vec<Point> = s.split(": closest beacon is at x=").map(
      |pos_string| {
        let nums: Vec<&str> = pos_string.split(", y=").collect();
        assert_eq!(nums.len(),2);
        (nums[0].parse::<i32>().unwrap(), nums[1].parse::<i32>().unwrap())
      }
    ).collect();
    assert_eq!(positions.len(), 2);
    return Sensor { pos: positions[0], beacon: positions[1] };
  }

  fn radius(&self) -> i32 {
    ((&self).pos.0.abs_diff((&self).beacon.0) + 
     (&self).pos.1.abs_diff((&self).beacon.1)) as i32
  }

  fn y_range(&self, y: i32) -> Option<(i32, i32)> {
    let y_distance = if (&self).pos.1 < y {((&self).pos.1 + (&self).radius()) - y} else {y - ((&self).pos.1 - (&self).radius())};
    if y_distance < 0 { return None; }
    let (mut lower, mut upper) = ((&self).pos.0-y_distance, (&self).pos.0+y_distance);
    if (lower, y) == (&self).beacon {
      lower += 1;
    } else if (upper, y) == (&self).beacon {
      upper -= 1;
    }

    //trace!("Range of {} on y={y} is {:?} [y_distance={y_distance}]", &self, if lower <= upper {Some((lower,upper))} else {None});

    return if lower <= upper {Some((lower,upper))} else {None};
  }
}

impl Display for Sensor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(S:({:>2},{:>2}),B:({:>2},{:>2}),r:{:>2})", (&self).pos.0, (&self).pos.1, (&self).beacon.0, (&self).beacon.1, (&self).radius())
  }
}

fn prepare(lines: &Vec<String>) -> (i32, Vec<Sensor>) {
  let mut lines_iter = lines.iter();
  let arg_strings: Vec<&str> = lines_iter.next().unwrap().split('=').collect();
  assert_eq!(arg_strings.len(), 2);
  lines_iter.next();
  return (arg_strings[1].parse::<i32>().unwrap(), lines_iter.map(|line| Sensor::new(line)).collect());
}

fn one(input: &Input) -> String {
  let (y, sensors) = prepare(&input.lines);
  //trace!("y={y}");
  //trace!("{}", sensors.iter().fold(String::new(), |acc, sensor| format!("{acc}\n{sensor}")));
  let ranges = sensors.into_iter().map(|sensor| { sensor.y_range(y) }); 
  let mut merged_ranges: Vec<(i32,i32)> = ranges.fold(
    HashSet::new(),
    |mut acc, range_option| {
      match range_option {
        Some(range@(lower,upper)) => {
          let to_merge: Vec<(i32, i32)> = acc.iter().filter(
            |(l,u)| *l-1     <= lower && lower <= *u+1     || *l-1     <= upper && upper <= *u+1 || 
                     lower-1 <= *l    && *l    <=  upper+1 ||  lower-1 <= *u    && *u    <=  upper+1
          ).map(|(l,u)| (*l,*u)).collect();
          trace!("Add {range:?} to {acc:?}\nto_merge: {to_merge:?}");
          if to_merge.is_empty() {
            acc.insert(range);
          } else {
            let new_range = (
              min(lower, to_merge.iter().min_by(|(l1,_), (l2,_)| l1.cmp(l2)).unwrap().0), 
              max(upper, to_merge.iter().max_by(|(_,u1), (_,u2)| u1.cmp(u2)).unwrap().1)
            );
            to_merge.into_iter().for_each(|range| { acc.remove(&range); } );
            acc.insert(new_range);
          }
        },
        None                      => ()
      };      
      acc
    }
  ).into_iter().collect();
  merged_ranges.sort();
  trace!("{}", merged_ranges.iter().fold(String::new(), |acc,(lower,upper)| format!("{acc}({lower},{upper}) ")));
  return merged_ranges.into_iter().map(|(l,u)| u-l+1).sum::<i32>().to_string();
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
