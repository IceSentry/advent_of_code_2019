use crate::intcode_computer::{parse_input, State, CPU, SIZE};

use permute;

#[aoc_generator(day7)]
fn generator_input(input: &str) -> Vec<SIZE> {
    parse_input(input)
}

fn init_amplifiers(code: &[SIZE], phase_settings: &[SIZE]) -> Vec<CPU> {
    phase_settings
        .iter()
        .map(|phase_setting| {
            let mut cpu = CPU::new(code.to_owned());
            cpu.halt_on_output = true;
            cpu.input.push_back(*phase_setting);
            cpu
        })
        .collect()
}

fn test_phase_setting(code: &[SIZE], phase_setting: Vec<SIZE>) -> SIZE {
    let mut cpus = init_amplifiers(code, &phase_setting);

    let mut phase_signal = 0;
    for (phase_index, _) in phase_setting.iter().enumerate().cycle() {
        let cpu = &mut cpus[phase_index];
        if let State::Halt = cpu.run(Some(phase_signal)) {
            break;
        };
        phase_signal = cpu.output.pop().expect("No output");
    }

    phase_signal
}

#[aoc(day7, part1)]
fn part1(input: &[SIZE]) -> SIZE {
    let phase_settings = vec![0, 1, 2, 3, 4];

    permute::permute(phase_settings)
        .iter()
        .map(|phase_setting| test_phase_setting(input, phase_setting.to_owned()))
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[SIZE]) -> SIZE {
    let phase_settings = vec![5, 6, 7, 8, 9];

    permute::permute(phase_settings)
        .iter()
        .map(|phase_setting| test_phase_setting(input, phase_setting.to_owned()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::test_phase_setting;

    #[test]
    fn test_day7_part1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let phase_setting = vec![4, 3, 2, 1, 0];

        assert_eq!(test_phase_setting(&input, phase_setting), 43210);

        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let phase_setting = vec![0, 1, 2, 3, 4];

        assert_eq!(test_phase_setting(&input, phase_setting), 54321);

        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let phase_setting = vec![1, 0, 4, 3, 2];

        assert_eq!(test_phase_setting(&input, phase_setting), 65210);
    }

    #[test]
    fn test_day7_part2() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let phase_setting = vec![9, 8, 7, 6, 5];

        assert_eq!(test_phase_setting(&input, phase_setting), 139_629_729);

        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let phase_setting = vec![9, 7, 8, 5, 6];

        assert_eq!(test_phase_setting(&input, phase_setting), 18216);
    }
}
