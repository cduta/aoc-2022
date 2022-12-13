use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

#[derive(Debug)]
enum Token { BOpen, BClose, Comma, Num(i32) }

impl Token {
  fn tokenize(line: &String) -> VecDeque<Token> {
    let mut rest = line.chars().into_iter();
    let mut result = VecDeque::new();
    let mut skip = 0;
    while let Some(c) = rest.next() {
      if skip > 0 { skip -= 1; continue; }
      result.push_back(match c {
        '[' => Token::BOpen,
        ']' => Token::BClose,
        ',' => Token::Comma,
         v  => {
          let tail: String = rest.clone().take_while(|c| *c != ']' && *c != '[' && *c != ',').collect();
          skip = tail.len();
          Token::Num(format!("{v}{tail}").parse().unwrap())
         }
      });
    }
    return result;
  }
}

/*
  [[1],[2,3,4]]
  [[1],4]

  [   ,             ]       [   ,             ]                                     
    |        |                |        |                                     
   [ ] [   ,   ,   ]         [ ] [   ,   ,   ]                                     
    |    |   |   |            |    |   |   |                                     
    1    2   3   4            1    2   3   4                                     
                        => 
  [   ,   ]                 [   ,   ]                                              
    |   |                     |   |                                              
   [ ]  4                    [ ] [ ]                                                 
    |                         |   |                                          
    1                         1   4                                          
 */
#[derive(Clone, Debug)]
enum Tree { Empty, Leaf(i32), Node(Vec<Tree>) }

impl Tree {
  fn parse_tree(line: &String) -> Tree {
    fn parse_more(mut curr: Vec<Tree>, mut tokens: VecDeque<Token>) -> (Tree, VecDeque<Token>) {
      match tokens.pop_front() {
        Some(Token::Comma)  => 
          match tokens.pop_front() {
            Some(Token::BOpen)  => { let (tree, rest_tokens) = parse_open(tokens); curr.push(tree); parse_more(curr, rest_tokens) },
            Some(Token::BClose) => panic!("Got `,` then `]`"),
            Some(Token::Comma)  => parse_more(curr, tokens),
            Some(Token::Num(v)) => { curr.push(Tree::Leaf(v)); parse_more(curr, tokens) },
            None                => panic!("Token stream ended while `[` unresolved")
          },
        Some(Token::BClose) => (Tree::Node(curr), tokens),
        token               => panic!("Token stream must be a `,` or `]`: Got {token:?}")
      }
    }

    fn parse_open(mut tokens: VecDeque<Token>) -> (Tree, VecDeque<Token>) {
      match tokens.pop_front() {
        Some(Token::BOpen)  => { let (tree, rest_tokens) = parse_open(tokens); parse_more(vec![tree], rest_tokens) },
        Some(Token::BClose) => (Tree::Node(vec![]), tokens),
        Some(Token::Comma)  => panic!("`,` found where there should be none.\n Tokens left: {tokens:?}"),
        Some(Token::Num(v)) => parse_more(vec![Tree::Leaf(v)], tokens),
        None                => panic!("Token stream ended while `[` unresolved")
      }
    }

    fn parse(mut tokens: VecDeque<Token>) -> (Tree, VecDeque<Token>) {
      match tokens.pop_front() {
        Some(Token::BOpen)  => parse_open(tokens),
        Some(Token::BClose) => panic!("`]` found where there should be none.\n Tokens left: {tokens:?}"),
        Some(Token::Comma)  => panic!("`,` found where there should be none.\n Tokens left: {tokens:?}"),
        Some(Token::Num(v)) => (Tree::Leaf(v), tokens),
        None                => (Tree::Empty, tokens)
      }
    }

    let tokens = Token::tokenize(line);
    let (tree, rest) = parse(tokens);
    assert!(rest.is_empty());

    return tree;

  }
}

