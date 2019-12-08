
use std::{
  env,
  fs,
  io::{prelude::*},
  path,
};

type Pixel = u8;
type Line = Vec<Pixel>;
type Layer = Vec<Line>;
type Image = Vec<Layer>;

const IMAGE_WIDTH : usize = 25;
const IMAGE_HEIGHT : usize = 6;

fn parse_file(filename : impl AsRef<path::Path>) -> Image {
  let mut file = fs::File::open(filename).expect("File not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Could not read file");
  let content : Vec<u8> = contents.trim().chars()
    .map(|c| c.to_digit(3).expect("Could not parse number") as Pixel)
    .collect();
  let layer_count = content.len() / (IMAGE_WIDTH * IMAGE_HEIGHT);
  let mut cursor = 0;
  let mut image = Image::new();
  for _ in 0..layer_count {
    let mut layer = Layer::new();
    for _ in 0..IMAGE_HEIGHT {
      let mut line = Line::new();
      for _ in 0..IMAGE_WIDTH {
        line.push(content[cursor]);
        cursor += 1;
      }
      layer.push(line);
    }
    image.push(layer);
  }
  image
}

fn get_digit_count(layer : &Layer, digit : Pixel) -> usize {
  let mut total_digits = 0;
  layer.iter().for_each(|line| total_digits += line.iter().filter(|&d| *d == digit).count());
  total_digits
}

fn part1(image : &Image) {
  let before = std::time::Instant::now();
  let mut least_zeros = get_digit_count(&image[0], 0);
  let mut winning_layer_id = 0;
  for layer_id in 1..image.len() {
    let zeros = get_digit_count(&image[layer_id], 0);
    if zeros < least_zeros {
      least_zeros = zeros;
      winning_layer_id = layer_id;
    }
  }
  let one = get_digit_count(&image[winning_layer_id], 1);
  let two = get_digit_count(&image[winning_layer_id], 2);
  let result = one * two;

  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part1: result: {}", result);
}

fn get_pixel(image : &Image, w : usize, h : usize) -> Pixel {
  for layer in image {
    let layer_pixel = layer[h][w];
    if layer_pixel != 2 {
      return layer_pixel
    }
  }
  2
}

fn part2(image : &Image) {
  let before = std::time::Instant::now();
  let mut combined_layer = Vec::new();
  for h in 0..IMAGE_HEIGHT {
    let mut line : String = String::new();
    for w in 0..IMAGE_WIDTH {
      let pixel = get_pixel(image, w, h);
      let pixel_char = match pixel {
        0 => ' ',
        1 => 'X',
        2 => '?',
        _ => panic!("Impossible happened"),
      };
      line.push(pixel_char)
    }
    combined_layer.push(line);
  }
  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part2: result:\n");
  for line in combined_layer {
    println!("{}", line);    
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let image = parse_file(filename);
  part1(&image);
  part2(&image);  
}
