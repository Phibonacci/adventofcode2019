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

fn add(content : &mut Vec<i32>, id : usize) -> Result<(), &'static str> {
  let result_id = content[id + 3] as usize;
  let id_left = content[id + 1] as usize;
  let id_right = content[id + 2] as usize;
  if result_id >= content.len() || id_left >= content.len() || id_right >= content.len() {
    return Err("Invalid input: out of range");
  }
  content[result_id] = content[id_left] + content[id_right];
  Ok(())
}

fn mult(content : &mut Vec<i32>, id : usize) -> Result<(), &'static str> {
  let result_id = content[id + 3] as usize;
  let id_left = content[id + 1] as usize;
  let id_right = content[id + 2] as usize;
  if result_id >= content.len() || id_left >= content.len() || id_right >= content.len() {
    return Err("Invalid input: out of range");
  }
  content[result_id] = content[id_left] * content[id_right];
  Ok(())
}

fn calculate(content : &mut Vec<i32>, noun : i32, verb : i32) -> Result<i32, &'static str> {
  content[1] = noun;
  content[2] = verb;
  let mut id = 0;
  while id + 3 < content.len() {
    match content[id] {
      1 => match add(content, id) {
        Err(e) => return Err(e),
        _ => (),
      },
      2 => match mult(content, id) {
        Err(e) => return Err(e),
        _ => (),
      },
      99 => return Ok(content[0]),
      _ => return Err("Invalid instruction"),
    }
    id += 4;
  }
  return Err("No exit code found")
}

fn find19690720(content : Vec<i32>) -> Result<i32, &'static str> {
  let mut noun = 1;
  let mut verb = 1;
  let mut content_copy = content.clone();
  loop {
    match calculate(&mut content_copy, noun, verb) {
      Ok(result) => match result {
        n if n < 19690720 => {
          verb += 1;
        },
        19690720 => return Ok(100 * noun + verb),
        n if n > 19690720 => {
          noun += 1;
          verb = 1;
        },
        _ => return Err("Impossible happened"),
      },
      Err(_e) => {
        noun += 1;
        verb = 1;
      },
    }
    content_copy = content.clone();
  }
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
  let result = calculate(&mut content_copy, 12, 2).expect("Failed to calculate");
  println!("Part1: value left at position 0: {}", result);
  let result = find19690720(content).unwrap();
  println!("Part2: value left at position 0: {}", result);
}
