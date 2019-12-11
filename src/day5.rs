use crate::intcode_computer::{parse_input, CPU, SIZE};

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<SIZE> {
  parse_input(input)
}

#[aoc(day5, part1)]
fn part1(input: &[SIZE]) -> SIZE {
  let mut cpu = CPU::new(input.to_owned());

  let output = cpu.run(Some(&[1]));

  *output.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[SIZE]) -> SIZE {
  let mut cpu = CPU::new(input.to_owned());

  let output = cpu.run(Some(&[5]));

  *output.last().unwrap()
}
