use std::collections::HashMap;

struct Orbit {
  center: String,
  satelite: String,
}

#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<Orbit> {
  input
    .split('\n')
    .map(|orbit| {
      let orbits: Vec<&str> = orbit.split(')').collect();
      Orbit {
        center: orbits[0].to_string(),
        satelite: orbits[1].to_string(),
      }
    })
    .collect()
}

fn build_map(orbits: &[Orbit]) -> HashMap<String, Vec<String>> {
  let mut map: HashMap<String, Vec<String>> = HashMap::new();
  let mut map_reverse: HashMap<String, Vec<String>> = HashMap::new();

  for orbit in orbits.iter() {
    match map.get_mut(&orbit.center.clone()) {
      None => {
        map.insert(orbit.center.clone(), vec![orbit.satelite.clone()]);
      }
      Some(orbits) => {
        orbits.push(orbit.satelite.clone());
      }
    };

    match map_reverse.get_mut(&orbit.satelite.clone()) {
      None => {
        map_reverse.insert(orbit.satelite.clone(), vec![orbit.center.clone()]);
      }
      Some(orbits) => {
        orbits.push(orbit.center.clone());
      }
    };
  }

  map
}

fn count_orbits(map: &HashMap<String, Vec<String>>, planet: String) -> i32 {
  match map.get(&planet) {
    None => 0,
    Some(orbits) => {
      let mut count = 0;
      for orbit in orbits {
        count += count_orbits(map, orbit.clone()) + 1;
      }
      count
    }
  }
}

#[aoc(day6, part1)]
fn part1(input: &[Orbit]) -> i32 {
  let map = build_map(input);

  map
    .keys()
    .fold(0, |acc, planet| acc + count_orbits(&map, planet.clone()))
}

fn find_parent(map: &HashMap<String, Vec<String>>, planet: String) -> String {
  map
    .iter()
    .filter(|(_, orbits)| orbits.contains(&planet))
    .map(|(planet, _)| planet.clone())
    .collect::<Vec<String>>()
    .first()
    .unwrap()
    .clone()
}

#[aoc(day6, part2)]
fn part2(input: &[Orbit]) -> i32 {
  let map = build_map(input);

  let start_planet = find_parent(&map, "YOU".to_string());
  let end_planet = find_parent(&map, "SAN".to_string());

  println!("start: {} end: {}", start_planet, end_planet);

  // let start_planet = for (center, orbits) in map.iter() {
  //   if orbits.contains(&"YOU".to_string()) {
  //     return center;
  //   }
  // };

  // println!("{}", start_planet);

  0
}

#[cfg(test)]
mod tests {
  use super::{build_map, count_orbits, generator_input, part1, part2};

  #[test]
  fn test_day6_part1() {
    let input = generator_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");

    assert_eq!(part1(&input), 42);
  }

  #[test]
  fn test_day6_part2() {
    let input =
      generator_input("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");

    assert_eq!(part2(&input), 4);
  }

  #[test]
  fn test_custom() {
    let input = generator_input("COM)B\nB)C\nC)D\nC)E");

    let map = build_map(&input);

    assert_eq!(count_orbits(&map, "COM".to_string()), 4);
    assert_eq!(count_orbits(&map, "B".to_string()), 3);
    assert_eq!(count_orbits(&map, "C".to_string()), 2);
    assert_eq!(count_orbits(&map, "D".to_string()), 0);
    assert_eq!(count_orbits(&map, "E".to_string()), 0);

    assert_eq!(part1(&input), 9);
  }
}
