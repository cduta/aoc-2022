// ## TODO ## (Part 2)
use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

type Number = i64;

#[derive(Clone)]
enum Arg { Wait(usize), Ready(Number) }

impl Arg {
  fn is_waiting(&self) -> bool {
    match self {
      Arg::Wait(_) => true,
      _            => false
    }
  } 

  fn ready(&mut self, other: &Monkey) {
    assert!(other.has_value());
    match self {
      Arg::Wait(id) if *id == other.id => *self = Arg::Ready(other.expr.get_value().unwrap()),
      _                                => ()
    }
  }
}

impl Display for Arg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", 
      match self {
        Arg::Wait(id)   => format!("m:{id:>4}"),
        Arg::Ready(val) => format!("val:{val:>4}")
      }
    )
  }
}

#[derive(Clone)]
enum Expr {  
  Plus(Arg,Arg),
  Minus(Arg,Arg),
  Multiply(Arg,Arg),
  Divide(Arg,Arg),
  Value(Number)
}

impl Expr {
  fn get_value(&self) -> Option<Number> {
    if let Expr::Value(val) = self {
      Some(*val)
    } else {
      None
    }
  }
}

impl Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", 
      match self {
        Expr::Plus(arg1,arg2)     => format!("{arg1}+{arg2}"),
        Expr::Minus(arg1,arg2)    => format!("{arg1}-{arg2}"),
        Expr::Multiply(arg1,arg2) => format!("{arg1}*{arg2}"),
        Expr::Divide(arg1,arg2)   => format!("{arg1}÷{arg2}"),
        Expr::Value(val)          => format!("{val}")
      }
    )
  }
}

#[derive(Clone)]
struct Monkey { id: usize, name: String, expr: Expr }

impl Monkey {
  fn is_root(&self, root_id: usize) -> bool { self.id == root_id }
  fn has_value(&self) -> bool { if let Expr::Value(_) = self.expr { true } else { false } }

  fn listen(&mut self, other: &Monkey) {
    assert!(self.waits());
    assert!(!self.has_value());
    assert!(other.has_value());
    let new_expr = match self.expr.clone() {
      Expr::Plus(mut arg1,mut arg2)     => {
        arg1.ready(other);
        arg2.ready(other);
        Expr::Plus(arg1, arg2)
      },
      Expr::Minus(mut arg1,mut arg2)    => {
        arg1.ready(other);
        arg2.ready(other);
        Expr::Minus(arg1, arg2)
      },
      Expr::Multiply(mut arg1,mut arg2) => {
        arg1.ready(other);
        arg2.ready(other);
        Expr::Multiply(arg1, arg2)
      },
      Expr::Divide(mut arg1,mut arg2)   => {
        arg1.ready(other);
        arg2.ready(other);
        Expr::Divide(arg1, arg2)
      },
      Expr::Value(_)                    => panic!("Monkey tried to listen, but was actually screaming")
    };
    self.expr = new_expr;
  }

  fn waits(&self) -> bool {
    match &self.expr {
      Expr::Plus(arg1,arg2)     => arg1.is_waiting() || arg2.is_waiting(), 
      Expr::Minus(arg1,arg2)    => arg1.is_waiting() || arg2.is_waiting(),
      Expr::Multiply(arg1,arg2) => arg1.is_waiting() || arg2.is_waiting(),
      Expr::Divide(arg1,arg2)   => arg1.is_waiting() || arg2.is_waiting(),
      Expr::Value(_)            => false 
    }
  }

  fn eval(&mut self) {
    assert!(!self.waits() && !self.has_value());
    let new_expr = Expr::Value(match self.expr.clone() {
      Expr::Plus(Arg::Ready(v1),Arg::Ready(v2))     => v1+v2,
      Expr::Minus(Arg::Ready(v1),Arg::Ready(v2))    => v1-v2,
      Expr::Multiply(Arg::Ready(v1),Arg::Ready(v2)) => v1*v2,
      Expr::Divide(Arg::Ready(v1),Arg::Ready(v2))   => v1/v2,
      _                                             => panic!("Monkey cannot do arithmetic while still waiting")
    });
    self.expr = new_expr;
  }
}

