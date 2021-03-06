use crate::intcode_computer::{parse_code, parse_input, SIZE};

fn find_pair(input: &[SIZE], target_value: SIZE) -> (SIZE, SIZE) {
    let mut noun = 0;
    let mut verb = 0;

    loop {
        if noun == 99 {
            return (0, 0);
        }
        loop {
            let mut input_copy = input.to_owned();

            input_copy[1] = noun;
            input_copy[2] = verb;

            let result = parse_code(&input_copy);

            if result[0] == target_value {
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

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

#[aoc(day2, part1)]
pub fn part1(input: &[SIZE]) -> SIZE {
    let mut result = Vec::from(input);

    result[1] = 12;
    result[2] = 2;

    let result = parse_code(&result);
    result[0]
}

#[aoc(day2, part2)]
pub fn part2(input: &[SIZE]) -> SIZE {
    let (noun, verb) = find_pair(&input, 19_690_720);

    (100 * noun + verb)
}

#[cfg(test)]
mod tests {
    use crate::intcode_computer::parse_code;
    #[test]
    fn test_day02_part1() {
        let input = vec![1, 0, 0, 0, 99];
        let result = parse_code(&input);
        assert_eq!(result[0], 2);
    }
}
