use crate::intcode_computer::{parse_input, CPU, SIZE};

#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day11, part1)]
fn part1(input: &[SIZE]) -> SIZE {
    let mut cpu = CPU::new(input.to_owned());

    cpu.run(Some(1));

    *cpu.output.last().unwrap()
}
