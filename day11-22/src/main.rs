use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

#[derive(Debug)]
enum Arg { Int(i64), Old }

impl Arg {
  fn new(arg_string: String) -> Arg {
    match arg_string.as_str() {
      "old" => Arg::Old, 
       arg  => Arg::Int(arg.parse::<i64>().unwrap())
    }
  }

  fn value(&self, old: i64) -> i64 {
    match &self {
      Arg::Old    => old, 
      Arg::Int(v) => *v
    }
  }
}

#[derive(Debug)]
enum Op { Plus(Arg), Mult(Arg) }

impl Op {
  fn new(op_string: String, arg_string: String) -> Op {
    match op_string.as_str() {
      "+" => Op::Plus(Arg::new(arg_string)), 
      "*" => Op::Mult(Arg::new(arg_string)),
       _  => panic!("Malformed operator string: {op_string}")
    }
  }

  fn apply(&self, old: i64) -> i64 {
    match &self {
      Op::Plus(arg) => old + arg.value(old),
      Op::Mult(arg) => old * arg.value(old)
    }
  }
}

#[derive(Debug)]
struct ThrowTest { divisible_by: i64, if_true: usize, if_false: usize }

impl ThrowTest {
  fn new(divisible_by_string: &String, if_true_string: &String, if_false_string: &String) -> ThrowTest {
    ThrowTest { 
      divisible_by: divisible_by_string.parse::<i64>().unwrap(), 
      if_true     : if_true_string.parse::<usize>().unwrap(),
      if_false    : if_false_string.parse::<usize>().unwrap()
    }
  }

  fn which(&self, item: i64) -> usize {
    if item % &self.divisible_by == 0 {
      (&self).if_true 
    } else {
      (&self).if_false
    }
  }
}

#[derive(Debug)]
struct Monkey { items: VecDeque<i64>, operation: Op, throw_test: ThrowTest, inspected: i64 }

impl Monkey {
  fn new(monkey_strings: &Vec<&String>) -> Monkey {
    assert_eq!(monkey_strings.len(),6);
    Monkey {
      items: monkey_strings[1].replace("  Starting items: ", "").split(", ").fold(
        VecDeque::new(), 
        |mut acc, elem| {
          acc.push_back(elem.parse::<i64>().unwrap());
          acc
        }
      ),
      operation: {
        if let [op_string, arg_string] = monkey_strings[2].replace("  Operation: new = old ", "").split_whitespace().collect::<Vec<&str>>()[..] {
          Op::new(op_string.to_string(), arg_string.to_string())
        } else {
          panic!("Malformed operation string: {}", monkey_strings[2]);
        }
      },
      throw_test: ThrowTest::new(&monkey_strings[3].replace("  Test: divisible by ", ""), 
                                 &monkey_strings[4].replace("    If true: throw to monkey ", ""), 
                                 &monkey_strings[5].replace("    If false: throw to monkey ", "")),
      inspected: 0
    }
  }

  fn throw(&self, item: i64, throw_to: &mut Vec<VecDeque<i64>>, modulo: i64, relieve: i64) {
    let new_item = ((&self).operation.apply(item)/relieve) % modulo;
    throw_to[(&self).throw_test.which(new_item)].push_back(new_item);
  }
}

impl Display for Monkey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.items.iter().enumerate().fold(
      "".to_string(),
      |acc, (i,item)| {
        format!("{acc}{}{}", if i == 0 {""} else {", "}, item)
      } 
    ))
  }
}

fn prepare(lines: &Vec<String>) -> Vec<Monkey> {
  lines.iter().fold(
    vec![vec![]],
    |mut acc, line| {
      match line.len() {
        0 => { acc.push(vec![]); acc },
        _ => { acc.last_mut().unwrap().push(line); acc}
      }
    }
  ).iter().map(|monkey_strings| {
    Monkey::new(monkey_strings)
  }).collect()
}

fn play(mut monkeys: Vec<Monkey>, rounds: i64, relieve: i64) -> Vec<Monkey> {
  let modulo = monkeys.iter().map(|monkey| monkey.throw_test.divisible_by).product();
  (0..rounds).for_each(|_round| {
    for i in 0..monkeys.len() {
      let mut throws: Vec<VecDeque<i64>> = (0..monkeys.len()).map(|_| VecDeque::new()).collect();
      monkeys[i].items.iter().for_each(|item| {
        monkeys[i].throw(*item, &mut throws, modulo, relieve);
      });
      monkeys[i].inspected += monkeys[i].items.len() as i64;
      monkeys[i].items.clear();
      throws.iter_mut().enumerate().for_each(|(j,items)| monkeys[j].items.append(items));
    }
    // trace!("After round {round}, the monkeys are holding items with these worry levels:");
    // trace!("{}", monkeys.iter().enumerate().fold(
    //   "".to_string(),
    //   |acc, (i, monkey)| {
    //     format!("{acc}\nMonkey {i}: {monkey}")
    //   }));
  });
  monkeys
}

fn one(input: &Input) -> String {
  let monkeys: Vec<i64> = play(prepare(&input.lines), 20, 3).into_iter().map(|monkey| monkey.inspected).collect();

  trace!("{}", monkeys.iter().enumerate().fold(
    "".to_string(),
    |acc, (i, inspected)| {
      format!("{acc}\nMonkey {i} inspected items {inspected} times")
    }
  ));

  let mut monkey_business = monkeys.into_iter().enumerate().collect::<Vec<(usize,i64)>>();
  monkey_business.sort_by(|(_,x), (_,y)| y.cmp(x));

  return (monkey_business[0].1 * monkey_business[1].1).to_string();
}

fn two(input: &Input) -> String {
  let monkeys: Vec<i64> = play(prepare(&input.lines), 10000, 1).into_iter().map(|monkey| monkey.inspected).collect();

  trace!("{}", monkeys.iter().enumerate().fold(
    "".to_string(),
    |acc, (i, inspected)| {
      format!("{acc}\nMonkey {i} inspected items {inspected} times")
    }
  ));

  let mut monkey_business = monkeys.into_iter().enumerate().collect::<Vec<(usize,i64)>>();
  monkey_business.sort_by(|(_,x), (_,y)| y.cmp(x));

  return (monkey_business[0].1 * monkey_business[1].1).to_string();
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
