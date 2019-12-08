use std::{
  env,
  fs,
  io::{prelude::*},
  path,
  collections::VecDeque,
  collections::HashSet,
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

type Parameters = [Mode; 2];

struct VirtualMachine {
  memory : Vec<i32>,
  pointer : usize,
  inputs : VecDeque<i32>,
  halted : bool,
}

fn get_mode(value : i32) -> Result<Mode, &'static str> {
  match value {
    0 => Ok(Mode::Position),
    1 => Ok(Mode::Immediate),
    _ => Err("Invalid value")
  }
}

impl VirtualMachine {
  fn get_value(&self, parameters : &Parameters, arg_id : usize) -> Result<i32, &'static str> {
    let arg = self.memory[self.pointer + arg_id];
    match parameters[arg_id - 1] {
      Mode::Position => {
        if arg < 0 {
          return Err("Negative parameter is in position mode");
        } else if (arg as usize) >= self.memory.len() {
          return Err("Parameter is in position mode and out of range");
        }
        Ok(self.memory[arg as usize])
      },
      Mode::Immediate => Ok(arg),
    }
  }

  fn add(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let result_id = self.memory[self.pointer + 3] as usize;
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if result_id >= self.memory.len() {
      return Err("result_id out of range");
    }
    self.memory[result_id] = value_left + value_right;
    self.pointer += 4;
    Ok(())
  }

  fn mult(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let result_id = self.memory[self.pointer + 3] as usize;
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if result_id >= self.memory.len() {
      return Err("result_id out of range");
    }
    self.memory[result_id] = value_left * value_right;
    self.pointer += 4;
    Ok(())
  }

  fn store_input(&mut self) -> Result<(), &'static str> {
    match self.inputs.front() {
      Some(input) => {
        let ptr = self.memory[self.pointer + 1] as usize;
        self.memory[ptr] = *input;
        self.pointer += 2;
        self.inputs.pop_front();
        Ok(())
      },
      None => Err("No input found"),
    }
  }

  fn store_output(&mut self, output : &mut i32) {
    let output_ptr = self.memory[self.pointer + 1] as usize;
    *output = self.memory[output_ptr];
    self.pointer += 2;
  }

  fn jump_if_true(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if value_left != 0 {
      self.pointer = value_right as usize;
    } else {
      self.pointer += 3;
    }
    Ok(())
  }

  fn jump_if_false(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if value_left == 0 {
      self.pointer = value_right as usize;
    } else {
      self.pointer += 3;
    }
    Ok(())
  }

  fn less_than(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let result_id = self.memory[self.pointer + 3] as usize;
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if result_id >= self.memory.len() {
      return Err("result_id out of range");
    }
    self.memory[result_id] = (value_left < value_right) as i32;
    self.pointer += 4;
    Ok(())
  }

  fn equals(&mut self, parameters : &Parameters) -> Result<(), &'static str> {
    let result_id = self.memory[self.pointer + 3] as usize;
    let value_left = self.get_value(&parameters, 1)?;
    let value_right = self.get_value(&parameters, 2)?;
    if result_id >= self.memory.len() {
      return Err("result_id out of range");
    }
    self.memory[result_id] = (value_left == value_right) as i32;
    self.pointer += 4;
    Ok(())
  }

  fn get_method(&self) -> Result<(i32, Parameters), &'static str> {
    let id = self.memory[self.pointer];
    Ok(( id % 100,
      [ 
        get_mode(id / 100 % 10)?,
        get_mode(id / 1000 % 10)?,
      ]
    ))
  }

  fn has_exit(&self) {
    self.memory[self.pointer] == 99;
  }

  fn run(&mut self) -> Result<Option<i32>, &'static str> {
    self.halted = false;
    let mut output = 0;
    loop {
      let ( op_code, parameters ) = self.get_method()?;
      match op_code {
        1 => self.add(&parameters)?,
        2 => self.mult(&parameters)?,
        3 => self.store_input()?,
        4 => {
          self.store_output(&mut output);
          return Ok(Some(output));
        },
        5 => self.jump_if_true(&parameters)?,
        6 => self.jump_if_false(&parameters)?,
        7 => self.less_than(&parameters)?,
        8 => self.equals(&parameters)?,
        99 => {
          return Ok(None);
        },
        _ => return Err("Invalid instruction"),
      }
    }
  }
}

fn create_vm(memory : &Vec<i32>, inputs : VecDeque<i32>) -> VirtualMachine {
  VirtualMachine {
    memory : memory.clone(),
    pointer : 0,
    inputs : inputs,
    halted : true,
  }
}

fn create_cluster(memory : &Vec<i32>, inputs : &[i32; 5]) -> [VirtualMachine; 5] {
  [
    create_vm(memory, vec!(inputs[0]).into_iter().collect()),
    create_vm(memory, vec!(inputs[1]).into_iter().collect()),
    create_vm(memory, vec!(inputs[2]).into_iter().collect()),
    create_vm(memory, vec!(inputs[3]).into_iter().collect()),
    create_vm(memory, vec!(inputs[4]).into_iter().collect()),
  ]
}

/*
 * An absurd inneficient solution. Because looking for a permutation algorithm is too hard.
 */
fn update_settings(settings : &mut [i32; 5]) -> bool {
  let mut recycles : HashSet<i32> = HashSet::new();
  let mut highest_recycle = 0;
  for i in (0..settings.len()).rev() {
    if highest_recycle > settings[i] {
      let mut smallest = highest_recycle;
      for recycle in &recycles {
        if *recycle > settings[i] && *recycle < smallest {
          smallest = *recycle;
        }
      }
      recycles.insert(settings[i]);
      settings[i] = smallest;
      recycles.remove(&smallest);
      let mut sorted_recycles : Vec<i32> = recycles.iter().map(|&x| x).collect();
      sorted_recycles.sort();
      for j in 0..sorted_recycles.len() {
        settings[i+1+j] = sorted_recycles[j];
      }
      return true;
    } else {
      if settings[i] > highest_recycle {
        highest_recycle = settings[i];
      }
      recycles.insert(settings[i]);
    }
  }
  false
}

fn part1(content : &Vec<i32>) {
  let before = std::time::Instant::now();
  let mut settings = [0,1,2,3,4];
  let mut max_output = 0;
  while update_settings(&mut settings) {
    let mut output = 0;
    let mut cluster = create_cluster(content, &settings);
    for vm in &mut cluster {
      vm.inputs.push_back(output);
      match vm.run().expect("Failed to calculate") {
        Some(result) => output = result,
        None => panic!("Program did not return an output"),
      }
    }
    if output > max_output {
      max_output = output
    };
  }
  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part1: max output: {}", max_output);
}

fn part2(content : &Vec<i32>) {
  let before = std::time::Instant::now();
  let mut settings = [5,6,7,8,9];
  let mut max_output = 0;
  while update_settings(&mut settings) {
    let mut output = 0;
    let mut cluster = create_cluster(content, &settings);
    let mut all_vm_running = true;
    while all_vm_running {
      for vm in &mut cluster {
        vm.inputs.push_back(output);
        match vm.run().expect("Failed to calculate") {
          Some(result) => output = result,
          None => {
            all_vm_running = false;
            break;
          },
        }
    }
    }
    if output > max_output {
      max_output = output
    };
  }
  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part2: max output: {}", max_output);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let content = parse_file(filename);
  //part1(&content);
  part2(&content);
}
