fn parse_opcode(input: &[usize]) -> std::vec::Vec<usize> {
  let mut result = input.to_owned();
  for index in (0..input.len()).step_by(4) {
    match input[index] {
      1 => {
        let value1 = result[index + 1];
        let value2 = result[index + 2];
        let value3 = result[index + 3];
        result[value3] = result[value1] + result[value2];
      }
      2 => {
        let value1 = result[index + 1];
        let value2 = result[index + 2];
        let value3 = result[index + 3];
        result[value3] = result[value1] * result[value2];
      }
      99 => break, // terminate
      _ => continue,
    }
  }

  result
}

fn find_pair(input: &[usize], target_value: usize) -> (usize, usize) {
  let mut noun = 0;
  let mut verb = 0;

  loop {
    loop {
      let mut input_copy = input.to_owned();

      input_copy[1] = noun;
      input_copy[2] = verb;

      parse_opcode(&input_copy);

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

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<usize> {
  input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[usize]) -> usize {
  let mut result = Vec::from(input);

  result[1] = 12;
  result[2] = 2;

  let result = parse_opcode(&result);
  result[0]
}

#[aoc(day2, part2)]
pub fn part2(input: &[usize]) -> usize {
  let (noun, verb) = find_pair(&input, 19_690_720);

  (100 * noun + verb)
}

#[cfg(test)]
mod tests {
  use super::parse_opcode;

  #[test]
  fn test_part1() {
    let input = vec![1, 0, 0, 0, 99];
    let result = parse_opcode(&input);
    assert_eq!(result[0], 2);
  }
}
