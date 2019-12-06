use std::{
  env,
  fs,
  io::{prelude::*},
  path,
};


fn parse_file(filename : impl AsRef<path::Path>) -> Vec<i32> {
  let mut file = fs::File::open(filename).expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Could not read file");
  contents.trim().split(",")
    .map(|l| l.parse::<i32>().expect("Could not parse number"))
    .collect()
}

enum Mode {
  Position = 0,
  Immediate = 1,
}

struct Parameters {
  one : Mode,
  two : Mode,
}

fn get_value(content : &Vec<i32>, mode : &Mode, parameter : i32) -> Result<i32, &'static str> {
  match mode {
    Mode::Position => {
      if parameter < 0 {
        return Err("Negative parameter is in position mode");
      } else if (parameter as usize) >= content.len() {
        return Err("Parameter is in position mode and out of range");
      }
      Ok(content[parameter as usize])
    },
    Mode::Immediate => Ok(parameter),
  }
}

fn add(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let result_id = content[*id + 3] as usize;
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if result_id >= content.len() {
    return Err("result_id out of range");
  }
  content[result_id] = value_left + value_right;
  *id += 4;
  Ok(())
}

fn mult(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let result_id = content[*id + 3] as usize;
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if result_id >= content.len() {
    return Err("result_id out of range");
  }
  content[result_id] = value_left * value_right;
  *id += 4;
  Ok(())
}

fn store_input(content : &mut Vec<i32>, input : i32, id: &mut usize) {
  let ptr = content[*id + 1] as usize;
  content[ptr] = input;
  *id += 2;
}

fn store_output(content : &mut Vec<i32>, id: &mut usize, output : &mut i32) {
  let ptr = content[*id + 1] as usize;
  *output = content[ptr];
  *id += 2;
}

fn jump_if_true(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if value_left != 0 {
    *id = value_right as usize;
  } else {
    *id += 3;
  }
  Ok(())
}

fn jump_if_false(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if value_left == 0 {
    *id = value_right as usize;
  } else {
    *id += 3;
  }
  Ok(())
}

fn less_than(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let result_id = content[*id + 3] as usize;
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if result_id >= content.len() {
    return Err("result_id out of range");
  }
  content[result_id] = (value_left < value_right) as i32;
  *id += 4;
  Ok(())
}

fn equals(parameters : &Parameters, content : &mut Vec<i32>, id : &mut usize) -> Result<(), &'static str> {
  let result_id = content[*id + 3] as usize;
  let value_left = get_value(&content, &parameters.one, content[*id + 1])?;
  let value_right = get_value(&content, &parameters.two, content[*id + 2])?;
  if result_id >= content.len() {
    return Err("result_id out of range");
  }
  content[result_id] = (value_left == value_right) as i32;
  *id += 4;
  Ok(())
}

fn get_mode(value : i32) -> Result<Mode, &'static str> {
  match value {
    0 => Ok(Mode::Position),
    1 => Ok(Mode::Immediate),
    _ => Err("Invalid value")
  }
}

fn get_method(id : i32) -> Result<(i32, Parameters), &'static str> {
  Ok(( id % 100,
    Parameters {
      one: get_mode(id / 100 % 10)?,
      two: get_mode(id / 1000 % 10)?,
    }
  ))
}

fn calculate(content : &mut Vec<i32>, input : i32) -> Result<i32, &'static str> {
  let mut id = 0;
  let mut output = -1;
  while id < content.len() {
    let ( op_code, parameters ) = get_method(content[id])?;
    match op_code {
      1 => add(&parameters, content, &mut id)?,
      2 => mult(&parameters, content, &mut id)?,
      3 => store_input(content, input, &mut id),
      4 => store_output(content, &mut id, &mut output),
      5 => jump_if_true(&parameters, content, &mut id)?,
      6 => jump_if_false(&parameters, content, &mut id)?,
      7 => less_than(&parameters, content, &mut id)?,
      8 => equals(&parameters, content, &mut id)?,
      99 => return Ok(output),
      _ => return Err("Invalid instruction"),
    }
  }
  return Err("No exit code found")
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let content = parse_file(filename);
  let mut content_copy = content.clone();
  let result1 = calculate(&mut content_copy, 1).expect("Failed to calculate");
  println!("Part1: output: {}", result1);
  let mut content_copy = content.clone();
  let result2 = calculate(&mut content_copy, 5).expect("Failed to calculate");
  println!("Part2: output: {}", result2);
}
