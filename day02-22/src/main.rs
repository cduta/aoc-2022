use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum Reason { MalformedLine, NotAnAction }

impl fmt::Display for Reason {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match &self {
      Reason::MalformedLine => "malformed line",
      Reason::NotAnAction   => "not an action"
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

#[derive(Debug,PartialEq,Clone,Copy)]
enum Action { Rock, Paper, Scissors }

#[derive(Debug)]
struct Round { player: Action, opponent: Action }

impl Round {
/// Converts a character c into an action
/// Returns an NotAnAction error, if the character could not be parsed
  fn from_char(c: char) -> Result<Action,ParseError> {
    match c {
      'A'|'X' => Ok(Action::Rock),
      'B'|'Y' => Ok(Action::Paper),
      'C'|'Z' => Ok(Action::Scissors),
      _       => Err(ParseError { reason: Reason::NotAnAction, input: c.to_string()})
    }
  }

  fn from_decision_char(c: char, opponent: Action) -> Result<Action,ParseError> {
    match (c, opponent) {
      ('X', Action::Paper)
    | ('Y', Action::Rock)   
    | ('Z', Action::Scissors) => Ok(Action::Rock),
      ('X', Action::Scissors)
    | ('Y', Action::Paper)   
    | ('Z', Action::Rock)     => Ok(Action::Paper),
      ('X', Action::Rock)
    | ('Y', Action::Scissors)   
    | ('Z', Action::Paper)    => Ok(Action::Scissors),
      _                       => Err(ParseError { reason: Reason::NotAnAction, input: c.to_string()})
    }
  }
}

fn prepare(lines: &Vec<String>, to_round: fn(String) -> Result<Round,ParseError>) -> Vec<Round> {
  let round_lines = lines.clone();

  let round_results: Vec<Result<Round,ParseError>> = round_lines.into_iter().map(to_round).collect();

  if let Some(Err(err)) = round_results.iter().find(|l| l.is_err()) {
    panic!("{err}");
  }

  round_results.into_iter().map(|round| round.unwrap()).collect()
}

fn score(gameplan: Vec<Round>) -> i32 {
  gameplan.into_iter().fold(0, |acc, Round {player, opponent}| 
    acc 
      +
    match (player, opponent) {
      (Action::Rock    , Action::Scissors)
    | (Action::Scissors, Action::Paper) 
    | (Action::Paper   , Action::Rock)     => 6,
      _              if player == opponent => 3,
      _                                    => 0
    } 
      +
    match player {
      Action::Rock     => 1, 
      Action::Paper    => 2,
      Action::Scissors => 3
    } 
  )
}

fn one(input: &Input) -> String {
  score(prepare(&input.lines, 
    |line| 
      match line.chars().collect::<Vec<char>>()[..] {
        [opponent_char,' ',player_char] => match Round::from_char(player_char) {
          Ok(player_action) => match Round::from_char(opponent_char) {
            Ok(opponent_action) => Ok(Round { player  : player_action, 
                                              opponent: opponent_action }),
            Err(err)            => Err(err)},
          Err(err)          => Err(err)},
        _                               => Err(ParseError { reason: Reason::MalformedLine, input: line })
      }
    )
  ).to_string()
}

fn two(input: &Input) -> String {
  score(prepare(&input.lines, 
    |line| 
      match line.chars().collect::<Vec<char>>()[..] {
        [opponent_char,' ',player_char] => match Round::from_char(opponent_char) {
          Ok(opponent_action) => match Round::from_decision_char(player_char, opponent_action) {
            Ok(player_action)   => Ok(Round { player  : player_action, 
                                              opponent: opponent_action }),
            Err(err)            => Err(err)},
          Err(err)            => Err(err)},
        _                               => Err(ParseError { reason: Reason::MalformedLine, input: line }
        )
      }
    )
  ).to_string()
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
