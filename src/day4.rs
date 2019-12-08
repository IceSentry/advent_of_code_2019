// - It is a six-digit number.
// - The value is within the range given in your puzzle input.
// - Two adjacent digits are the same (like 22 in 122345).
// - Going from left to right, the digits never decrease;
//   they only ever increase or stay the same (like 111123 or 135679).

fn validate_password_part1(pw: &str) -> bool {
  if pw.len() > 6 || pw.len() < 6 {
    return false;
  }
  let chars = pw.chars();

  let mut last_c = 0;
  let mut has_adj = false;
  for c in chars {
    let c: i32 = c.to_string().parse().unwrap();

    if c < last_c {
      return false;
    }

    if c == last_c {
      has_adj = true
    }

    last_c = c;
  }

  has_adj
}

fn validate_password_part2(pw: &str) -> bool {
  if pw.len() > 6 || pw.len() < 6 {
    return false;
  }
  let chars = pw.chars();

  let mut last_c = 0;
  let mut has_adj = false;
  for c in chars {
    let c: i32 = c.to_string().parse().unwrap();

    if c < last_c {
      return false;
    }

    if c == last_c {
      has_adj = true
    }

    last_c = c;
  }

  has_adj
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
  let input: Vec<i32> = input
    .split('-')
    .map(|i| i.parse::<i32>())
    .filter_map(Result::ok)
    .collect();

  let mut count = 0;
  for i in input[0]..input[1] {
    if validate_password_part1(&i.to_string()) {
      count += 1;
    }
  }
  count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
  let input: Vec<i32> = input
    .split('-')
    .map(|i| i.parse::<i32>())
    .filter_map(Result::ok)
    .collect();

  let mut count = 0;
  for i in input[0]..input[1] {
    if validate_password_part2(&i.to_string()) {
      count += 1;
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use super::{validate_password_part1, validate_password_part2};

  #[test]
  fn test_day4_part1() {
    assert!(validate_password_part1(&String::from("111111")));
    assert!(validate_password_part1(&String::from("223456")));
    assert!(!validate_password_part1(&String::from("223450")));
    assert!(!validate_password_part1(&String::from("123789")));
  }

  #[test]
  fn test_day4_part2() {
    assert!(validate_password_part2(&String::from("111111")));
    assert!(validate_password_part2(&String::from("223456")));
    assert!(!validate_password_part2(&String::from("223450")));
    assert!(!validate_password_part2(&String::from("123789")));
  }
}
