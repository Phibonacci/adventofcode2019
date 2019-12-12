extern crate ordered_float;

use std::{collections::BTreeMap, collections::HashSet, env};

#[derive(Hash, Clone, PartialEq)]
struct Pos {
	x: i32,
	y: i32,
}

impl std::cmp::Ord for Pos {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self == other {
			std::cmp::Ordering::Equal
		} else {
			use ordered_float::OrderedFloat;
			OrderedFloat(direction_to_angle(self)).cmp(&OrderedFloat(direction_to_angle(other)))
		}
	}
}

impl std::cmp::PartialOrd for Pos {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl std::cmp::Eq for Pos {}

struct Asteroid {
	pos: Pos,
	distance: f32,
}

fn parse_file(filename: impl AsRef<std::path::Path>) -> Vec<Pos> {
	let before = std::time::Instant::now();
	use std::io::prelude::*;
	let file = std::fs::File::open(filename).expect("File not found");
	let buf = std::io::BufReader::new(file);
	let mut asteroids = Vec::new();
	let mut y = 0;
	for line in buf.lines().map(|l| l.expect("Could not parse line")) {
		let mut x = 0;
		for c in line.chars() {
			if c == '#' {
				asteroids.push(Pos { x: x, y: y });
			}
			x += 1;
		}
		y += 1;
	}
	println!("Parsing: elapsed time: {:.2?}", before.elapsed());
	asteroids
}

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Recursive_version_in_C
fn get_gcd(u: u32, v: u32) -> u32 {
	// A - simple cases (termination)
	if u == v {
		u
	} else if u == 0 {
		v
	} else if v == 0 {
		u
	// B - look for factors of 2
	} else if !u & 1 != 0 {
		// u is even
		if v & 1 != 0 {
			// v is odd
			get_gcd(u >> 1, v)
		} else {
			// both u and v are even
			get_gcd(u >> 1, v >> 1) << 1
		}
	} else if !v & 1 != 0 {
		// u is odd, v is even
		get_gcd(u, v >> 1)
	// C - reduce larger argument
	} else if u > v {
		get_gcd((u - v) >> 1, v)
	} else {
		get_gcd((v - u) >> 1, u)
	}
}

fn get_direction(source: &Pos, target: &Pos) -> Pos {
	let vector = Pos {
		x: target.x - source.x,
		y: target.y - source.y,
	};
	let gcd = get_gcd(vector.x.abs() as u32, vector.y.abs() as u32);
	Pos {
		x: vector.x / gcd as i32,
		y: vector.y / gcd as i32,
	}
}

fn get_asteroids_in_range_count(asteroids: &Vec<Pos>, source: &Pos) -> usize {
	let mut asteroids_in_range = HashSet::new();
	for target in asteroids {
		if target == source {
			continue;
		}
		let direction = get_direction(source, target);
		asteroids_in_range.insert(direction);
	}
	asteroids_in_range.len()
}

fn part1(asteroids: &Vec<Pos>) -> Pos {
	let before = std::time::Instant::now();
	let mut max_asteroids_in_range = 0;
	let mut station = &asteroids[0];
	for asteroid in asteroids {
		let asteroids_in_range = get_asteroids_in_range_count(asteroids, asteroid);
		if asteroids_in_range > max_asteroids_in_range {
			max_asteroids_in_range = asteroids_in_range;
			station = asteroid;
		}
	}
	println!("Part1: Elapsed time: {:.2?}", before.elapsed());
	println!("Part1: max asteroids in range: {}", max_asteroids_in_range);
	station.clone()
}

fn get_vector_length(vector: &Pos) -> f32 {
	((vector.x.pow(2) + vector.y.pow(2)) as f32).sqrt()
}

fn get_distance(source: &Pos, target: &Pos) -> f32 {
	let vector = Pos {
		x: target.x - source.x,
		y: target.y - source.y,
	};
	get_vector_length(&vector)
}

fn direction_to_angle(direction: &Pos) -> f32 {
	let rad_angle = (-direction.x as f32).atan2(-direction.y as f32);
	let angle_part1 = if rad_angle > 0.0 {
		rad_angle
	} else {
		(2.0 * std::f32::consts::PI + rad_angle)
	};
	angle_part1 * 360.0 / (2.0 * std::f32::consts::PI)
}

fn part2(asteroids_pos: &Vec<Pos>, station_pos: &Pos) {
	let before = std::time::Instant::now();

	use ordered_float::OrderedFloat; // WTF Rust. I mean, seriously, are you out of your mind?
	let mut map: BTreeMap<Pos, Vec<Asteroid>> = BTreeMap::new();
	for asteroid_pos in asteroids_pos {
		if asteroid_pos == station_pos {
			continue;
		}
		let direction = get_direction(station_pos, asteroid_pos);
		map.entry(direction).or_default().push(Asteroid {
			pos: asteroid_pos.clone(),
			distance: get_distance(station_pos, asteroid_pos),
		});
	}
	for (_, asteroids) in &mut map {
		asteroids.sort_by(|a, b| OrderedFloat(b.distance).cmp(&OrderedFloat(a.distance)));
	}
	let mut found_asteroid = true;
	let mut asteroids_destroyed = 0;
	let mut search_target = None;
	while found_asteroid && asteroids_destroyed < 200 {
		found_asteroid = false;
		for (_, asteroids) in &mut map.iter_mut().rev() {
			match asteroids.pop() {
				Some(asteroid) => {
					asteroids_destroyed += 1;
					if asteroids_destroyed == 200 {
						search_target = Some(asteroid);
						break;
					}
					found_asteroid = true;
				}
				None => continue,
			}
		}
	}
	let result;
	match search_target {
		None => panic!("200th asteroid not pulverised"),
		Some(asteroid) => result = asteroid.pos.x * 100 + asteroid.pos.y,
	}
	println!("Part2: Elapsed time: {:.2?}", before.elapsed());
	println!("Part2: 100*x+y of 200th asteroid pulverised: {}", result);
}

fn main() {
	let before = std::time::Instant::now();
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		panic!("Not enough arguments");
	}
	let filename = &args[1];
	println!("Loading file {}", filename);
	let content = parse_file(filename);
	let station = part1(&content);
	part2(&content, &station);
	println!("Total elapsed time: {:.2?}", before.elapsed());
}
