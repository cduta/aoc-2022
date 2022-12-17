use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::{HashSet, HashMap};
use std::fmt::Display;
use std::time::Instant;

type ValveId = usize;
type Flow = u32;
type Time = u32;

#[derive(Debug)]
struct Valve { 
  id       : ValveId, 
  name     : String, 
  flow     : Flow, 
  neighbors: HashSet<ValveId> }

impl Valve {
  fn new(id: ValveId, name: String, flow: Flow, neighbors: HashSet<ValveId>) -> Valve {
    Valve { id, name, flow, neighbors }
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
    write!(f, "({:>2} {} {:>2}) -> [{}]", 
      self.id, 
      self.name, 
      self.flow, 
      neighbor_iter.fold(
        first_neighbor,
        |acc, nid| {
          format!("{acc}, {nid}")
        }
      )
    )
  } 
}

fn prepare(lines: &Vec<String>) -> (Vec<Valve>, HashMap<String,ValveId>) {
  fn parse(line: &String) -> (String, Flow, HashSet<String>) {
    let mut first_split: Vec<&str> = line.split("; tunnels lead to valves ").collect();
    if first_split.len() == 1 {
      first_split = line.split("; tunnel leads to valve ").collect();
    }
    assert_eq!(first_split.len(), 2);
    let second_split: Vec<&str> = first_split[0].split(" has flow rate=").collect();
    assert_eq!(second_split.len(), 2);
    

    return (second_split[0].split("Valve ").nth(1).unwrap().to_string(), 
            second_split[1].parse::<Flow>().unwrap(), 
            first_split[1].split(", ").map(|e| e.to_string()).collect());
  }

  let mut valve_parsed = vec![];
  let mut name_ids = HashMap::new();
  lines.iter().enumerate().for_each(
    |(i,line)| {
      let (name, flow, neighbor_strings) = parse(line);
      valve_parsed.push((name.to_string(), flow, neighbor_strings));
      name_ids.insert(name, i);
    }
  );
  
  let mut valves = vec![];

  valve_parsed.iter().enumerate().for_each(
    |(i, (name, flow, neighbor_names))| {
      valves.push(Valve::new(
        i, 
        name.to_string(), 
        *flow, 
        neighbor_names.iter().map(
          |name| 
            name_ids[name]
        ).collect::<HashSet<ValveId>>()));
    }
  );
  return (valves, name_ids);
}

#[derive(Debug)]
struct Distance { mins: Time, valve_id: ValveId }

fn distance_matrix(valves: &Vec<Valve>) -> Vec<Vec<Distance>>{
  let mut distance_matrix = vec![];
  let useless = valves.iter().filter(|v| v.flow == 0).fold(
    HashSet::new(), 
    |mut acc, v| {
      acc.insert(v.id); 
      acc
    }
  );
  valves.iter().for_each(
    |valve| {
      let mut mins: Time = 1;
      let mut visited = HashSet::new();
      let mut distances = Vec::new();
      let mut to_visit = HashSet::new();
      to_visit.insert(valve.id);

      while !to_visit.is_empty() {
        let mut next_visit = HashSet::new();
        to_visit.iter().for_each(
          |id| {
            distances.push(Distance { mins: mins, valve_id: *id });
            visited.insert(*id);
            (&(&valves[*id].neighbors - &visited) - &to_visit).into_iter().for_each(|id| { next_visit.insert(id); } );
          }
        );
        to_visit = next_visit;
        mins += 1;
      }
      distances = distances.into_iter().filter(|d| !useless.contains(&d.valve_id)).collect();
      //distances.sort_by(|d1,d2| d1.mins.cmp(&d2.mins));
      assert_eq!(distances.len(), valves.len() - useless.len());
      distance_matrix.push(distances);
    }
  );
  return distance_matrix;
}

fn release_pressure(valves: &Vec<Valve>, distance_matrix: &Vec<Vec<Distance>>, start: ValveId, total_mins: Time) -> Flow {
  let mut steps = vec![(start, total_mins, 0, HashSet::new())];
  let mut result = 0;

  while let Some((pos, mins_left, released, open)) = steps.pop() {
    distance_matrix[pos].iter().for_each(
      |Distance {mins: t, valve_id: v}| {
        if mins_left >= *t && !open.contains(v) {
          let r = open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * t);
          let mut next_open = open.clone();
          result = std::cmp::max(result, r);
          next_open.insert(*v);
          steps.push((*v, mins_left-t, r, next_open));
        }
      }
    );
    result = std::cmp::max(result, open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * mins_left));
  }

  return result;
}

fn one(input: &Input) -> String {
  let (valves, string_map) = prepare(&input.lines);
  let distance_matrix = distance_matrix(&valves);

  return release_pressure(&valves, &distance_matrix, string_map["AA"], 30).to_string();
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
