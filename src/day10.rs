use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct AsteroidInfo {
    position: Vector2,
    distance: f32,
    slope: f32,
}

impl Vector2 {
    fn distance_to(self, target: Vector2) -> f32 {
        (((self.x - target.x).pow(2) + (self.y - target.y).pow(2)) as f32).sqrt()
    }

    fn slope_to(self, target: Vector2) -> f32 {
        let deltax = target.x - self.x;
        let deltay = target.y - self.y;
        let slope = (deltay as f32).atan2(deltax as f32);

        let mut slope = slope * -1.0;
        if slope <= std::f32::consts::FRAC_PI_2 {
            slope += 2.0 * std::f32::consts::PI;
        }
        slope
    }
}

struct AsteroidMap {
    asteroids: HashSet<Vector2>,
}

impl AsteroidMap {
    fn from(input: &str) -> Self {
        let width = input.find('\n').unwrap() as i32;
        let mut asteroids = HashSet::new();
        for (i, point) in input.replace('\n', "").chars().enumerate() {
            if point == '#' {
                asteroids.insert(Vector2 {
                    x: i as i32 % width,
                    y: i as i32 / width,
                });
            }
        }

        AsteroidMap { asteroids }
    }

    fn eval_position(&self, position: Vector2) -> Vec<AsteroidInfo> {
        let mut asteroids_info: Vec<AsteroidInfo> = Vec::new();
        for asteroid in self.asteroids.iter() {
            if *asteroid == position {
                continue;
            }
            asteroids_info.push(AsteroidInfo {
                slope: position.slope_to(*asteroid),
                position: *asteroid,
                distance: position.distance_to(*asteroid),
            });
        }
        asteroids_info
    }

    fn find_best_position(&self) -> (Vector2, i32) {
        self.asteroids
            .iter()
            .map(|ast| {
                let slopes: HashSet<String> = self
                    .eval_position(*ast)
                    .iter()
                    .map(|info| info.slope.to_string())
                    .collect();
                (*ast, slopes.len() as i32)
            })
            .max_by_key(|(_, count)| *count)
            .unwrap()
    }

    fn destroy_asteroids(&self, position: Vector2, target_amout: i32) -> AsteroidInfo {
        let mut asteroids: Vec<AsteroidInfo> = self.eval_position(position);

        asteroids.sort_by(|info_a, info_b| info_a.slope.partial_cmp(&info_b.slope).unwrap());
        asteroids.reverse();

        let grouped_by_slopes = asteroids.iter().group_by(|info| info.slope);

        for (i, (_, group)) in grouped_by_slopes.into_iter().enumerate() {
            if i as i32 == target_amout - 1 {
                let ast = group.min_by_key(|x| OrderedFloat(x.distance)).unwrap();
                return *ast;
            }
        }

        panic!("Failed to destory required amount")
    }
}

#[aoc_generator(day10)]
fn generator_input(input: &str) -> AsteroidMap {
    AsteroidMap::from(input)
}

#[aoc(day10, part1)]
fn part1(map: &AsteroidMap) -> String {
    let (_, count) = map.find_best_position();
    format!("{}", count)
}

/// Solution for 1 is more than 200 so we don't need to care about more than 1 rotation
#[aoc(day10, part2)]
fn part2(map: &AsteroidMap) -> i32 {
    let part1_result = Vector2 { x: 17, y: 22 };
    let ast = map.destroy_asteroids(part1_result, 200);
    ast.position.x * 100 + ast.position.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_example() {
        let map = AsteroidMap::from(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );

        let result = map.find_best_position();
        assert_eq!(result, (Vector2 { x: 3, y: 4 }, 8));
    }

    #[test]
    fn test_day10_part1() {
        let map = AsteroidMap::from(
            "......#.#.\n\
             #..#.#....\n\
             ..#######.\n\
             .#.#.###..\n\
             .#..#.....\n\
             ..#....#.#\n\
             #..#....#.\n\
             .##.#..###\n\
             ##...#..#.\n\
             .#....####",
        );

        let target = Vector2 { x: 5, y: 8 };

        let result = map.find_best_position();
        assert_eq!(result, (target, 33));

        let map = AsteroidMap::from(
            "#.#...#.#.\n\
             .###....#.\n\
             .#....#...\n\
             ##.#.#.#.#\n\
             ....#.#.#.\n\
             .##..###.#\n\
             ..#...##..\n\
             ..##....##\n\
             ......#...\n\
             .####.###.",
        );

        let result = map.find_best_position();
        assert_eq!(result, (Vector2 { x: 1, y: 2 }, 35));

        let map = AsteroidMap::from(
            ".#..#..###\n\
             ####.###.#\n\
             ....###.#.\n\
             ..###.##.#\n\
             ##.##.#.#.\n\
             ....###..#\n\
             ..#.#..#.#\n\
             #..#.#.###\n\
             .##...##.#\n\
             .....#.#..",
        );

        let result = map.find_best_position();
        assert_eq!(result, (Vector2 { x: 6, y: 3 }, 41));

        let map = AsteroidMap::from(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        );

        let result = map.find_best_position();
        assert_eq!(result, (Vector2 { x: 11, y: 13 }, 210));
    }

    #[test]
    #[should_panic]
    fn test_day10_part2_simple() {
        let map = AsteroidMap::from(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );

        part2(&map);
    }

    #[test]
    fn test_day10_part2() {
        let map = AsteroidMap::from(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        );

        let ast = map.destroy_asteroids(Vector2 { x: 11, y: 13 }, 200);

        assert_eq!(ast.position, Vector2 { x: 8, y: 2 });
    }
}
