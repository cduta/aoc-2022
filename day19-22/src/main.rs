use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::fmt::Display;
use std::time::Instant;

type Int = i64;

const ORE       : usize = 0;
const CLAY      : usize = 1;
const OBSIDIAN  : usize = 2;
const GEODE     : usize = 3;
const ROCK_COUNT: usize = 4;

fn print_rock(rock: usize) -> String {
  match rock {
    0 => "Ore".to_string(),
    1 => "Clay".to_string(),
    2 => "Obsidian".to_string(),
    3 => "Geode".to_string(),
    _ => panic!("Cannot print unknown rock")
  }
}

struct Blueprint { id: Int, costs: Vec<Vec<Int>> }

impl Display for Blueprint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Blueprint {}:\n{}", self.id, 
      self.costs.iter().enumerate().fold(
        String::new(), 
        |acc, (robot, costs)| {
          format!("{acc}   {:<8} Robot costs {}\n", print_rock(robot), 
            costs.iter().enumerate().fold(String::new(), |acc, (rock, cost)| format!("{acc}{}:{cost:<2} ", print_rock(rock)))
          )
        }
      )
    )
  }
}

struct State { robots: Vec<Int>, ressources: Vec<Int> }

impl State {
  fn new() -> State {
    State { 
      robots    : vec![1, 0, 0, 0],
      ressources: vec![0, 0, 0, 0]
    }
  }
}

impl State {
  fn to_build(&self, blueprint: &Blueprint) -> Option<usize> {
    return if self.ressources[OBSIDIAN] >= blueprint.costs[GEODE][OBSIDIAN] {
      Some(GEODE)
    } else if self.ressources[CLAY] >= blueprint.costs[OBSIDIAN][CLAY] {
      Some(OBSIDIAN)
    } else if self.ressources[ORE] >= blueprint.costs[CLAY][ORE] {
      Some(CLAY)
    } else {
      None
    }
  }

  fn build(&mut self, robot: usize, blueprint: &Blueprint) -> bool {
    let can_build = blueprint.costs[robot].iter().enumerate().all(|(rock, cost)| cost <= &self.ressources[rock]);
    if can_build {
      blueprint.costs[robot].iter().enumerate().for_each(
        |(rock, cost)| {
          self.ressources[rock] -= cost;
        }
      );
      self.robots[robot] += 1;
    }
    return can_build;
  }

  fn mine(&mut self) { for rock in 0..ROCK_COUNT { self.ressources[rock] += self.robots[rock]; }; }
}

impl Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}# Available: {}", 
      self.robots.iter().enumerate().fold(String::new(), |acc, (rock, count)| format!("{acc}{} Robots:{count:>2} ", print_rock(rock))),
      self.ressources.iter().enumerate().fold(String::new(), |acc, (rock, count)| format!("{acc}{}:{count:>2} ", print_rock(rock))),
    )
  }
}

fn prepare(lines: &Vec<String>) -> Vec<Blueprint> {
  lines.iter().map(
    |line| {
      let split1: Vec<&str> = line.split("Blueprint ").into_iter().collect();
      assert_eq!(split1.len(),2);
      let split2: Vec<&str> = split1[1].split(": Each ore robot costs ").into_iter().collect();
      assert_eq!(split2.len(),2);
      let id  = split2[0].parse::<Int>().unwrap();
      let split3: Vec<&str> = split2[1].split(" ore. Each clay robot costs ").into_iter().collect();
      assert_eq!(split3.len(),2);
      let ore_robot_costs = vec![split3[0].parse::<Int>().unwrap()];
      let split4: Vec<&str> = split3[1].split(" ore. Each obsidian robot costs ").into_iter().collect();
      assert_eq!(split4.len(),2);
      let clay_robot_costs = vec![split4[0].parse::<Int>().unwrap()];
      let split5: Vec<&str> = split4[1].split(" clay. Each geode robot costs ").into_iter().collect();
      assert_eq!(split5.len(),2);
      let obsidian_robot_costs: Vec<Int> = split5[0].split(" ore and ").map(|s| s.parse::<Int>().unwrap()).collect();
      assert_eq!(obsidian_robot_costs.len(),2);
      let split6: Vec<&str> = split5[1].split(" obsidian.").into_iter().collect();
      assert_eq!(split6.len(),2);
      let geode_robot_costs   : Vec<Int> = split6[0].split(" ore and ").map(|s| s.parse::<Int>().unwrap()).collect();
      assert_eq!(geode_robot_costs.len(),2);
      Blueprint { 
        id, 
        costs: 
          vec![
            vec![ore_robot_costs[0]     , 0                      , 0,                    0],
            vec![clay_robot_costs[0]    , 0                      , 0,                    0],
            vec![obsidian_robot_costs[0], obsidian_robot_costs[1], 0,                    0],
            vec![geode_robot_costs[0]   , 0                      , geode_robot_costs[1], 0]
          ]
      }
    }
  ).collect()
}

fn one(input: &Input) -> String {
  let blueprints = prepare(&input.lines);
  let mut quality = 0;

  blueprints.into_iter().for_each(
    |blueprint| {
      trace!("{blueprint}");
      let mut state = State::new();
      let total_mins = 24;
      let mut mins_left = total_mins;
      while mins_left > 0 {
        mins_left -= 1;
        trace!("Min: {:>2} # {state}", total_mins-mins_left);
        let robot_option = state.to_build(&blueprint);
        state.mine();
        if let Some(robot) = robot_option {
          state.build(robot, &blueprint);
        }
      }
      trace!("Final   # {state}");
      quality += blueprint.id*state.ressources[GEODE];
    }
  );

  return quality.to_string();
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
