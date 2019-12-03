use std::{
  env,
  fs,
  io,
  io::{prelude::*},
  path
};

fn fuel_required_by_mass(mass : i32) -> i32 {
  mass / 3 - 2
}

fn fuel_required_by_mass_iterated(mass : i32) -> i32 {
  let mut fuel = 0;
  let mut mass_left = fuel_required_by_mass(mass);
  while mass_left > 0 {
    fuel += mass_left;
    mass_left = fuel_required_by_mass(mass_left);
  }
  fuel
}

fn total_fuel_required_part1(items : &Vec<i32>) -> i32 {
  items.iter().map(|x| fuel_required_by_mass(*x)).sum()
}

fn total_fuel_required_part2(items : &Vec<i32>) -> i32 {
  items.iter().map(|x| fuel_required_by_mass_iterated(*x)).sum()
}

fn parse_file(filename : impl AsRef<path::Path>) -> Vec<i32> {
  let file = fs::File::open(filename).expect("File not found");
  let buf = io::BufReader::new(file);
  buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .map(|l| l.parse::<i32>().expect("Could not parse number"))
      .collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  fuel_required_by_mass_iterated(1969);
  let filename = &args[1];
  println!("Loading file {}", filename);
  let masses = parse_file(filename);
  let total1 = total_fuel_required_part1(&masses);
  println!("Part1: fuel required: {}", total1);
  let total2 = total_fuel_required_part2(&masses);
  println!("Part2: fuel required: {}", total2);
}
