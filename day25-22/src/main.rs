use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;
use itertools::EitherOrBoth::{Left, Right, Both};
use itertools::Itertools;

type Int = i64;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Digit { MTwo = -2, MOne = -1, Zero = 0, One = 1, Two = 2 }

impl Digit {
  fn new(d: i8) -> Digit {
    match d {
     -2 => Digit::MTwo,
     -1 => Digit::MOne,
      0 => Digit::Zero,
      1 => Digit::One,
      2 => Digit::Two,
      _ => { panic!("Cannot create Digit from number: {d}"); }
    }
  }

  fn add(&self, other: &Digit) -> (Digit, Digit) {
    match (*self as i8) + (*other as i8) {
      sum if sum ==  4 => (Digit::MOne, Digit::One ),
      sum if sum ==  3 => (Digit::MTwo, Digit::One ),
      sum if sum == -3 => (Digit::Two , Digit::MOne),
      sum if sum == -4 => (Digit::One , Digit::MOne),
      sum              => (Digit::new(sum), Digit::Zero)
    }
  }

  fn add2(&self, o1: &Digit, o2: &Digit) -> (Digit, Digit) {
    match (*self as i8) + (*o1 as i8) + (*o2 as i8) {
      sum if sum ==  6 => (Digit::One , Digit::One ),
      sum if sum ==  5 => (Digit::Zero, Digit::One ),
      sum if sum ==  4 => (Digit::MOne, Digit::One ),
      sum if sum ==  3 => (Digit::MTwo, Digit::One ),
      sum if sum == -3 => (Digit::Two , Digit::MOne),
      sum if sum == -4 => (Digit::One , Digit::MOne),
      sum if sum == -5 => (Digit::Zero, Digit::MOne),
      sum if sum == -6 => (Digit::MOne, Digit::MOne),
      sum              => (Digit::new(sum), Digit::Zero)
    }
  }
}

impl Display for Digit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", 
      match self {
        Digit::MTwo => '=',
        Digit::MOne => '-',
        Digit::Zero => '0',
        Digit::One  => '1',
        Digit::Two  => '2'
      }
    )    
  } 
}

#[derive(Debug)]
struct SNAFU { digits: Vec<Digit> }

impl SNAFU {
  fn zero() -> SNAFU {
    SNAFU { digits: vec![Digit::new(0)] }
  }

  fn add(&self, other: &SNAFU) -> SNAFU {
    let (mut digits, carry) = self.digits.iter().rev().zip_longest(other.digits.iter().rev()).fold(
      (vec![], Digit::Zero),
      |(mut digits, carry), either_digit| {
        let (digit, c) = match either_digit {
          Left(l)   => l.add(&carry),
          Right(r)  => r.add(&carry),
          Both(l,r) => l.add2(&r,&carry),
        };
        digits.push(digit);
        (digits, c)
      }
    );
    if carry == Digit::One || carry == Digit::Two {
      digits.push(carry);
    }
    digits.reverse();
    SNAFU { digits }
  }


  fn to_int(&self) -> Int {
    self.digits.iter().rev().enumerate().fold(
      0, 
      |acc, (i,d)| {
        acc + (*d as Int)*num::pow(5,i)
      }
    )
  }

  fn sum(snafus: &Vec<SNAFU>) -> SNAFU {
    snafus.iter().fold(
      SNAFU::zero(), 
      |acc, snafu| {
        acc.add(snafu)
      }
    )
  }
}

impl Display for SNAFU {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", 
      self.digits.iter().fold(
        String::new(),
        |acc, digit| {
          format!("{acc}{digit}")
        }
      )
    )
  }
}

#[derive(Debug)]
struct ParseSNAFUError;

impl FromStr for SNAFU {
  type Err = ParseSNAFUError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let digit_results: Vec<Result<Digit, ParseSNAFUError>> = s.chars().map(
      |c| {
        match c {
          '=' => Ok(Digit::MTwo),
          '-' => Ok(Digit::MOne),
          '0' => Ok(Digit::Zero),
          '1' => Ok(Digit::One),
          '2' => Ok(Digit::Two),
           _  => Err(ParseSNAFUError)
        }
      }).collect();
    return if digit_results.iter().all(|d_res| d_res.is_ok()) { Ok(SNAFU { digits: digit_results.into_iter().map(|d_res| d_res.unwrap()).collect()}) } else { Err(ParseSNAFUError) };
  }
}

fn prepare(lines: &Vec<String>) -> Vec<SNAFU> {
  return lines.iter().map(|line| line.parse::<SNAFU>().unwrap()).collect();
}

fn one(input: &Input) -> String {
  let snafu_number = SNAFU::sum(&prepare(&input.lines));
  trace!("Result: {}", snafu_number.to_int());
  return snafu_number.to_string();
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