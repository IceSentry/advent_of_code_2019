use std::collections::{HashMap, VecDeque};

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
        center: orbits[0].to_owned(),
        satelite: orbits[1].to_owned(),
      }
    })
    .collect()
}

struct OrbitMap {
  map: HashMap<String, Vec<String>>,
}

impl OrbitMap {
  fn build_map(orbits: &[Orbit]) -> Self {
    let mut map = HashMap::new();
    for orbit in orbits.iter() {
      match map.get_mut(&orbit.center.clone()) {
        None => {
          map.insert(orbit.center.clone(), vec![orbit.satelite.clone()]);
        }
        Some(orbits) => {
          orbits.push(orbit.satelite.clone());
        }
      };
    }
    OrbitMap { map }
  }

  fn build_complete_map(orbits: &[Orbit]) -> Self {
    let mut map = OrbitMap::build_map(orbits).map;
    for orbit in orbits.iter() {
      match map.get_mut(&orbit.satelite.clone()) {
        None => {
          map.insert(orbit.satelite.clone(), vec![orbit.center.clone()]);
        }
        Some(orbits) => {
          orbits.push(orbit.center.clone());
        }
      };
    }
    OrbitMap { map }
  }

  fn count_orbits(&self, planet: &str) -> i32 {
    match self.map.get(&planet.to_owned()) {
      None => 0,
      Some(orbits) => orbits
        .iter()
        .map(|orbit| self.count_orbits(&orbit.clone()) + 1)
        .sum(),
    }
  }

  /// Uses Breadth First Search
  fn find_path(&self, start: &str, target: &str) -> Vec<String> {
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut discovered: HashMap<String, &str> = HashMap::new();
    let mut path: Vec<String> = vec![target.to_owned()];

    discovered.insert(start.to_owned(), "");
    queue.push_front(start);

    while !queue.is_empty() {
      let planet = queue.pop_front().unwrap();
      if planet == target {
        break;
      }
      for edge in self.map.get(planet).unwrap() {
        match discovered.get(&edge.to_owned()) {
          Some(_) => continue,
          None => {
            discovered.insert(edge.to_owned(), planet);
            queue.push_front(edge);
          }
        }
      }
    }

    let mut current = target;
    while current != start {
      current = discovered.get(&current.to_owned()).unwrap();
      path.push(current.to_owned());
    }

    path.reverse(); // Not necessary for the puzzle
    path
  }
}

#[aoc(day6, part1)]
fn part1(input: &[Orbit]) -> i32 {
  let orbit_map = OrbitMap::build_map(input);

  orbit_map
    .map
    .keys()
    .map(|planet| orbit_map.count_orbits(&planet.clone()))
    .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Orbit]) -> usize {
  let orbit_map = OrbitMap::build_complete_map(input);
  let path = orbit_map.find_path("YOU", "SAN");
  path.len() - 3 // remove YOU and SAN from count and
                 // -1 because we count the jumps not the planets
}

#[cfg(test)]
mod tests {
  use super::{generator_input, part1, part2, OrbitMap};

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

    let map = OrbitMap::build_map(&input);

    assert_eq!(map.count_orbits("COM"), 4);
    assert_eq!(map.count_orbits("B"), 3);
    assert_eq!(map.count_orbits("C"), 2);
    assert_eq!(map.count_orbits("D"), 0);
    assert_eq!(map.count_orbits("E"), 0);

    assert_eq!(part1(&input), 9);
  }
}
