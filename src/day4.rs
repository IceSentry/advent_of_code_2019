use std::collections::HashMap;

fn validate_password_part1(pw: &str) -> bool {
    if pw.len() > 6 || pw.len() < 6 {
        return false;
    }
    let chars = pw.chars();

    let mut last_c = 0;
    let mut has_adj = false;
    let chars = chars.map(|c| c.to_string().parse().unwrap());

    for c in chars {
        if c < last_c {
            return false;
        }

        if c == last_c {
            has_adj = true
        }

        last_c = c;
    }

    has_adj
}

fn validate_password_part2(pw: &str) -> bool {
    let chars = pw.chars();

    let mut last_c = 0;
    let mut adj: HashMap<i32, i32> = HashMap::new();

    for c in chars {
        let c: i32 = c.to_string().parse().unwrap();

        if c == last_c {
            adj.insert(
                c,
                match adj.get(&c) {
                    Some(val) => val + 1,
                    None => 1,
                },
            );
        }

        last_c = c;
    }

    adj.values().any(|val| *val == 1)
}

#[aoc_generator(day4)]
fn generator_input(input: &str) -> (i32, i32) {
    let input: Vec<i32> = input
        .split('-')
        .map(|i| i.parse())
        .filter_map(Result::ok)
        .collect();

    (input[0], input[1])
}

#[aoc(day4, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
fn part1((min, max): &(i32, i32)) -> usize {
    (*min..*max)
        .map(|x| format!("{}", x))
        .filter(|x| validate_password_part1(x))
        .count()
}

#[aoc(day4, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
fn part2((min, max): &(i32, i32)) -> usize {
    (*min..*max)
        .map(|x| format!("{}", x))
        .filter(|x| validate_password_part1(x))
        .filter(|x| validate_password_part2(x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_part1() {
        assert!(validate_password_part1(&String::from("111111")));
        assert!(validate_password_part1(&String::from("223456")));
        assert!(!validate_password_part1(&String::from("223450")));
        assert!(!validate_password_part1(&String::from("123789")));
    }

    #[test]
    fn test_day04_part2() {
        assert!(
            validate_password_part2(&String::from("112233")),
            "digits never decrease and are repeated at 2 digits"
        );
        assert!(
            !validate_password_part2(&String::from("123444")),
            "no repeated digits"
        );
        assert!(
            validate_password_part2(&String::from("111122")),
            "at least one pair of digits"
        );
    }
}
