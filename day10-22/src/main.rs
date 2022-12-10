use common::Input;
use common::args::Part;
use common::init::{startup, print, shutdown};
use log::{trace,info};
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug)]
enum Instruction { Noop, Addx(i32) }

impl Display for Instruction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match &self {
        Instruction::Noop    => write!(f, "noop"),
        Instruction::Addx(v) => write!(f, "addx {v}")
      }
  }
}

impl Instruction {
  fn apply(&self, x: &mut i32) {
    match &self {
      Instruction::Noop    => (),
      Instruction::Addx(v) => *x += v
    }
  }

  fn cycles(&self) -> i32 {
    match &self {
      Instruction::Noop    => 1,
      Instruction::Addx(_) => 2
    }
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.split_whitespace().collect::<Vec<&str>>()[..] {
        ["noop"]        => Ok(Instruction::Noop),
        ["addx", v_str] => Ok(Instruction::Addx(v_str.parse().unwrap())),
        _               => Err(())
      }
  }
}

fn prepare(lines: &Vec<String>) -> Vec<Instruction> {
  lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn one(input: &Input) -> String {
  let instructions = prepare(&input.lines);
  let first_cycle = 20;
  let nth_cycle   = 40;

  let (result, _, _) = instructions.iter().fold(
    (0, 1, 0),
    |(mut result, mut x, mut cycle) , inst| {
      let local_cycle = cycle%nth_cycle;
      let step = cycle/nth_cycle;
      let cycles_needed = inst.cycles();
      if local_cycle < first_cycle && first_cycle <= local_cycle+cycles_needed { 
        trace!("Cycle {cycle} mod {nth_cycle} is between {local_cycle} and {}: Multiply {first_cycle}+{step}*{nth_cycle} = {} by {x}", local_cycle+cycles_needed, {first_cycle}+{step}*{nth_cycle});
        result += (first_cycle+step*nth_cycle)*x;
      }
      cycle += cycles_needed;
      inst.apply(&mut x);
      (result, x, cycle)
    }
  );

  return result.to_string();
}

fn two(input: &Input) -> String {
  let instructions = prepare(&input.lines);
  let crt_width = 40;

  fn draw(crt: &mut String, crt_width: &usize, scan_pos: &mut i32, sprite_pos: &i32, cycles_needed: i32) {
    if cycles_needed > 0 {
      let local_scan_pos = *scan_pos % (*crt_width as i32);
      *crt = format!("{}{}{}", 
              *crt, 
              if local_scan_pos == 0 {"\n"} else {""},
              if *sprite_pos-1 <= local_scan_pos && local_scan_pos <= *sprite_pos+1 {"#"} else {"."}
            );
      *scan_pos += 1;
      draw(crt, crt_width, scan_pos, sprite_pos, cycles_needed-1);
    }
  }

  let (result, _, _) = instructions.iter().fold(
    ("".to_string(), 0, 1),
    |(mut crt, mut scan_pos, mut sprite_pos), inst| {
      draw(&mut crt, &crt_width, &mut scan_pos, &sprite_pos, inst.cycles());
      inst.apply(&mut sprite_pos);
      (crt, scan_pos, sprite_pos)
    }
  );

  return result;
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
