use crate::intcode_computer::{parse_input, State, CPU, SIZE};
use pancurses::{endwin, initscr, Input};
use std::cmp::Ordering;
use std::{thread, time};

#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day13, part1)]
fn part1(input: &[SIZE]) -> i32 {
    let mut cpu = CPU::new(input.to_owned());

    cpu.halt_on_output = true;

    let mut points: Vec<(SIZE, SIZE, SIZE)> = Vec::new();
    let mut block_count = 0;

    loop {
        match cpu.run(None) {
            State::Halt => break,
            State::Output(x) => {
                if let State::Output(y) = cpu.run(None) {
                    if let State::Output(tile_id) = cpu.run(None) {
                        if tile_id == 2 {
                            block_count += 1;
                        }
                        points.push((x, y, tile_id));
                    }
                }
            }
            _ => continue,
        }
    }

    block_count as i32
}

#[aoc(day13, part2)]
fn part2(input: &[SIZE]) -> i32 {
    let mut cpu = CPU::new(input.to_owned());
    cpu.memory[0] = 2;

    let window = initscr();
    window.keypad(true);

    let use_human_player = false;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    loop {
        window.refresh();
        match cpu.step() {
            State::Halt => {
                println!("game over!");
                break;
            }
            State::Input => {
                if use_human_player {
                    match window.getch() {
                        Some(Input::KeyLeft) => {
                            println!("left");
                            cpu.input.push_front(-1);
                        }
                        Some(Input::KeyRight) => {
                            println!("right");
                            cpu.input.push_front(1);
                        }
                        _ => cpu.input.push_front(0),
                    }
                } else {
                    match paddle_x.cmp(&ball_x) {
                        Ordering::Less => cpu.input.push_front(1),
                        Ordering::Greater => cpu.input.push_front(-1),
                        Ordering::Equal => cpu.input.push_front(0),
                    }
                }
            }
            State::Running => {
                if cpu.output.len() == 3 {
                    let x = cpu.output[0];
                    let y = cpu.output[1];
                    let tile_id = cpu.output[2];
                    cpu.output.clear();

                    if x == -1 && y == 0 {
                        score = tile_id;
                        println!("score: {}", score);
                    } else {
                        match tile_id {
                            3 => paddle_x = x,
                            4 => ball_x = x,
                            _ => (),
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    endwin();

    score as i32
}
