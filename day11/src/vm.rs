pub type BigByte = i64;

pub type Memory = std::collections::HashMap<BigByte, BigByte>;

pub fn parse_memory_file(filename: impl AsRef<std::path::Path>) -> Memory {
	use std::io::prelude::*;
	let mut file = std::fs::File::open(filename).expect("File not found");
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.expect("Could not read file");
	let mut memory = Memory::new();
	let mut index: BigByte = 0;
	for string_number in contents.trim().split(",") {
		let number = string_number
			.parse::<BigByte>()
			.expect("Could not parse number");
		memory.insert(index, number);
		index += 1;
	}
	memory
}

pub type Input = std::collections::LinkedList<BigByte>;
pub type Output = std::collections::LinkedList<BigByte>;

enum Mode {
	Position = 0,
	Immediate = 1,
	Relative = 2,
}

type Parameters = [Mode; 3];

pub struct VirtualMachine {
	memory: Memory,
	pointer: BigByte,
	inputs: Input,
	outputs: Output,
	relative_base: BigByte,
}

fn get_mode(value: BigByte) -> Result<Mode, &'static str> {
	match value {
		0 => Ok(Mode::Position),
		1 => Ok(Mode::Immediate),
		2 => Ok(Mode::Relative),
		_ => Err("Invalid value"),
	}
}

impl VirtualMachine {
	fn get_byte(&self, pointer: BigByte) -> BigByte {
		if self.memory.contains_key(&pointer) {
			self.memory[&pointer]
		} else {
			0
		}
	}

	fn set_byte(&mut self, pointer: BigByte, value: BigByte) {
		self.memory.insert(pointer, value);
	}

	fn get_value(&self, parameters: &Parameters, arg_id: BigByte) -> Result<BigByte, &'static str> {
		let arg = self.get_byte(self.pointer + arg_id);
		match parameters[arg_id as usize - 1] {
			Mode::Position => {
				if arg < 0 {
					return Err("Negative parameter in position mode");
				}
				Ok(self.get_byte(arg))
			}
			Mode::Immediate => Ok(arg),
			Mode::Relative => {
				let pointer = self.relative_base + arg;
				if pointer < 0 {
					return Err("Invalid relative pointer < 0");
				}
				Ok(self.get_byte(pointer))
			}
		}
	}

	fn get_result_pointer(
		&self,
		parameters: &Parameters,
		arg_id: BigByte,
	) -> Result<BigByte, &'static str> {
		let arg = self.get_byte(self.pointer + arg_id);
		match parameters[arg_id as usize - 1] {
			Mode::Position => {
				if arg < 0 {
					return Err("Negative parameter in position mode");
				}
				Ok(arg)
			}
			Mode::Immediate => Err("Result cannot have immediate parameter"),
			Mode::Relative => {
				let pointer = self.relative_base + arg;
				if pointer < 0 {
					Err("Invalid relative pointer < 0")
				} else {
					Ok(pointer)
				}
			}
		}
	}

	fn set_result(
		&mut self,
		parameters: &Parameters,
		arg_id: BigByte,
		result: BigByte,
	) -> Result<(), &'static str> {
		let ptr = self.get_result_pointer(parameters, arg_id)?;
		self.set_byte(ptr, result);
		Ok(())
	}

	fn add(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		self.set_result(&parameters, 3, value_left + value_right)?;
		self.pointer += 4;
		Ok(())
	}

	fn mult(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		self.set_result(&parameters, 3, value_left * value_right)?;
		self.pointer += 4;
		Ok(())
	}

	fn store_input(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		match self.inputs.pop_front() {
			Some(input) => {
				// so Rust do not believe I am sharing it with a mutable reference of self
				self.set_result(&parameters, 1, input)?;
				self.pointer += 2;
				Ok(())
			}
			None => Err("No input found"),
		}
	}

	fn store_output(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let output = self.get_value(&parameters, 1)?;
		self.outputs.push_back(output);
		self.pointer += 2;
		Ok(())
	}

	fn jump_if_true(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		if value_left != 0 {
			self.pointer = value_right;
		} else {
			self.pointer += 3;
		}
		Ok(())
	}

	fn jump_if_false(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		if value_left == 0 {
			self.pointer = value_right;
		} else {
			self.pointer += 3;
		}
		Ok(())
	}

	fn less_than(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		self.set_result(&parameters, 3, (value_left < value_right) as BigByte)?;
		self.pointer += 4;
		Ok(())
	}

	fn equals(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		let value_left = self.get_value(&parameters, 1)?;
		let value_right = self.get_value(&parameters, 2)?;
		self.set_result(&parameters, 3, (value_left == value_right) as BigByte)?;
		self.pointer += 4;
		Ok(())
	}

	fn update_relative_base(&mut self, parameters: &Parameters) -> Result<(), &'static str> {
		self.relative_base += self.get_value(&parameters, 1)?;
		self.pointer += 2;
		Ok(())
	}

	fn get_method(&self) -> Result<(BigByte, Parameters), &'static str> {
		let id = self.get_byte(self.pointer);
		Ok((
			id % 100,
			[
				get_mode(id / 100 % 10)?,
				get_mode(id / 1000 % 10)?,
				get_mode(id / 10000 % 10)?,
			],
		))
	}

	pub fn run(&mut self) -> Result<(), &'static str> {
		loop {
			let (op_code, parameters) = self.get_method()?;
			match op_code {
				1 => self.add(&parameters)?,
				2 => self.mult(&parameters)?,
				3 => match self.store_input(&parameters) {
					Err(_) => return Ok(()),
					_ => (),
				},
				4 => self.store_output(&parameters)?,
				5 => self.jump_if_true(&parameters)?,
				6 => self.jump_if_false(&parameters)?,
				7 => self.less_than(&parameters)?,
				8 => self.equals(&parameters)?,
				9 => self.update_relative_base(&parameters)?,
				99 => {
					return Ok(());
				}
				_ => return Err("Invalid instruction"),
			}
		}
	}

	pub fn pop_output(&mut self) -> Option<BigByte> {
		self.outputs.pop_front()
	}

	pub fn is_running(&mut self) -> bool {
		self.memory[&self.pointer] != 99
	}

	pub fn push_input(&mut self, input: BigByte) {
		self.inputs.push_back(input);
	}
}

pub fn create_vm(memory: Memory) -> VirtualMachine {
	VirtualMachine {
		memory: memory,
		pointer: 0,
		inputs: Input::new(),
		outputs: Output::new(),
		relative_base: 0,
	}
}
