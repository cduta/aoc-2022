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

fn elephant_pressure(valves: &Vec<Valve>, distance_matrix: &Vec<Vec<Distance>>, start: ValveId, total_mins: Time) -> Flow {
  let mut steps = vec![(start, 0, start, 0, total_mins, 0, HashSet::<ValveId>::new())];
  let mut result = 0;

  while let Some((h_pos, h_wait, e_pos, e_wait, mins_left, released, mut open)) = steps.pop() {
    let new_h_wait;
    let new_e_wait;
    let new_mins_left;
    let r;
    if h_wait == e_wait && h_wait < mins_left { // Both wait the same amount
      new_mins_left = mins_left - h_wait;
      r = if h_pos == e_pos {0} else {
        let r = open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * h_wait);
        result = std::cmp::max(result, r);
        open.insert(h_pos);
        open.insert(e_pos);
        r
      };

      let h_steps = distance_matrix[h_pos].iter().fold(
        Vec::new(),
        |mut acc, Distance {mins: ht, valve_id: hv}| {
          if mins_left >= *ht && !open.contains(hv) {
            acc.push((*ht, *hv));
          };
          acc
        }
      );

      let e_steps = distance_matrix[e_pos].iter().fold(
        Vec::new(),
        |mut acc, Distance {mins: et, valve_id: ev}| {
          if mins_left >= *et && !open.contains(ev) {
            acc.push((*et, *ev));
          };
          acc
        }
      );

      if !h_steps.is_empty() && !e_steps.is_empty() {
        h_steps.iter().for_each(
          |(ht,hv)| {
            e_steps.iter().for_each(
              |(et,ev)| {
                if *hv != *ev {
                  steps.push((*hv, *ht, *ev, *et, new_mins_left, r, open.clone()));
                }
              }
            );
          }
        );
      } else if !h_steps.is_empty() {
        h_steps.iter().for_each(
          |(ht,hv)| {
            steps.push((*hv, *ht, e_pos, 100, new_mins_left, r, open.clone()));
          }
        );
      } else if !e_steps.is_empty() {
        e_steps.iter().for_each(
          |(et,ev)| {
            steps.push((e_pos, 100, *ev, *et, new_mins_left, r, open.clone()));
          }
        );
      } 

    } else if h_wait < e_wait && h_wait < mins_left { // Elephant must wait longer
      new_e_wait = e_wait - h_wait;
      new_mins_left = mins_left - h_wait;
      r = open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * h_wait);
      result = std::cmp::max(result, r);
      open.insert(h_pos);

      distance_matrix[h_pos].iter().for_each(
        |Distance {mins: t, valve_id: v}| {
          if new_mins_left >= *t && !open.contains(v) {
            steps.push((*v, *t, e_pos, new_e_wait, new_mins_left, r, open.clone()));
          }
        }
      );
    } else if h_wait > e_wait && e_wait < mins_left { // Human must wait longer
      new_h_wait = h_wait - e_wait;
      new_mins_left = mins_left - e_wait;
      r = open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * e_wait);
      result = std::cmp::max(result, r);
      open.insert(e_pos);

      distance_matrix[e_pos].iter().for_each(
        |Distance {mins: t, valve_id: v}| {
          if new_mins_left >= *t && !open.contains(v) {
            steps.push((h_pos, new_h_wait, *v, *t, new_mins_left, r, open.clone()));
          }
        }
      );
    }
    result = std::cmp::max(result, open.iter().fold(released, |acc, o: &ValveId| acc + valves[*o].flow * mins_left));
  }

  return result;
}

fn two(input: &Input) -> String {
  let (valves, string_map) = prepare(&input.lines);
  let distance_matrix = distance_matrix(&valves);

  return elephant_pressure(&valves, &distance_matrix, string_map["AA"], 26).to_string();
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
