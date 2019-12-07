use std::collections::HashMap;
use std::collections::HashSet;

type SpaceObjectId = usize;

struct SpaceObject {
  name : String,
  parent : Option<SpaceObjectId>,
  childs : Vec<SpaceObjectId>,
}

struct SpaceMap {
  data: Vec<SpaceObject>,
  map: HashMap<String, SpaceObjectId>,
}

fn add_orbit(dic : &mut HashMap<String, Vec<String>>, line: &str) {
  let parent : String = line.chars().take(3).collect();
  let child  : String = line.chars().skip(4).take(3).collect();
  let current_childs = dic.entry(parent.clone()).or_insert(Vec::new());
  current_childs.push(child);
}

fn dic_to_space_map(dic : &HashMap<String, Vec<String>>, parent : Option<SpaceObjectId>, space_map : &mut SpaceMap, key : String) {
  let space_objet_id = space_map.data.len();
  space_map.map.insert(key.clone(), space_objet_id);
  space_map.data.push(SpaceObject {
    name : key.clone(),
    parent: parent,
    childs: Vec::new(),
  });
  if dic.contains_key(&key) {
    dic[&key].iter().for_each(|child_name| dic_to_space_map(dic, Some(space_objet_id), space_map, child_name.clone()));
  }
  match parent {
    None => (),
    Some(id) => space_map.data[id].childs.push(space_objet_id),
  }
}

fn parse_file(filename : impl AsRef<std::path::Path>) -> SpaceMap {
  use std::io::{prelude::*};
  let mut dic = std::collections::HashMap::new();
  let file = std::fs::File::open(filename).expect("File not found");
  let buf = std::io::BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .for_each(|l| add_orbit(&mut dic, &l));
  let mut space_map = SpaceMap {
    data : Vec::new(),
    map : HashMap::new(),
  };
  dic_to_space_map(&dic, None, &mut space_map, String::from("COM"));
  space_map
}

fn get_total_orbits(space_map: &SpaceMap, planet_name: &String, distance : usize) -> usize {
  if !space_map.map.contains_key(planet_name) {
    distance
  } else {
    let mut child_orbits = 0;
    let object_id = &space_map.map[&String::from(planet_name)];
    let childs_id = &space_map.data[*object_id].childs;
    childs_id.iter().for_each(|child_id| child_orbits +=
      get_total_orbits(space_map, &space_map.data[*child_id].name, distance + 1));
    child_orbits + distance
  }
}

fn part1(space_map: &SpaceMap) {
  let before = std::time::Instant::now();
  let result1 = get_total_orbits(space_map, &String::from("COM"), 0);
  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part1: total number of direct and indirect orbits: {}", result1);
}

fn get_object_to_visite(space_object : &SpaceObject, visited_node : &HashSet<SpaceObjectId>) -> HashSet<SpaceObjectId> {
  let mut to_visit : HashSet<SpaceObjectId> = space_object.childs.iter()
    .filter(|id| !visited_node.contains(&id))
    .map(|id| *id)
    .collect();
    match space_object.parent {
      None => (),
      Some(parent_id) => { to_visit.insert(parent_id); () },
    }
  to_visit
}

fn get_orbits_to_santa(space_map: &SpaceMap) -> Result<usize, &'static str> {
  let you_id = space_map.map["YOU"];
  let san_id = space_map.map["SAN"];
  let mut path_len = 0;
  let mut visited_node = HashSet::new();
  let mut objects_id_to_visit = get_object_to_visite(&space_map.data[you_id], &visited_node);
  while objects_id_to_visit.len() > 0 {
    let mut next_objects_id_to_visit : HashSet<SpaceObjectId> = HashSet::new();
    for object_id in objects_id_to_visit {
      if object_id == san_id {
        return Ok(path_len - 1);
      }
      visited_node.insert(object_id);
      next_objects_id_to_visit.extend(get_object_to_visite(&space_map.data[object_id], &visited_node));
    }
    objects_id_to_visit = next_objects_id_to_visit;
    path_len += 1;
  }
  Err("Path not found")
}

fn part2(space_map: &SpaceMap) {
  let before = std::time::Instant::now();
  let result = get_orbits_to_santa(space_map).unwrap();
  println!("Elapsed time: {:.2?}", before.elapsed());
  println!("Part2: orbits to reach Santa: {}", result);
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let filename = &args[1];
  println!("Loading file {}", filename);
  let content = parse_file(filename);
  part1(&content);
  part2(&content);
}
