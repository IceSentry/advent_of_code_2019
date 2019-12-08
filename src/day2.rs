fn parse_opcode(input: &mut Vec<usize>) {
  for index in (0..input.len()).step_by(4) {
    match input[index] {
      1 => {
        let value1 = input[index + 1];
        let value2 = input[index + 2];
        let value3 = input[index + 3];
        input[value3] = input[value1] + input[value2];
      }
      2 => {
        let value1 = input[index + 1];
        let value2 = input[index + 2];
        let value3 = input[index + 3];
        input[value3] = input[value1] * input[value2];
      }
      99 => break, // terminate
      _ => continue,
    }
  }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
  let mut input: Vec<usize> = input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  input[1] = 12;
  input[2] = 2;

  parse_opcode(&mut input);
  input[0] as i32
}

fn find_pair(input: &[usize], target_value: usize) -> (usize, usize) {
  let mut noun = 0;
  let mut verb = 0;

  loop {
    loop {
      let mut input_copy = input.to_owned();

      input_copy[1] = noun;
      input_copy[2] = verb;

      parse_opcode(&mut input_copy);

      if input_copy[0] == target_value {
        return (noun, verb);
      } else if verb >= 99 {
        break;
      }

      verb += 1;
    }
    verb = 0;
    noun += 1;
  }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
  let input: Vec<usize> = input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  let (noun, verb) = find_pair(&input, 19_690_720);

  (100 * noun + verb) as i32
}

#[cfg(test)]
mod tests {
  use super::parse_opcode;

  #[test]
  fn test_part1() {
    let mut input = vec![1, 0, 0, 0, 99];
    parse_opcode(&mut input);
    assert_eq!(input[0], 2);
  }
}
