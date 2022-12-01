use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;

fn prepare(lines: &Vec<String>) -> Vec<Option<i32>> {
  let mut lines = lines.clone();

  lines.push("".to_string());

  let items_result: Vec<Option<Result<i32, core::num::ParseIntError>>> = lines.iter().map(|line| if line == "" {None} else {Some(line.parse::<i32>())}).collect();

  if let Some(Some(Err(err))) = items_result.iter().find(|item| item.is_some() && item.as_ref().unwrap().is_err()) {
    panic!("Failed to parse all lines of input to i32: {err}");
  }

  items_result.into_iter().map(|some_item| 
    match some_item {
      Some(Ok(item)) => Some(item),
      _              => None
    }
  ).collect()
}

fn one(input: &Input) -> String {
  #[derive(Debug)]
  struct MostPacked {
    some_max : Option<i32>,
    some_curr: Option<i32>
  }

  let items = prepare(&input.lines);

  let most_packed = items.into_iter().fold( MostPacked { some_max: None, some_curr: None }, 
    |MostPacked { some_max, some_curr }, some_item| 
      match some_item {
        Some(item) => MostPacked { some_max: some_max, some_curr: if let Some(curr) = some_curr {Some(curr+item)} else {Some(item)} },
        None       => match some_max {
          Some(max) => MostPacked { some_max: if let Some(curr) = some_curr {Some(std::cmp::max(max, curr))} else {Some(max)}, some_curr: None },
          None      => MostPacked { some_max: some_curr, some_curr: None }
        }
      }
  );

  return if let Some(max) = most_packed.some_max {max.to_string()} else {"None".to_string()};
}

fn two(input: &Input) -> String {
  #[derive(Debug)]
  struct MostPacked {
    top_three: Vec<Option<i32>>,
    some_curr: Option<i32>
  }

  let items = prepare(&input.lines);

  let most_packed = items.into_iter().fold( MostPacked { top_three: vec![None,None,None], some_curr: None }, 
    |MostPacked { top_three, some_curr }, some_item| 
      match some_item {
        Some(item) => MostPacked { top_three: top_three, some_curr: if let Some(curr) = some_curr {Some(curr+item)} else {Some(item)} },
        None       => match top_three[..] {
          [fst,snd,trd] => {
            let new_top_three = match some_curr {
              Some(curr) if fst.is_none() || curr > fst.unwrap() => vec![some_curr,fst,snd],
              Some(curr) if snd.is_none() || curr > snd.unwrap() => vec![fst,some_curr,snd],
              Some(curr) if trd.is_none() || curr > trd.unwrap() => vec![fst,snd,some_curr],
              _          => top_three  
            };
            MostPacked { top_three: new_top_three, some_curr: None }},
          _             => panic!("The amount of elements in to_three was not three: {top_three:?}")
        }
      }
  );

  return if let [None,None,None] = most_packed.top_three[..] {
    "None".to_string()
  } else { 
    most_packed.top_three.into_iter().fold(0, |sum, some_top| sum + if let Some(top) = some_top {top} else {0}).to_string()
  };
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