impl Display for Monkey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.expr)
  }
}

fn prepare(lines: &Vec<String>) -> (Vec<Monkey>, HashMap<usize, Vec<usize>>, Option<usize>) {
  let mut monkey_do = vec![];
  let monkey_map: HashMap<String, usize> = lines.iter().enumerate().map(
    |(id,line)| {
      let split: Vec<&str> = line.split(": ").collect();
      assert_eq!(split.len(),2);
      monkey_do.push((split[0].to_string(), split[1].to_string()));
      (split[0].to_string(),id)
    }
  ).collect();
  let mut dependencies: HashMap<usize, Vec<usize>> = HashMap::new();
  let mut root_id_option = None;
  let monkeys = monkey_do.into_iter().enumerate().map(
    |(id, (name, mdo))| {
      if name == "root".to_string() { root_id_option = Some(id); }
      let split: Vec<&str> = mdo.split_whitespace().collect();
      Monkey { 
        id,
        name,
        expr: if split.len() == 1 {
                Expr::Value(split[0].parse::<Number>().unwrap())
              } else if split.len() == 3 {
                let (m1,m2) = (monkey_map[split[0]], monkey_map[split[2]]);
                if !dependencies.contains_key(&m1) { dependencies.insert(m1, vec![]); }
                if !dependencies.contains_key(&m2) { dependencies.insert(m2, vec![]); }
                dependencies.get_mut(&m1).unwrap().push(id);
                dependencies.get_mut(&m2).unwrap().push(id);
                match split[1] {
                  "+" => Expr::Plus(Arg::Wait(m1), Arg::Wait(m2)), 
                  "-" => Expr::Minus(Arg::Wait(m1), Arg::Wait(m2)), 
                  "*" => Expr::Multiply(Arg::Wait(m1), Arg::Wait(m2)), 
                  "/" => Expr::Divide(Arg::Wait(m1), Arg::Wait(m2)),
                   _  => panic!("Invalid operator found")
                }
              } else {
                panic!("malformed monkey")
              }
      }
    }
  ).collect();
  return (monkeys, dependencies, root_id_option);
}

fn screaming_monkeys(root_id: usize, monkeys: &mut Vec<Monkey>, dependencies: &mut HashMap<usize, Vec<usize>>) -> Number {
  let mut ready_monkeys: Vec<Monkey> = vec![];
  monkeys.iter().for_each(
    |monkey| match monkey.expr {
      Expr::Value(_) => ready_monkeys.push(monkey.clone()),
      _              => ()
    }
  );

  while !ready_monkeys.is_empty() {
    let screaming_monkey = ready_monkeys.pop().unwrap();
    if screaming_monkey.is_root(root_id) { break; }
    dependencies.remove(&screaming_monkey.id).unwrap().into_iter().for_each(
      |id| {
        let other_monkey = monkeys.get_mut(id).unwrap();
        other_monkey.listen(&screaming_monkey);
        if !other_monkey.waits() {
          other_monkey.eval();
          ready_monkeys.push(other_monkey.clone());
        }
      }
    );
  }
  assert!(ready_monkeys.is_empty());
  return if let Expr::Value(val) = monkeys[root_id].expr { val } else { panic!("wait, root monkey was not ready") };
}

fn one(input: &Input) -> String {
  let (mut monkeys, mut dependencies, root_id_option) = prepare(&input.lines);
  let root_id = root_id_option.unwrap();
  return screaming_monkeys(root_id, &mut monkeys, &mut dependencies).to_string();
}

fn two(input: &Input) -> String {
  let (mut monkeys, mut dependencies, root_id_option) = prepare(&input.lines);
  let root_id = root_id_option.unwrap();

  let (human_branch_root, monkey_branch_root) = monkeys.get_branches(root_id, &dependencies);
  let monkey_result = screaming_monkeys(monkey_branch_root, &mut monkeys, &mut dependencies).to_string();
  return solve_for_x(human_branch_root, monkey_result, &monkeys, &dependencies).to_string();
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
