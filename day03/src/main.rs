use std::{
  env,
  fs,
  io::{prelude::*},
  path,
};

enum Direction {
  Up,
  Left,
  Right,
  Down,
}

struct Movement {
  direction: Direction,
  steps: i32,
}

fn parse_movement(text : &str) -> Movement {
  let mut it = text.chars();
  Movement {
    direction : match it.next().unwrap() {
      'D' => Direction::Down,
      'U' => Direction::Up,
      'L' => Direction::Left,
      'R' => Direction::Right,
      _ => panic!("Invalid direction found"),
    },
    steps : it.as_str().parse::<i32>().unwrap()
  }
}

fn parse_line(text : &str) -> Vec<Movement> {
  text.trim().split(",")
  .map(|token| parse_movement(token))
  .collect()
}

fn parse_file(filename : impl AsRef<path::Path>) -> Vec<Vec<Movement>> {
  let mut file = fs::File::open(filename).expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
    .expect("Could not read file");
    contents.trim().split("\n").map(|line| parse_line(line)).collect()
}

#[derive(Copy, Clone)]
struct Point {
  x : i32,
  y : i32,
}

struct Line {
  p1 : Point,
  p2 : Point,
}

fn make_point(movement : &Movement, start : &Point) -> Point {
  match movement.direction {
    Direction::Down  => Point { x: start.x, y : start.y - movement.steps },
    Direction::Up    => Point { x: start.x, y : start.y + movement.steps },
    Direction::Left  => Point { x: start.x - movement.steps, y : start.y },
    Direction::Right => Point { x: start.x + movement.steps, y : start.y },
  }
}

fn make_wires_segments(wires_movements : Vec<Vec<Movement>>) -> Vec<Vec<Line>> {
  let mut wires = Vec::new();
  for wire_movements in wires_movements {
    let mut wire = Vec::new();
    let mut previous_point = Point{ x: 0, y: 0};
    for movement in wire_movements {
      let next_point = make_point(&movement, &previous_point);
      let line = Line {
        p1: previous_point,
        p2: next_point,
      };
      previous_point = next_point;
      wire.push(line);
    }
    wires.push(wire);
  }
  wires
}

fn between(r : i32, n1 : i32, n2 : i32) -> bool {
  (r < n1 && r > n2) || (r > n1 && r < n2)
}

fn get_intersection(segment1 : &Line, segment2 : &Line) -> Result<Point, &'static str> {
  if segment1.p1.x == 0 && segment2.p1.x == 0 && segment2.p1.y == 0 && segment2.p1.y == 0 {
    return Err("First segments")
  }
  if segment1.p1.x == segment1.p2.x
    && segment2.p1.y == segment2.p2.y
    && between(segment1.p1.x, segment2.p1.x, segment2.p2.x)
    && between(segment2.p1.y, segment1.p1.y, segment1.p2.y) {
      return Ok(Point{ x : segment1.p1.x, y: segment2.p1.y});
  } else if segment1.p1.y == segment1.p2.y
    && segment2.p1.x == segment2.p2.x
    && between(segment2.p1.x, segment1.p1.x, segment1.p2.x)
    && between(segment1.p1.y, segment2.p1.y, segment2.p2.y) {
    return Ok(Point{ x : segment2.p1.x, y: segment1.p1.y});
  }
  Err("Not intersection")
}

fn closest_intersection_distance(wires_segments : &Vec<Vec<Line>>) -> i32 {
  let mut closest = -1;
  for segment1 in &wires_segments[0] {
    for segment2 in &wires_segments[1] {
      match get_intersection(&segment1, &segment2) {
        Ok(intersection) => {
          let distance = intersection.x.abs() + intersection.y.abs();
          if closest == -1 || distance < closest {
            closest = distance;
          }
        },
        Err(_e) => (),
      }
    }
  }
  closest
}

fn segment_size(line : &Line) -> i32 {
  if line.p1.x == line.p2.x {
    return (line.p1.y - line.p2.y).abs()
  } else {
    return (line.p1.x - line.p2.x).abs()
  }
}

fn min_step_to_intersection(wires_segments : &Vec<Vec<Line>>) -> i32 {
  let mut min_steps = -1;
  let mut steps1 = 0;
  for segment1 in &wires_segments[0] {
    let mut steps2 = 0;
    for segment2 in &wires_segments[1] {
      match get_intersection(&segment1, &segment2) {
        Ok(intersection) => {
          let distance = steps1 + steps2
            + segment_size(&Line { p1 : segment1.p1, p2 : intersection })
            + segment_size(&Line { p1 : segment2.p1, p2 : intersection });
          if min_steps == -1 || distance < min_steps {
            min_steps = distance;
          }
        },
        Err(_e) => (),
      }
      steps2 += segment_size(&segment2);
    }
    steps1 += segment_size(&segment1);
  }
  min_steps
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let content = parse_file(filename);
  let wires_segments = make_wires_segments(content);
  let result1 = closest_intersection_distance(&wires_segments);
  println!("Part1: closest intersection: {}", result1);
  let result2 = min_step_to_intersection(&wires_segments);
  println!("Part2: minimum steps to intersection: {}", result2);
}
