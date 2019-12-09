#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
  let input: Vec<&str> = input.split('\n').collect();
  0
}

struct Planet {
  label: String,
  child_orbit: Vec<Planet>,
}
