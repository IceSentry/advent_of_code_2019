use crate::intcode_computer::{parse_input, CPU, SIZE};

use permute;

#[aoc_generator(day7)]
fn generator_input(input: &str) -> Vec<SIZE> {
  parse_input(input)
}

fn test_phase_setting(code: &[SIZE], phase_setting: Vec<SIZE>) -> SIZE {
  let amplifiers = vec!["A", "B", "C", "D", "E"];
  let mut cpus: Vec<CPU> = amplifiers
    .iter()
    .map(|_| CPU::new(code.to_owned()))
    .collect();

  let mut phase_signal = 0;
  for (phase_index, cpu) in cpus.iter_mut().enumerate() {
    let output = cpu.run(Some(&[phase_setting[phase_index], phase_signal]));
    phase_signal = output[0];
  }

  phase_signal
}

#[aoc(day7, part1)]
fn part1(input: &[SIZE]) -> SIZE {
  let settings = vec![0, 1, 2, 3, 4];

  permute::permute(settings)
    .iter()
    .map(|setting| test_phase_setting(input, setting.to_owned()))
    .max()
    .unwrap()
}

#[cfg(test)]
mod test {
  use super::test_phase_setting;

  #[test]
  fn test_day7_phase_setting() {
    let input = vec![
      3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    let phase_setting = vec![4, 3, 2, 1, 0];

    assert_eq!(test_phase_setting(&input, phase_setting), 43210);

    let input = vec![
      3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
      0, 0,
    ];

    let phase_setting = vec![0, 1, 2, 3, 4];

    assert_eq!(test_phase_setting(&input, phase_setting), 54321);

    let input = vec![
      3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
      31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];

    let phase_setting = vec![1, 0, 4, 3, 2];

    assert_eq!(test_phase_setting(&input, phase_setting), 65210);
  }
}
