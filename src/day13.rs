use crate::intcode_computer::{parse_input, State, CPU, SIZE};
use pancurses::{endwin, initscr, Input};

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
    cpu.halt_on_output = true;
    cpu.memory[0] = 2;

    let window = initscr();

    let mut block_count = 0;

    let mut input = None;

    loop {
        window.refresh();
        match cpu.run(input) {
            State::Halt => break,
            State::Output(x) => {
                if let State::Output(y) = cpu.run(None) {
                    if let State::Output(tile_id) = cpu.run(None) {
                        if tile_id == 2 {
                            block_count += 1;
                        }
                        // points.push((x, y, tile_id));
                    }
                }
            }
            State::Input => {
                println!("block_count: {}", block_count);
                match window.getch() {
                    Some(Input::KeyDC) => break,
                    Some(Input::Character(c)) => {
                        window.addch(c);
                    }
                    Some(Input::KeyLeft) => {
                        println!("left");
                        input = Some(-1);
                        window.printw("left");
                    }
                    Some(Input::KeyRight) => {
                        println!("right");
                        input = Some(1);
                        window.printw("right");
                    }
                    _ => (),
                }
            }
            _ => continue,
        }
    }

    endwin();

    0
}
