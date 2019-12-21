use crate::intcode_computer::{parse_input, State, CPU, SIZE};
use ansi_term::Colour::{Black, Red, White};
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Point(i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn run_paint_robot(
    cpu: &mut CPU,
    default_color: SIZE,
) -> (HashMap<Point, SIZE>, i32, i32, i32, i32) {
    let mut panels = HashMap::new();

    let mut x = 0;
    let mut max_x = x;
    let mut min_x = x;
    let mut y = 0;
    let mut max_y = y;
    let mut min_y = y;
    let mut input = Point(x, y);
    let mut current_direction = Direction::Up;
    loop {
        match cpu.run_with_input(Some(*panels.get(&input).unwrap_or(&default_color))) {
            State::Halt => break,
            State::Output(color_to_paint) => {
                panels.insert(Point(input.0, input.1), color_to_paint);
                if let State::Output(direction) = cpu.run() {
                    match direction {
                        0 => match current_direction {
                            Direction::Up => current_direction = Direction::Left,
                            Direction::Down => current_direction = Direction::Right,
                            Direction::Left => current_direction = Direction::Down,
                            Direction::Right => current_direction = Direction::Up,
                        }, // turn left
                        1 => match current_direction {
                            Direction::Up => current_direction = Direction::Right,
                            Direction::Down => current_direction = Direction::Left,
                            Direction::Left => current_direction = Direction::Up,
                            Direction::Right => current_direction = Direction::Down,
                        }, // turn right
                        _ => unreachable!(),
                    }
                };

                match current_direction {
                    Direction::Up => y += 1,
                    Direction::Down => y -= 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                };

                input = Point(x, y);

                if x > max_x {
                    max_x = x;
                } else if x < min_x {
                    min_x = x;
                }

                if y > max_y {
                    max_y = y;
                } else if y < min_y {
                    min_y = y;
                }
            }
            _ => continue,
        }
    }

    (panels, max_x, min_x, max_y, min_y)
}

#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day11, part1)]
fn part1(input: &[SIZE]) -> usize {
    let mut cpu = CPU::new(input.to_owned());

    cpu.halt_on_output = true;

    let result = run_paint_robot(&mut cpu, 0);

    result.0.len()
}

#[aoc(day11, part2)]
fn part2(input: &[SIZE]) -> String {
    let mut cpu = CPU::new(input.to_owned());

    cpu.halt_on_output = true;

    let (result, max_x, min_x, max_y, min_y) = run_paint_robot(&mut cpu, 1);

    let height = max_y - min_y + 1;
    let width = max_x - min_x;

    let mut output = String::from("\n");

    for y in 0..height {
        for x in 0..width {
            let color: String = match result.get(&Point(x, -y)).unwrap_or(&2) {
                0 => Black.on(Black).paint("#").to_string(),
                1 => White.on(Black).paint("#").to_string(),
                _ => Red.on(Black).paint("#").to_string(),
            };
            output.push_str(&color);
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_day11_part1() -> std::io::Result<()> {
        let mut file = File::open("input/2019/day11.txt")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let result = part1(&generator_input(&input.trim()));
        assert_eq!(result, 2539);

        Ok(())
    }
}
