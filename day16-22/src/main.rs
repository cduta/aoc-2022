use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::{HashSet, HashMap};
use std::fmt::Display;
use std::time::Instant;

#[derive(Clone)]
struct Valve { 
  id       : usize, 
  name     : String, 
  flow     : i64, 
  open     : bool, 
  neighbors: HashSet<usize> }

impl Valve {
  fn new(id: usize, name: String, flow: i64, neighbors: HashSet<usize>) -> Valve {
    Valve { id, name, flow, open: false, neighbors }
  }
}

impl Display for Valve {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut neighbor_iter = self.neighbors.iter();
    let first_neighbor = if let Some(nid) = neighbor_iter.next() {
      nid.to_string()
    } else {
      String::new()
    };
    write!(f, "({:>2} {} {:>2} {}) -> [{}]", 
      self.id, 
      self.name, 
      self.flow, 
      if self.open {'✓'} else {'✗'}, 
      neighbor_iter.fold(
        first_neighbor,
        |acc, nid| {
          format!("{acc}, {nid}")
        }
      )
    )
  } 
}

fn prepare(lines: &Vec<String>) -> (HashMap<usize, Valve>, HashMap<String, usize>) {
  fn parse(line: &String) -> (String, i64, HashSet<String>) {
    let mut first_split: Vec<&str> = line.split("; tunnels lead to valves ").collect();
    if first_split.len() == 1 {
      first_split = line.split("; tunnel leads to valve ").collect();
    }
    assert_eq!(first_split.len(), 2);
    let second_split: Vec<&str> = first_split[0].split(" has flow rate=").collect();
    assert_eq!(second_split.len(), 2);
    

    return (second_split[0].split("Valve ").nth(1).unwrap().to_string(), 
            second_split[1].parse::<i64>().unwrap(), 
            first_split[1].split(", ").map(|e| e.to_string()).collect());
  }

  let mut valve_parsed = HashMap::new();
  let mut name_ids = HashMap::new();
  lines.iter().enumerate().for_each(
    |(i,line)| {
      let (name, flow_string, neighbor_strings) = parse(line);
      valve_parsed.insert(i+1, (name.to_string(), flow_string, neighbor_strings));
      name_ids.insert(name, i+1);
    }
  );
  
  let mut valves = HashMap::new();

  valve_parsed.iter().for_each(
    |(i, (name, flow, neighbor_names))| {
      valves.insert(*i, Valve::new(
        *i, 
        name.to_string(), 
        *flow, 
        neighbor_names.iter().map(
          |name| 
            name_ids[name]
        ).collect::<HashSet<usize>>()));
    }
  );
  return (valves, name_ids);
}

fn trace_valves(valve_map: &HashMap<usize, Valve>) {
  let mut valves: Vec<&usize> = valve_map.keys().collect();
  valves.sort_by(|v1, v2| valve_map[v1].id.cmp(&valve_map[v2].id));

  trace!("{}", valves.iter().fold(
    String::new(),
    |acc, id| {
      format!("{acc}\n{}", valve_map[id])
    }
  ));
}

fn make_distance_map(valve_map: &HashMap<usize, Valve>) -> HashMap<usize, Vec<(usize,usize)>>{
  let mut distance_map = HashMap::new();
  valve_map.values().for_each(
    |valve| {
      let mut minutes: usize = 1;
      let mut visited = HashSet::new();
      let mut distances = Vec::new();
      let mut to_visit = HashSet::new();
      to_visit.insert(valve.id);

      while !to_visit.is_empty() {
        let mut next_visit = HashSet::new();
        to_visit.iter().for_each(
          |id| {
            distances.push((minutes, *id));
            visited.insert(*id);
            (&(&valve_map.get(&id).unwrap().neighbors - &visited) - &to_visit).into_iter().for_each(|id| { next_visit.insert(id); } );
          }
        );
        to_visit = next_visit;
        minutes += 1;
      }
      distances.sort_by(|(d1,_),(d2,_)| d1.cmp(&d2));
      assert_eq!(distances.len(), valve_map.len());
      distance_map.insert(valve.id, distances);
    }
  );
  return distance_map;
}

fn toggle_valve(valve_map: &mut HashMap<usize, Valve>, id: &usize) {
  valve_map.get_mut(id).unwrap().open = true;
}

fn one(input: &Input) -> String {
  let (mut valve_map, string_map) = prepare(&input.lines);
  let (mut mins_left, mut pos, mut released) = (30, string_map["AA"], 0);
  let distance_map = make_distance_map(&valve_map);

  trace!("{distance_map:?}");

  while mins_left > 0 {
    // Go walk through the cave and release the best valves based on mins_left and distance_map
    mins_left -= 1;
  }

  return "42".to_string();
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
