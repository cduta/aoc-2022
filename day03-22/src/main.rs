use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

#[derive(Debug,Clone,Copy)]
enum Reason { OddLineLength, InvalidItemId, InvalidItem, NotExactlyOneDuplicate }

impl fmt::Display for Reason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match &self {
      Reason::OddLineLength          => "odd line length",
      Reason::InvalidItemId          => "invalid item id",
      Reason::InvalidItem            => "invalid item",
      Reason::NotExactlyOneDuplicate => "not exactly one duplicate"
    })
  }
}

impl Error for Reason {}

#[derive(Debug)]
struct ParseError { reason: Reason, input: String }

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "failed to parse {}: {}", &self.input, &self.reason)
  }
}

impl Error for ParseError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.reason)
  }
}

type Group = Vec<HashSet<Item,RandomState>>;

fn format_group(group: &Group) -> String {
  group.iter().fold(
    "".to_string(),
    |acc,items| 
      format!("{acc} {}", format_items(items))
  ) 
}

#[derive(Debug,Clone,Copy)]
enum GroupReason { MalformedGroup }

impl fmt::Display for GroupReason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match &self {
      GroupReason::MalformedGroup => "malformed group"
    })
  }
}

impl Error for GroupReason {}

#[derive(Debug)]
struct GroupError { reason: GroupReason, group_string: String }

impl fmt::Display for GroupError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "failed to group {:?}: {}", &self.group_string, &self.reason)
  }
}

impl Error for GroupError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.reason)
  }
}

#[derive(Debug,Clone,Copy)]
enum BadgeReason { MoreThanOneBadge }

impl fmt::Display for BadgeReason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match &self {
      BadgeReason::MoreThanOneBadge => "more than one badge"
    })
  }
}

impl Error for BadgeReason {}

#[derive(Debug)]
struct BadgeError { reason: BadgeReason, badge_string: String }

impl fmt::Display for BadgeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "failed to find badge {:?}: {}", &self.badge_string, &self.reason)
  }
}

impl Error for BadgeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.reason)
  }
}

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Item {
  id: char
}

fn format_items(items: &HashSet<Item,RandomState>) -> String {
  items.iter().fold(
    "".to_string(), 
    |acc, item| 
      format!("{acc}{}", item.id).to_string()
  )
}

impl Item {
  fn new(id: char) -> Result<Item,ParseError> {
    let item_expr_result = Regex::new(r"^[a-zA-Z]$");
    if item_expr_result.is_err() {
      panic!("Malformed regular expression: {}", item_expr_result.unwrap_err());
    }
    let item_expr = item_expr_result.unwrap();

    if item_expr.is_match(id.to_string().as_str()) {
      Ok(Item {id: id})
    } else {
      Err(ParseError { reason: Reason::InvalidItemId, input: id.to_string()})
    }
  }

  fn to_priority(&self) -> Result<i32,ParseError> {
    match (&self).id as i32 {
      id@97..=122 => Ok(id-96),
      id@65..=90  => Ok(id-38),
      id          => Err(ParseError { reason: Reason::InvalidItem, input: id.to_string()})
    }
  }
}

#[derive(Debug)]
struct Backpack {
  compartments: Vec<Vec<Item>>
}

impl Backpack {
  fn new(parts: Vec<String>) -> Result<Backpack,ParseError> {
    let compartment_results: Vec<Result<Vec<Item>,ParseError>> = parts.iter().map(
      |compartment_string| {
        let item_results: Vec<Result<Item,ParseError>> = compartment_string.chars().map(|c| Item::new(c)).collect();

        if let Some(Err(err)) = item_results.iter().find(|l| l.is_err()) {
          Err(ParseError { reason: err.reason, input: err.input.clone() } )
        } else {
          Ok(item_results.into_iter().map(|i| i.unwrap()).collect())
        }
      }
    ).collect();


    if let Some(Err(err)) = compartment_results.iter().find(|l| l.is_err()) {
      Err(ParseError { reason: err.reason, input: err.input.clone() } )
    } else {
      Ok(Backpack { compartments: compartment_results.into_iter().map(|c| c.unwrap()).collect() })
    }
  }
}

