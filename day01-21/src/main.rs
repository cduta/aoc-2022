use common::helper::{from_strings};
use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;

use std::collections::VecDeque;

fn one(input: &Input) -> String {  
  struct Acc {
    prev : Option<i32>,
    count: i32
  }

  let depths: Vec<i32> = from_strings(input.lines.clone());

  let acc = depths.into_iter().fold(Acc { prev: None, count: 0 }, |acc, curr| {
    match acc {
      Acc { prev: None,       count } => Acc { prev: Some(curr), count: count },
      Acc { prev: Some(prev), count } => Acc { prev: Some(curr), count: count + if prev < curr {1} else {0} }
    }
  });

  acc.count.to_string()
}

fn two(input: &Input) -> String {
  struct Acc {
    prevs: VecDeque<i32>,
    count: i32
  }
  let sum = |acc: i32, x: &i32| acc+x;

  let depths: Vec<i32> = from_strings(input.lines.clone());

  let acc = depths.into_iter().fold(Acc { prevs: VecDeque::new(), count: 0 }, |acc, curr| {
    match acc {
      Acc { prevs, count: _ } if prevs.len() > 3 => panic!("On day two: prevs cannot have more than three elements. prevs: {:?}", prevs),
      Acc { mut prevs, count } if prevs.len() < 3 => {
        Acc { prevs: { prevs.push_front(curr); prevs }, count: count }
      },
      Acc { mut prevs, count } => {
        let prev_sum = prevs.iter().fold(0, sum);
        prevs.pop_back();
        prevs.push_front(curr);
        let curr_sum = prevs.iter().fold(0, sum);

        Acc { prevs: prevs, count: count + if prev_sum < curr_sum {1} else {0} }
      }
    }
  });

  acc.count.to_string()
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
