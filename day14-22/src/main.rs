use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

type Point = (i32,i32);

#[derive(Debug,PartialEq)]
enum Unit { Rock, Source, Sand, LostSand }

impl Display for Unit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match &self {
      Unit::Rock     => '#',
      Unit::Source   => '+',
      Unit::Sand     => 'o',
      Unit::LostSand => '~'
    })
  }
}

#[derive(Debug)]
struct Cave { objects: HashMap<Point, Unit>, height: i32, has_bottom: bool }

impl Cave {
  fn new(lines: &Vec<String>, has_bottom: bool) -> Cave { 
    fn to_drawings(lines: &Vec<String>) -> Vec<Vec<Point>> {
      lines.iter().map(|line| {
        line.split(" -> ").map(
          |p_string| {
            let xy_list: Vec<i32> = p_string.split(',').map(|p| p.parse::<i32>().unwrap()).collect(); 
            assert_eq!(xy_list.len(), 2);
            (xy_list[0],xy_list[1])
          }
        ).collect()
      }).collect()
    }

    fn to_line(here: Point, there: Point) -> Vec<Point> {
      let reverse;
      
      let mut result: Vec<Point> = 
        if here.0 == there.0 {
          reverse = here.1 > there.1;
          (min(there.1,here.1)..=max(there.1,here.1)).map(|y| (here.0, y)).collect()
        } else if here.1 == there.1 {
          reverse = here.0 > there.0;
          (min(there.0,here.0)..=max(there.0,here.0)).map(|x| (x, here.1)).collect()
        } else {
          panic!("Could not produce a straight line from: {here:?} and {there:?}");
        };

      if reverse {
        result.reverse();
      }

      return result;
    }

    let mut objects = HashMap::new();
    let drawings = to_drawings(lines);

    drawings.into_iter().for_each(
      |drawing| {
        let mut drawing_iter = drawing.into_iter();
        if let Some(start) = drawing_iter.next() {
          objects.insert(start, Unit::Rock);
          drawing_iter.fold(
            start,
            |prev, next| {
              let mut line_iter = to_line(prev, next).into_iter();
              line_iter.next();
              line_iter.for_each(|p| { objects.insert(p, Unit::Rock); });
              next
            }
          );
        }
      }
    );

    objects.insert((500,0), Unit::Source);

    let (_, (_, height)) = Cave::to_dimensions(&objects);

    return Cave { objects: objects, height: height, has_bottom: has_bottom }
  }

  fn to_dimensions(objects: &HashMap<Point, Unit>) -> (Point, Point) {
    let (mut xs, mut ys): (Vec<i32>, Vec<i32>) = objects.keys().map(|p| *p).unzip();

    xs.sort_unstable();
    ys.sort_unstable();
 
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0); 

    return ((*xs.first().unwrap(),*ys.first().unwrap()),(*xs.last().unwrap(),*ys.last().unwrap()));
  }

  fn height(&self) -> i32 {(&self).height + if (&self).has_bottom {1} else {0} }
  fn dimensions(&self) -> (Point, Point) { Cave::to_dimensions(&self.objects) }
  fn end_reached(&self, p: Point) -> bool { p.1 == (&self).height() }

  fn sand_flow(&mut self) {
    fn sand_step(cave: &Cave, (x,y): Point) -> Option<Point> {
      let mut result = None;
      for (dx,dy) in vec![(0,1),(-1,1),(1,1)] {
        let curr = (x+dx,y+dy);
        let unit_option = cave.objects.get(&curr);
        if unit_option.is_none() || unit_option.is_some() && (*unit_option.unwrap() == Unit::Source || *unit_option.unwrap() == Unit::LostSand) {
          result = Some(curr);
          break;
        }
      }
      return result;
    }  

    let mut abyss_reached = false;
    let mut sources: Vec<Point> = (&self).objects.iter().filter(|(_,u)| **u == Unit::Source).map(|(p,_)| *p).collect();

    while !abyss_reached && !sources.is_empty() {
      for i in 0..sources.len() {
        let mut sand_pos = sources[i];
        let mut fix = false;
        while !fix {
          if (&self).end_reached(sand_pos) {
            fix = true;
            abyss_reached = !(&self).has_bottom;
          } else if let Some(new_pos) = sand_step(&self, sand_pos) {
            sand_pos = new_pos;
          } else {
            fix = true;
          }
        }
        sources = sources.into_iter().filter(|s| *s != sand_pos).collect();
        self.objects.insert(sand_pos, if abyss_reached {Unit::LostSand} else {Unit::Sand});
      }
    }
  }

  fn count_sand(&self) -> usize {
    (&self).objects.values().fold(0, |acc, u| acc + if *u == Unit::Sand {1} else {0})
  }
}

impl Display for Cave {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let dims@(ul, dr) = (&self).dimensions();
      write!(f, "{:?}{}", dims, ((ul.1)..=(&self).height()+1).fold(
        String::new(),
        |acc, y| {
          format!("{acc}\n{}", ((ul.0)..=(dr.0)).fold(
            String::new(),
            |acc, x| {
              format!("{acc}{}", 
                if y == (&self).height()+1 {
                  if (&self).has_bottom {
                    "#"
                  } else {
                    "v"
                  }.to_string()
                } else {
                  match (&self).objects.get(&(x,y)) {
                    Some(unit) => unit.to_string(), 
                    None       => ".".to_string()
                  }
                }
              )
            }
          ))
        }
      )
    )
  }
}

fn one(input: &Input) -> String {
  let mut cave = Cave::new(&input.lines, false);

  trace!("{cave}");
  cave.sand_flow();
  trace!("{cave}");

  return cave.count_sand().to_string();
}

fn two(input: &Input) -> String {
  let mut cave = Cave::new(&input.lines, true);

  trace!("{cave}");
  cave.sand_flow();
  trace!("{cave}");

  return cave.count_sand().to_string();
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