impl Display for Tree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Tree::Empty          => write!(f, "Empty Tree"),
      Tree::Leaf(v)        => write!(f, "{v}"),
      Tree::Node(children) => {
        write!(f, "[{}]", { 
          let list = children.iter().fold("".to_string(), |acc, child| format!("{acc},{child}"));
          let mut iter = list.chars(); 
          iter.next(); 
          iter.collect::<String>() 
        })
      } 
    }
  }
}

#[derive(Debug)]
struct Pair { left: Tree, right: Tree }

impl Pair {
  fn parse_pairs(lines: &Vec<String>) -> Vec<Pair> {
    let grouped_lines: Vec<Vec<String>> = lines.iter().fold(
      vec![vec![]], 
      |mut acc, line| {
        match line.as_str() {
          "" => acc.push(vec![]),
          _  => acc.last_mut().unwrap().push(line.to_string())
        };
        acc
      }
    );
    assert_eq!(grouped_lines.len() as f32, (lines.len()+1) as f32/3.0);

    return grouped_lines.into_iter().fold(
      vec![],
      |mut acc, lines| {
        assert_eq!(lines.len(), 2);
        acc.push(Pair { left: Tree::parse_tree(&lines[0]), right: Tree::parse_tree(&lines[1]) });
        acc
      }
    );
  }

  fn depth_correct(&mut self) {
    fn correct(left: &mut Tree, right: &mut Tree) {
      let mut done = false;
      while !done {
        done = true;
        match (&mut *left, &mut *right) {
          (Tree::Node(cl), Tree::Node(cr)) => {
            let (cl_iter, cr_iter) = (cl.iter_mut(), cr.iter_mut());
            cl_iter.zip(cr_iter).for_each(|(l,r)| correct(l,r));
          },
          (Tree::Node(cl), Tree::Leaf(vr)) => { *right = Tree::Node(vec![Tree::Leaf(*vr)]); done = cl.is_empty() },
          (Tree::Leaf(vl), Tree::Node(cr)) => { *left  = Tree::Node(vec![Tree::Leaf(*vl)]); done = cr.is_empty() },
          (_,_)                            => ()
        }
      }
    }

    correct(&mut (*self).left, &mut (*self).right);
  }

  fn check(&self) -> Option<bool> {
    fn helper(left: &Tree, right: &Tree) -> Option<bool> {
      match (left,right) {
        (Tree::Leaf(vl), Tree::Leaf(vr)) => 
          match vl.cmp(vr) {
            Ordering::Less    => Some(true),
            Ordering::Greater => Some(false), 
            Ordering::Equal   => None
          }
        (Tree::Node(cl), Tree::Node(cr)) => {
          if let Some(check) = cl.iter().zip(cr.iter()).fold(None, |acc, (l,r)| if acc.is_none() { helper(l,r) } else { acc }) {
            Some(check)
          } else {
            match cl.len().cmp(&cr.len()) {
              Ordering::Less    => Some(true),
              Ordering::Greater => Some(false), 
              Ordering::Equal   => None
            }
          }
        },
        (Tree::Empty, Tree::Empty)       => None,
        (Tree::Empty, _)                 => Some(true), 
        (_, Tree::Empty)                 => Some(false),
        _                                => panic!("Cannot check leaf and node")
      }
    }

    return helper(&self.left, &self.right);
  }
}

impl Display for Pair {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Node pair:\n{}\n{}", &self.left, &self.right)
  }
}

fn one(input: &Input) -> String {
  let mut pairs = Pair::parse_pairs(&input.lines);

  //trace!("{}", pairs.iter().fold("".to_string(), |acc, pair| format!("{acc}{pair}\n\n")));

  pairs.iter_mut().for_each(|pair| pair.depth_correct());  

  //trace!("Correcting!");
  //trace!("{}", pairs.iter().fold("".to_string(), |acc, pair| format!("{acc}{pair}\n\n")));

  return pairs.iter().enumerate().map(|(i,pair)| if let Some(true) = pair.check() {i+1} else {0} ).sum::<usize>().to_string();
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
