use num::Integer;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn energy(&self) -> i32 {
        let potential_energy =
            self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kinetic_energy = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        potential_energy * kinetic_energy
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    moons: Vec<Moon>,
}

impl State {
    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.position += moon.velocity;
        }
    }

    fn apply_gravity(&mut self) {
        let others = self.moons.clone();
        for moon in self.moons.iter_mut() {
            for other in others.iter() {
                match moon.position.x.cmp(&other.position.x) {
                    Ordering::Greater => moon.velocity.x -= 1,
                    Ordering::Less => moon.velocity.x += 1,
                    Ordering::Equal => (),
                }
                match moon.position.y.cmp(&other.position.y) {
                    Ordering::Greater => moon.velocity.y -= 1,
                    Ordering::Less => moon.velocity.y += 1,
                    Ordering::Equal => (),
                }
                match moon.position.z.cmp(&other.position.z) {
                    Ordering::Greater => moon.velocity.z -= 1,
                    Ordering::Less => moon.velocity.z += 1,
                    Ordering::Equal => (),
                }
            }
        }
    }

    fn total_energy(&self) -> i32 {
        let mut total_energy = 0;
        for moon in self.moons.iter() {
            total_energy += moon.energy();
        }
        total_energy
    }

    fn find_steps_repeating(&self) -> i64 {
        let initial_state = self.clone();
        let mut state = initial_state.clone();
        state.step();

        let mut steps: i64 = 1;
        let mut step_x = 0;
        let mut step_y = 0;
        let mut step_z = 0;

        loop {
            state.step();
            steps += 1;

            let cmp_map: (i32, i32, i32) = state
                .moons
                .iter()
                .enumerate()
                .map(|(i, moon)| {
                    // compare each axis of each moon
                    let initial_moon = initial_state.moons[i];
                    (
                        moon.position.x == initial_moon.position.x && moon.velocity.x == 0,
                        moon.position.y == initial_moon.position.y && moon.velocity.y == 0,
                        moon.position.z == initial_moon.position.z && moon.velocity.z == 0,
                    )
                })
                .fold((0, 0, 0), |acc, (x, y, z)| {
                    // count all moons that have identical axis
                    (
                        acc.0 + if x { 1 } else { 0 },
                        acc.1 + if y { 1 } else { 0 },
                        acc.2 + if z { 1 } else { 0 },
                    )
                });

            if step_x == 0 && cmp_map.0 == 4 {
                step_x = steps;
            }
            if step_y == 0 && cmp_map.1 == 4 {
                step_y = steps;
            }
            if step_z == 0 && cmp_map.2 == 4 {
                step_z = steps;
            }

            if step_x != 0 && step_y != 0 && step_z != 0 {
                break;
            }
        }

        // calculate least common multiplicator
        let mut lcm: i64 = step_x;
        for i in &[step_y, step_z] {
            lcm = lcm.lcm(i)
        }

        lcm
    }
}

#[aoc_generator(day12)]
fn generator_input(input: &str) -> State {
    let mut result = Vec::new();

    let pattern = regex::Regex::new(r"^<x=(.+), y=(.+), z=(.+)>$").unwrap();

    for line in input.lines() {
        let groups = pattern.captures(line).unwrap();
        let moon = Moon {
            position: Vec3 {
                x: groups.get(1).unwrap().as_str().parse().unwrap(),
                y: groups.get(2).unwrap().as_str().parse().unwrap(),
                z: groups.get(3).unwrap().as_str().parse().unwrap(),
            },
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        };
        result.push(moon);
    }

    State { moons: result }
}

#[aoc(day12, part1)]
fn part1(input: &State) -> i32 {
    let mut state = input.clone();
    for _ in 0..1000 {
        state.step();
    }

    state.total_energy()
}

