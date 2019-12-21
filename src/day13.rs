use crate::intcode_computer::{parse_input, State, CPU, SIZE};
use std::cmp::Ordering;

#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day13, part1)]
fn part1(input: &[SIZE]) -> i32 {
    let mut cpu = CPU::new(input.to_owned());

    let mut block_count = 0;

    loop {
        match cpu.step() {
            State::Halt => break,
            State::Running => {
                if cpu.output.len() == 3 {
                    let tile_id = cpu.output[2];
                    cpu.output.clear();

                    if tile_id == 2 {
                        block_count += 1;
                    }
                }
            }
            _ => (),
        }
    }

    block_count as i32
}

#[aoc(day13, part2)]
fn part2(input: &[SIZE]) -> SIZE {
    let mut cpu = CPU::new(input.to_owned());
    cpu.memory[0] = 2;

    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    loop {
        match cpu.step() {
            State::Halt => break,
            State::Input => match paddle_x.cmp(&ball_x) {
                Ordering::Less => cpu.input.push_front(1),
                Ordering::Greater => cpu.input.push_front(-1),
                Ordering::Equal => cpu.input.push_front(0),
            },
            State::Running => {
                if cpu.output.len() == 3 {
                    let x = cpu.output[0];
                    let y = cpu.output[1];
                    let tile_id = cpu.output[2];
                    cpu.output.clear();

                    match (x, y, tile_id) {
                        (-1, 0, _) => score = tile_id,
                        (_, _, 3) => paddle_x = x,
                        (_, _, 4) => ball_x = x,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    score
}
