mod vm;

#[derive(Clone, PartialEq)]
enum PanelColor {
  Black = 0,
  White = 1,
}

enum Turn {
  Left = 0,
  Right = 1,
}

struct Direction {
  x: i32,
  y: i32,
}

#[derive(Hash, Clone)]
struct Coordinates {
  x: i32,
  y: i32,
}

impl std::cmp::PartialEq for Coordinates {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl std::cmp::Eq for Coordinates {}

struct Order {
  panel_color: PanelColor,
  turn: Turn,
}

struct Robot {
  direction: Direction,
  position: Coordinates,
  known_panels: std::collections::HashMap<Coordinates, PanelColor>,
  top_left: Coordinates,
  bottom_right: Coordinates,
  painted_panels: usize,
  vm: vm::VirtualMachine,
}

impl Robot {
  fn paint_panel(&mut self, panel_color: &PanelColor) {
    self
      .known_panels
      .insert(self.position.clone(), panel_color.clone());
  }

  fn execute(&mut self, order: &Order) -> vm::BigByte {
    if self.top_left.x > self.position.x {
      self.top_left.x = self.position.x;
    }
    if self.top_left.y > self.position.y {
      self.top_left.y = self.position.y;
    }
    if self.bottom_right.x < self.position.x {
      self.bottom_right.x = self.position.x;
    }
    if self.bottom_right.y < self.position.y {
      self.bottom_right.y = self.position.y;
    }
    self.paint_panel(&order.panel_color);
    self.painted_panels = self.known_panels.len();
    self.next_panel(&order.turn)
  }

  fn run(&mut self, start_color: PanelColor) {
    let mut next_input = start_color as vm::BigByte;
    while self.vm.is_running() {
      self.vm.push_input(next_input);
      self.vm.run().expect("VM run failed");
      if self.vm.is_running() {
        let order = get_order(&mut self.vm).expect("Malformed orders");
        next_input = self.execute(&order);
      }
    }
  }

  fn number_of_known_panels(&self) -> usize {
    self.painted_panels
  }

  fn next_panel(&mut self, turn: &Turn) -> vm::BigByte {
    self.rotate(turn);
    self.position.x += self.direction.x;
    self.position.y += self.direction.y;
    self
      .known_panels
      .entry(self.position.clone())
      .or_insert(PanelColor::Black)
      .clone() as vm::BigByte
  }

  fn rotate(&mut self, turn: &Turn) {
    match turn {
      Turn::Left => {
        self.direction = Direction {
          x: self.direction.y,
          y: -self.direction.x,
        }
      }
      Turn::Right => {
        self.direction = Direction {
          x: -self.direction.y,
          y: self.direction.x,
        }
      }
    }
  }

  fn draw(&self) {
    let line_size = (self.bottom_right.x - self.top_left.x) as usize;
    let mut index = Coordinates { x: 0, y: 0 };
    for y in self.top_left.y..self.bottom_right.y + 1 {
      let mut line = vec!['.'; line_size];
      index.y = y;
      for (i, x) in (self.top_left.x..self.bottom_right.x + 1).enumerate() {
        index.x = x;
        if self.known_panels.contains_key(&index) && self.known_panels[&index] == PanelColor::White
        {
          line[i] = '#';
        }
      }
      let line_str: String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }
}

fn create_robot(memory: &vm::Memory) -> Robot {
  Robot {
    direction: Direction { x: 0, y: -1 },
    position: Coordinates { x: 0, y: 0 },
    known_panels: std::collections::HashMap::new(),
    top_left: Coordinates { x: 0, y: 0 },
    bottom_right: Coordinates { x: 0, y: 0 },
    painted_panels: 0,
    vm: vm::create_vm(memory.clone()),
  }
}

fn get_panel_color(value: vm::BigByte) -> Result<PanelColor, &'static str> {
  match value {
    0 => Ok(PanelColor::Black),
    1 => Ok(PanelColor::White),
    _ => Err("Invalid PanelColor"),
  }
}

fn get_direction(value: vm::BigByte) -> Result<Turn, &'static str> {
  match value {
    0 => Ok(Turn::Left),
    1 => Ok(Turn::Right),
    _ => Err("Invalid Direction"),
  }
}

fn get_order(vm: &mut vm::VirtualMachine) -> Result<Order, &'static str> {
  Ok(Order {
    panel_color: match vm.pop_output() {
      Some(output) => get_panel_color(output)?,
      None => return Err("Expected an output for PanelColor but none were found"),
    },
    turn: match vm.pop_output() {
      Some(output) => get_direction(output)?,
      None => return Err("Expected an output for Direction but none were found"),
    },
  })
}

fn part1(memory: &vm::Memory) -> () {
  let before = std::time::Instant::now();
  let mut robot = create_robot(memory);
  robot.run(PanelColor::Black);
  println!("Part1: Elapsed time: {:.2?}", before.elapsed());
  println!("Part1: result: {}", robot.number_of_known_panels());
}

fn part2(memory: &vm::Memory) -> () {
  let before = std::time::Instant::now();
  let mut robot = create_robot(memory);
  robot.run(PanelColor::White);
  println!("Part2: Elapsed time: {:.2?}", before.elapsed());
  println!("Part2: result:");
  robot.draw();
}

fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let memory = vm::parse_memory_file(filename);
  part1(&memory);
  part2(&memory);
  println!("Total elapsed time: {:.2?}", before.elapsed());
}
