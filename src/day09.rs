use crate::intcode_computer::{parse_input, CPU, SIZE};

#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day9, part1)]
fn part1(input: &[SIZE]) -> SIZE {
    let mut cpu = CPU::new(input.to_owned());

    cpu.run_with_input(Some(1));

    *cpu.output.last().unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[SIZE]) -> SIZE {
    let mut cpu = CPU::new(input.to_owned());

    cpu.run_with_input(Some(2));

    *cpu.output.last().unwrap()
}
