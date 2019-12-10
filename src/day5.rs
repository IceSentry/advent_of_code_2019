use crate::intcode_computer::{parse_input, CPU, SIZE};

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<SIZE> {
  parse_input(input)
}

#[aoc(day5, part1)]
fn part1(input: &[SIZE]) -> SIZE {
  let mut cpu = CPU::new(input.to_owned(), Some(1));

  cpu.run();

  *cpu.output.last().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[SIZE]) -> SIZE {
  let mut cpu = CPU::new(input.to_owned(), Some(5));

  cpu.run();

  *cpu.output.last().unwrap()
}
