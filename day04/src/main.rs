use std::{
  env,
  fs,
  io::{prelude::*},
  path,
};

fn is_valid_password(password : &Vec<i32>, twin : bool) -> Result<bool, &'static str> {
  let mut previous_digit = -1;
  let mut found_duplicate = false;
  let mut duplicate_size = 1;
  let mut found_twin = false;
  for digit in password {
    if *digit < 0 || *digit > 9 {
      return Err("Invalid digit");
    }
    match previous_digit {
      -1 => previous_digit = *digit,
      n if n < *digit => {
        previous_digit = *digit;
        if duplicate_size == 2 {
          found_twin = true;
        }
        duplicate_size = 1;
      },
      n if n > *digit => return Ok(false),
      n if n == *digit => {
        found_duplicate = true;
        duplicate_size += 1;
      },
      _ => return Err("Impossible happened"),
    }
  }
  if duplicate_size == 2 {
    found_twin = true;
  }
  Ok(if twin { found_duplicate && found_twin} else {found_duplicate})
}

fn increment_password(password : &mut Vec<i32>) {
  let mut i = (password.len() - 1) as i32;
  while i >= 0 {
    if password[i as usize] < 9 {
      password[i as usize] += 1;
      break;
    } else {
      password[i as usize] = 0;
    }
    i -= 1;
  }
}

fn get_valid_password_count(start: &Vec<i32>, end : &Vec<i32>, twin : bool) -> i32 {
  let mut current = start.clone();
  let mut valid_count = 0;
  while current != *end {
    if is_valid_password(&current, twin).unwrap() {
       valid_count += 1;
    }
    increment_password(&mut current);
  }
  valid_count
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    panic!("Not enough arguments");
  }
  let start = args[1].chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
  let end = args[2].chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
  let result1 = get_valid_password_count(&start, &end, false);
  println!("Part1: number of valid password: {}", result1);
  let result2 = get_valid_password_count(&start, &end, true);
  println!("Part2: number of valid password: {}", result2);
}
