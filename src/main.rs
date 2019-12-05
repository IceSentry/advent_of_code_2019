use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    day_2();
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

    output
        .write_all(total.to_string().as_bytes())
        .expect("Unable to write result");
}

fn day_2() {
    let input = fs::read_to_string("src/2/input.txt").expect("Unable to open input.txt");
    let mut output = File::create("src/2/output.txt").expect("Unable to create output.txt");

    output
        .write_all(&input.as_bytes())
        .expect("Unable to write result");
}
