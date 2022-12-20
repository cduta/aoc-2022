use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

type Int = i64;

#[derive(Clone)]
struct LinkedListElement { 
  id: usize, value: Int, 
  next: Option<Rc<RefCell<LinkedListElement>>>, 
  prev: Option<Rc<RefCell<LinkedListElement>>>
}

impl LinkedListElement {
  fn new(id: usize, value: Int) -> LinkedListElement { LinkedListElement { id, value, prev: None, next: None } }
}

#[derive(Clone)]
struct LinkedList {
  first : Option<Rc<RefCell<LinkedListElement>>>,
  last  : Option<Rc<RefCell<LinkedListElement>>>,
  length: usize,
  max_id: usize
}

impl LinkedList { 
  fn new() -> LinkedList { LinkedList { first: None, last: None, length: 0, max_id: 0 } } 
  fn insert(&mut self, value: Int, at: usize) {
    assert!(at <= self.length);
    self.max_id += 1;
    let element = Rc::new(RefCell::new(LinkedListElement::new(self.max_id, value)));
    match (&self.first, &self.last) {
      (Some(first),Some(last)) => {
        todo!()
      },
      (None,None)        => {
        (*element).borrow_mut().next = Some(Rc::clone(&element));
        (*element).borrow_mut().prev = Some(Rc::clone(&element));
        self.first = Some(Rc::clone(&element));
        self.last  = Some(element);
      },
      _                  => panic!("Linked list is in invalid state")
    };
    self.length += 1;
  }

  fn move_by_value(&mut self, id: usize) {
    todo!()
  }

  fn get(&self, index: usize) -> Int {
    todo!()
  }

  fn index(&self, value: Int) -> usize {
    todo!()
  }
}

fn prepare(lines: &Vec<String>, key: Int) -> LinkedList {
  let mut list = LinkedList::new();

  lines.iter().for_each(
    |line| {
      list.insert(line.parse::<Int>().unwrap()*key as Int, list.length)
    }
  );

  return list;
}

fn decrypt(mut list: LinkedList, mixes: usize) -> Int {
  for _ in 0..mixes {
    for id in 1..=list.length {
      list.move_by_value(id);
    }
  }

  return vec![1000,2000,3000].into_iter().map(
    |shift| {
      list.get((list.index(0) + shift) % list.length)
    }
  ).sum();
}

fn one(input: &Input) -> String {
  let key = 0;
  let mut list = prepare(&input.lines, key);
  return decrypt(list, 1).to_string();
}

fn two(input: &Input) -> String {
  let key = 811589153;
  let mut list = prepare(&input.lines, key);
  return decrypt(list, 1).to_string();
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
