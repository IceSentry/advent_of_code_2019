use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);
fn parse_wire(wire_string: &str) -> Vec<Point> {
    let mut wire = vec![];
    let mut x = 0;
    let mut y = 0;

    for step in wire_string.split(',') {
        let mut chars = step.chars();
        let direction = chars.next();
        let len: i32 = chars.as_str().parse().unwrap();

        for _ in 0..len {
            match direction.unwrap() {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!("Unknown direction"),
            };

            wire.push(Point(x, y));
        }
    }

    wire
}

fn parse_wires(input: Vec<&str>) -> (Vec<Point>, Vec<Point>, HashSet<Point>, HashSet<Point>) {
    let wire_1 = parse_wire(input[0]);
    let wire_2 = parse_wire(input[1]);

    let wire_1_h: HashSet<Point> = HashSet::from_iter(wire_1.iter().cloned());
    let wire_2_h: HashSet<Point> = HashSet::from_iter(wire_2.iter().cloned());

    (wire_1, wire_2, wire_1_h, wire_2_h)
}

fn manhattan_dist(p: Point, q: &Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn calculate_steps(wire: &[Point], intersect: &Point) -> i32 {
    let mut steps = 0;

    for p in wire.iter() {
        steps += 1;
        if p == intersect {
            break;
        }
    }

    steps
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let input: Vec<&str> = input.split('\n').collect();

    let (_, _, wire_1_h, wire_2_h) = parse_wires(input);

    let intersect: HashSet<&Point> = wire_1_h.intersection(&wire_2_h).collect();
    intersect
        .iter()
        .map(|int| manhattan_dist(Point(0, 0), &int))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let input: Vec<&str> = input.split('\n').collect();

    let (wire_1, wire_2, wire_1_h, wire_2_h) = parse_wires(input);

    let intersect: HashSet<&Point> = wire_1_h.intersection(&wire_2_h).collect();

    intersect
        .iter()
        .map(|int| {
            let wire_1_steps = calculate_steps(&wire_1, &int);
            let wire_2_steps = calculate_steps(&wire_2, &int);
            wire_1_steps + wire_2_steps
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_day03_part1() {
        assert_eq!(
            part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
        assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    }

    #[test]
    fn test_day03_part2() {
        assert_eq!(
            part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
        assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    }
}
