use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use std::fmt;
use core::slice::Iter;

#[derive(Debug)]
struct File { name: String, size: i64 }

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "- {} (file, size={})", &self.name, &self.size)
  }
}

#[derive(Debug)]
struct Directory { name: String, subdirs: Vec<Directory>, files: Vec<File> }

impl Directory {
  fn new(name: String) -> Directory {
    Directory { name: name, subdirs: vec![], files: vec![] }
  }
}

impl fmt::Display for Directory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f,"- {} (dir){}{}", &self.name, &self.subdirs.iter().fold(
      String::new(),
      |acc, subdir| {
        format!("{acc}{}", subdir.to_string().lines().fold(String::new(), |acc, line| format!("{acc}\n  {line}")))
      }
    ), &self.files.iter().fold(
      "\n".to_string(),
      |acc, file| {
        format!("{acc}  {file}\n")
      }
    ))
  }
}

#[derive(Debug)]
struct Size { name: String, value: i64, subsizes: Vec<Size>}

impl Size {
  fn new(name: String) -> Size {
    return Size { name: name, value: 0, subsizes: vec![] }
  }
}

#[derive(Debug)]
enum Instruction {
  CD(String),
  LS,
  Dir(String),
  File(String,i64)
}

fn prepare(lines: &Vec<String>) -> Vec<Instruction> {
  lines.iter().map(
    |line| {
      match line.chars().next().unwrap() {
        '$'                                     => {
          match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [_,_,dir] => Instruction::CD(dir.to_string()),
            [_,_]           => Instruction::LS,
             _              => panic!("Could not parse command. Got: {line}")
          }
        }, 
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
          match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [size,name] => Instruction::File(name.to_string(), size.parse::<i64>().unwrap()),
             _                      => panic!("Could not parse file. Got: {line}")
          }
        },
         _                                      => {
          match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [_,name] => Instruction::Dir(name.to_string()), 
             _             => panic!("Could not parse directory. Got: {line}")
          }
         }  
      }
    }
  ).collect()
}

fn to_filesystem(mut instructions: Vec<Instruction>) -> (Directory, i64) {
  let first_inst = instructions.remove(0);
  let root: Directory = 
    match first_inst {
      Instruction::CD(ref name) => match name.as_str() {
        ".." => panic!("Tried to change dir into root. Got: {first_inst:?}"),
        name => { 
          Directory::new(name.to_string())
        }
      },
      _                         => panic!("Expected `cd` instruction. Got: {first_inst:?}")
    };
  let mut total_size = 0;
  let (ancestors, current): (Vec<Directory>, Directory) = instructions.iter().fold(
       (vec![], root),
      |(mut ancestors, mut current), inst| {
      match inst {
        Instruction::CD(name)        => {
          match name.as_str() {
            ".."    => {
              match ancestors.pop() {
                Some(mut parent) => {
                  parent.subdirs.push(current);
                  (ancestors, parent)
                },
                None         => panic!("Tried to move up one level at root")
              }
            },
            name    => {
              let child = if let Some(i) = current.subdirs.iter().position(|d| d.name == name) {
                current.subdirs.remove(i)
              } else {
                Directory::new(name.to_string())
              };
              ancestors.push(current);
              (ancestors, child)
            }
          }
        },
        Instruction::LS               => (ancestors, current),
        Instruction::Dir(name)        => {
          if current.subdirs.iter().find(|d| d.name == *name).is_none() {
            current.subdirs.push(Directory::new(name.to_string()));
          }
          (ancestors, current)
        },
        Instruction::File(name, size) =>  {
          if current.files.iter().find(|f| f.name == *name).is_none() {
            total_size += *size;
            current.files.push(File { name: name.to_string(), size: *size });
          }
          (ancestors, current)
        }
      }
    }
  );

  (ancestors.into_iter().rev().fold(current, |current, mut parent| {parent.subdirs.push(current); parent }), total_size)
}

fn one(input: &Input) -> String {
  let (filesystem, _) = to_filesystem(prepare(&input.lines));

  trace!("\n{filesystem}");

  let mut rest: Vec<(Size, &Directory, Iter<Directory>)> = vec![(Size::new(filesystem.name.to_string()), &filesystem, filesystem.subdirs.iter())];
  let mut sizes_found: Vec<i64> = vec![];

  while !rest.is_empty() {
    let (mut size, curr, mut children) = rest.pop().unwrap();
    match children.next() {
      Some(child) => {
        rest.push((size,curr,children));
        rest.push((Size::new(child.name.to_string()), child, child.subdirs.iter()));
      },
      None        => {
        size.value += size.value + 
                      size.subsizes.iter().fold(0, |acc,s| acc+s.value) +
                         curr.files.iter().fold(0, |acc,f| acc+f.size);
        if size.value <= 100000 { sizes_found.push(size.value) }
        if let Some((parent_size, _, _)) = rest.last_mut() {
          parent_size.subsizes.push(size);
        }
      }
    }
  }

  trace!("\n{sizes_found:?}");

  return sizes_found.iter().sum::<i64>().to_string();
}

fn two(input: &Input) -> String {
  let total_size               = 70000000;
  let needed_size              = 30000000;
  let (filesystem, used_space) = to_filesystem(prepare(&input.lines));
  let to_be_deleted            = needed_size - (total_size  - used_space);

  let mut rest: Vec<(Size, &Directory, Iter<Directory>)> = vec![(Size::new(filesystem.name.to_string()), &filesystem, filesystem.subdirs.iter())];
  let mut delete_this_option: Option<(String, i64)> = None;

  while !rest.is_empty() {
    let (mut size, curr, mut children) = rest.pop().unwrap();
    match children.next() {
      Some(child) => {
        rest.push((size,curr,children));
        rest.push(
          (Size::new(format!("{}/{}", rest.iter().fold(
            "".to_string(), 
            |acc, (_,dir,_)| 
              if dir.name != "/".to_string() {format!("{acc}/{}", dir.name)} else {acc}
            ), child.name)
          ), child, child.subdirs.iter())
        );
      },
      None        => {
        size.value += size.value + 
                      size.subsizes.iter().fold(0, |acc,s| acc+s.value) +
                         curr.files.iter().fold(0, |acc,f| acc+f.size);
        if size.value >= to_be_deleted {
          delete_this_option = match delete_this_option {
            Some((delete_name, delete_size)) if delete_size <= size.value => Some((delete_name, delete_size)),
            _                                                             => Some((size.name.to_string(), size.value)) };
        }
        if let Some((parent_size, _, _)) = rest.last_mut() {
          parent_size.subsizes.push(size);
        } 
      }
    }
  }

  return if let Some((delete_name, delete_size)) = delete_this_option {
    trace!("Delete directory {delete_name} (size: {delete_size})");
    delete_size.to_string()
  } else {
    "N/A".to_string()
  }
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