#[aoc(day12, part2)]
fn part2(input: &State) -> i64 {
    input.find_steps_repeating()
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Vec3 {
        fn from(x: i32, y: i32, z: i32) -> Self {
            Vec3 { x, y, z }
        }
    }

    #[test]
    fn test_day12_2_step() {
        let mut input = generator_input(
            "<x=-1, y=0, z=2>\n\
             <x=2, y=-10, z=-7>\n\
             <x=4, y=-8, z=8>\n\
             <x=3, y=5, z=-1>",
        );

        input.step();
        let moons = input.moons.to_vec();

        assert_eq!(moons[0].velocity, Vec3::from(3, -1, -1));
        assert_eq!(moons[1].velocity, Vec3::from(1, 3, 3));
        assert_eq!(moons[2].velocity, Vec3::from(-3, 1, -3));
        assert_eq!(moons[3].velocity, Vec3::from(-1, -3, 1));

        assert_eq!(moons[0].position, Vec3::from(2, -1, 1));
        assert_eq!(moons[1].position, Vec3::from(3, -7, -4));
        assert_eq!(moons[2].position, Vec3::from(1, -7, 5));
        assert_eq!(moons[3].position, Vec3::from(2, 2, 0));

        input.step();
        let moons = input.moons.to_vec();

        assert_eq!(moons[0].velocity, Vec3::from(3, -2, -2));
        assert_eq!(moons[1].velocity, Vec3::from(-2, 5, 6));
        assert_eq!(moons[2].velocity, Vec3::from(0, 3, -6));
        assert_eq!(moons[3].velocity, Vec3::from(-1, -6, 2));

        assert_eq!(moons[0].position, Vec3::from(5, -3, -1));
        assert_eq!(moons[1].position, Vec3::from(1, -2, 2));
        assert_eq!(moons[2].position, Vec3::from(1, -4, -1));
        assert_eq!(moons[3].position, Vec3::from(1, -4, 2));
    }

    #[test]
    fn test_day12_part1_10_steps() {
        let input = generator_input(
            "<x=-1, y=0, z=2>\n\
             <x=2, y=-10, z=-7>\n\
             <x=4, y=-8, z=8>\n\
             <x=3, y=5, z=-1>",
        );

        let mut state = input;
        for _ in 0..10 {
            state.step();
        }

        let moons = state.moons;

        assert_eq!(moons[0].velocity, Vec3::from(-3, -2, 1));
        assert_eq!(moons[1].velocity, Vec3::from(-1, 1, 3));
        assert_eq!(moons[2].velocity, Vec3::from(3, 2, -3));
        assert_eq!(moons[3].velocity, Vec3::from(1, -1, -1));

        assert_eq!(moons[0].position, Vec3::from(2, 1, -3));
        assert_eq!(moons[1].position, Vec3::from(1, -8, 0));
        assert_eq!(moons[2].position, Vec3::from(3, -6, 1));
        assert_eq!(moons[3].position, Vec3::from(2, -0, 4));
    }

    #[test]
    fn test_day12_part1_total_energy() {
        let input = generator_input(
            "<x=-1, y=0, z=2>\n\
             <x=2, y=-10, z=-7>\n\
             <x=4, y=-8, z=8>\n\
             <x=3, y=5, z=-1>",
        );

        let mut state = input;
        for _ in 0..10 {
            state.step();
        }

        assert_eq!(state.moons[0].energy(), 36);

        assert_eq!(state.total_energy(), 179);
    }

    #[test]
    fn test_day12_part2() {
        let input = generator_input(
            "<x=-1, y=0, z=2>\n\
             <x=2, y=-10, z=-7>\n\
             <x=4, y=-8, z=8>\n\
             <x=3, y=5, z=-1>",
        );

        let steps = input.find_steps_repeating();

        assert_eq!(steps, 2772);
    }

    #[test]
    fn test_day12_part2_long() {
        let input = generator_input(
            "<x=-8, y=-10, z=0>\n\
             <x=5, y=5, z=10>\n\
             <x=2, y=-7, z=3>\n\
             <x=9, y=-8, z=-3>",
        );

        let steps = input.find_steps_repeating();

        assert_eq!(steps, 4_686_774_924);
    }
}
