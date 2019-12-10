fn fuel_needed(w: i32) -> i32 {
  (w / 3) - 2
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
  let mut total = 0;

  for line in input.lines() {
    let mass: i32 = line.parse().unwrap();
    total += fuel_needed(mass);
  }

  total
}

#[aoc(day1, part1, map)]
pub fn part1_map(input: &str) -> i32 {
  input
    .lines()
    .map(|mass| mass.parse().unwrap())
    .fold(0, |acc, mass| acc + fuel_needed(mass))
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
  let mut total = 0;

  for line in input.lines() {
    let mass = line.parse::<i32>().unwrap();
    let mut fuel = fuel_needed(mass);

    let mut result = fuel_needed(fuel);
    while result > 0 {
      fuel += result;
      result = fuel_needed(result);
    }

    total += fuel;
  }

  total
}

#[aoc(day1, part2, map)]
pub fn part2_map(input: &str) -> i32 {
  input
    .lines()
    .map(|mass| mass.parse().unwrap())
    .fold(0, |acc, mass| {
      let mut fuel = fuel_needed(mass);

      let mut result = fuel_needed(fuel);
      while result > 0 {
        fuel += result;
        result = fuel_needed(result);
      }
      acc + fuel
    })
}

#[cfg(test)]
mod tests {
  use super::{part1, part1_map};

  #[test]
  fn test_part1() {
    assert_eq!(part1("12"), 2);
    assert_eq!(part1_map("12"), part1("12"));
    assert_eq!(part1("14"), 2);
  }
}