fn prepare(lines: &Vec<String>) -> Vec<Backpack> {
  let backpack_result: Vec<Result<Backpack,ParseError>> = lines.iter().map(
    |line| 
      if line.len() % 2 == 0 {
        Backpack::new(vec!(line[..line.len()/2].to_string(), line[line.len()/2..].to_string()))
      } else {
        Err(ParseError { reason: Reason::OddLineLength, input: line.to_string() })
      }
  ).collect();

  if let Some(Err(err)) = backpack_result.iter().find(|l| l.is_err()) {
    panic!("Failed to turn all lines into backpacks: {err}");
  }

  backpack_result.into_iter().map(|b| b.unwrap()).collect()
}

fn one(input: &Input) -> String {
  let backpacks = prepare(&input.lines);

  if let Some(backpack) = backpacks.iter().find(|b| b.compartments.len() != 2) {
    panic!("Day One: There were not exactly two compartments in the backpack. Count: {}\n{backpack:?}",backpack.compartments.len());
  }

  let priority_results: Vec<Result<i32, ParseError>> = backpacks.into_iter().map(
    |backpack| {
      let c1: HashSet<&Item, RandomState> = backpack.compartments[0].iter().collect();
      let c2: HashSet<&Item, RandomState> = backpack.compartments[1].iter().collect();
      let duplicate: HashSet<&&Item, RandomState> = c1.intersection(&c2).collect();
      if duplicate.len() == 1 {
        (*duplicate.into_iter().next().unwrap()).to_priority()
      } else {
        Err(ParseError { reason: Reason::NotExactlyOneDuplicate, input: format!("{duplicate:?}") })
      }
    }
  ).collect();

  if let Some(Err(err)) = priority_results.iter().find(|p| p.is_err()) {
    panic!("Day One: Something went wrong when collecting priorities: {err}");
  }

  return priority_results.into_iter().fold(0, |acc,p| acc+p.unwrap()).to_string();
}

fn two(input: &Input) -> String {
  let ordered_items: Vec<HashSet<Item,RandomState>> = prepare(&input.lines).into_iter().map(
    |backpack| 
      backpack.compartments.into_iter().fold(
        HashSet::new(), 
        |acc,curr| {
          let hash_set: HashSet<Item, RandomState> = curr.into_iter().collect();
          acc.union(&hash_set).map(|i| *i).collect()
        })
  ).collect();
  
  let group_results: Vec<Result<Group,GroupError>> = ordered_items.into_iter().fold::<Vec<Result<Group,GroupError>>,_>(
    vec![],
    |mut acc, items| {
      let group_option: Option<&mut Result<Group,GroupError>> = acc.last_mut();
      match group_option {
        Some(group_result) => 
          match group_result {
            Ok(group) => 
              match group.len() {
                1|2 => group.push(items),
                3   => acc.push(Ok(vec![items])),
                _   => { let group_string = format_group(group); acc.push(Err(GroupError { reason: GroupReason::MalformedGroup, group_string: group_string })) }
              },
            Err(_) => {}
          },
        None       => acc.push(Ok(vec![items]))
      }
    return acc;
    }
  );

  if let Some(Err(err)) = group_results.iter().find(|g| g.is_err()) {
    panic!("Failed to group all backpacks: {err}");
  }

  let groups: Vec<Group> = group_results.into_iter().map(|g| g.unwrap()).collect();

  if let Some(group) = groups.iter().find(|g| g.len() != 3) {
    panic!("Failed to group all backpacks: Group was not of size 3. Instead: {}\nGroup: {}", group.len(), format_group(group));
  }

  let badge_results: Vec<Result<Item,BadgeError>> = groups.into_iter().map(
    |mut group| {
      let first = group.remove(0);
      let badge: HashSet<Item> = group.into_iter().fold(
        first,
        |badge: HashSet<Item>, items| {
          badge.intersection(&items).map(|b| *b).collect()
        }
      );
      match badge.len() {
        1 => Ok(badge.into_iter().next().unwrap()),
        n => Err(BadgeError { reason: BadgeReason::MoreThanOneBadge, badge_string: format!("Badges found: {n}") })
      }
    }
  ).collect();

  if let Some(Err(err)) = badge_results.iter().find(|b| b.is_err()) {
    panic!("Failed to find the correct badge: {err}");
  }

  let priority_badge_results: Vec<Result<i32,ParseError>> = badge_results.into_iter().map(|b| b.unwrap().to_priority()).collect();

  if let Some(Err(err)) = priority_badge_results.iter().find(|pb| pb.is_err()) {
    panic!("Failed to turn all badges into priorities: {err}");
  }

  return priority_badge_results.into_iter().map(|pb| pb.unwrap()).fold(0, |acc,pb| acc+pb ).to_string();
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
