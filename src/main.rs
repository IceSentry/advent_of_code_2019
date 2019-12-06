#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    day_3();
}

fn day_1() {
    let input = File::open("src/1/input.txt").expect("Unable to open file");
    let reader = BufReader::new(input);
    let mut output = File::create("src/1/output.txt").expect("Unable to create file");

    let mut total = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let mut fuel = (mass / 3) - 2;

        let mut result = (fuel / 3) - 2;
        while result > 0 {
            fuel += result;
            result = (result / 3) - 2;
        }

        total += fuel;
    }

    write!(output, "{}", total).expect("Unable to write result");
}

fn parse_opcode(input: &mut Vec<usize>) {
    for index in (0..input.len()).step_by(4) {
        match input[index] {
            1 => {
                let value1 = input[index + 1];
                let value2 = input[index + 2];
                let value3 = input[index + 3];
                input[value3] = input[value1] + input[value2];
            }
            2 => {
                let value1 = input[index + 1];
                let value2 = input[index + 2];
                let value3 = input[index + 3];
                input[value3] = input[value1] * input[value2];
            }
            99 => break, // terminate
            _ => continue,
        }
    }
}

fn day_2() {
    let input = fs::read_to_string("src/2/input.txt").expect("Unable to open input.txt");
    let mut output = File::create("src/2/output.txt").expect("Unable to create output.txt");

    let mut input: Vec<usize> = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    input[1] = 12;
    input[2] = 2;

    parse_opcode(&mut input);

    // println!(
    //     "{}",
    //     input
    //         .iter()
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>()
    //         .join(",")
    // );

    write!(output, "{}", input[0]).expect("Unable to write result");
}

fn find_pair(input: &[usize], target_value: usize) -> (usize, usize) {
    let mut noun = 0;
    let mut verb = 0;

    loop {
        loop {
            let mut input_copy = input.to_owned();

            input_copy[1] = noun;
            input_copy[2] = verb;

            parse_opcode(&mut input_copy);

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

fn day_2_part_2() {
    let input = fs::read_to_string("src/2/input.txt").expect("Unable to open input.txt");
    let mut output = File::create("src/2/output.txt").expect("Unable to create output.txt");

    let input: Vec<usize> = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let (noun, verb) = find_pair(&input, 19_690_720);

    let result = 100 * noun + verb;

    write!(output, "{}", result).expect("Unable to write result");
}

struct Point(i32, i32);

fn manhattan_dist(p: Point, q: Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn day_3() {
    let input = fs::read_to_string("src/2/input.txt").expect("Unable to open input.txt");
    let input = input.split('\n');

    println!("{}", manhattan_dist(Point(1, 1), Point(3, 3)))
}
