use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::time::Instant;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Inst { dir: Dir, arg: i32 }

#[derive(Debug)]
enum Dir { Forward, Up, Down }

fn lines_to_instructions(lines: &Vec<String>) -> Vec<Inst> {

  #[derive(Debug)]
  enum Reason { MalformedLine, InvalidDirection, MalformedInteger }

  impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", match &self {
        Reason::MalformedLine    => "malformed line",
        Reason::InvalidDirection => "invalid direction",
        Reason::MalformedInteger => "malformed integer"
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

  fn to_dir(s: &str) -> Result<Dir,ParseError> {
    match s.to_lowercase().as_str() {
      "forward" => Ok(Dir::Forward),
      "up"      => Ok(Dir::Up),
      "down"    => Ok(Dir::Down),
      _         => Err(ParseError { reason: Reason::InvalidDirection, input: s.to_string() })
    }
  }

  let instructions_results: Vec<Result<Inst,ParseError>> = lines.iter().map(
    |l| {
      let splits: Vec<&str> = l.split(" ").collect();
      match splits[..] {
        [dir_string,arg_string] => match to_dir(dir_string) {
                                     Ok(dir)    => 
                                        match arg_string.parse::<i32>() {
                                          Ok(arg) => Ok(Inst{dir: dir, arg: arg}),
                                          _       => Err(ParseError { input: l.to_string(), reason: Reason::MalformedInteger })
                                        },
                                     Err(error) => Err(error) 
                                   },
        _                       => Err(ParseError { input: l.to_string(), reason: Reason::MalformedLine })
      }
    }
  ).collect();

  if let Some(Err(err)) = instructions_results.iter().find(|inst| inst.is_err()) {
    panic!("Lines to Inst failed: {err}");
  }

  instructions_results.into_iter().map(|inst| inst.unwrap()).collect()
}

fn one(input: &Input) -> String {
  struct State { x: i32, y: i32 }

  let result = lines_to_instructions(&input.lines).into_iter().fold(State {x: 0, y:0 }, 
    | mut state, Inst { dir, arg } | {
      match dir {
        Dir::Forward => state.x += arg,
        Dir::Up      => state.y -= arg,
        Dir::Down    => state.y += arg
      };
      state
    }
  );

  (result.x * result.y).to_string()
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
