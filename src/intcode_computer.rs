use std::collections::VecDeque;

pub type SIZE = i64;

#[derive(PartialEq, Debug)]
enum ParamMode {
    Immediate,
    Position,
    Relative,
}

impl ParamMode {
    fn from(param: SIZE) -> Self {
        match param {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unknown ParamMode {}", param),
        }
    }
}

enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JmpTrue,
    JmpFalse,
    JmpLessThan,
    JmpEquals,
    SetRelativeBase,
    Halt,
}

impl Opcode {
    fn from(opcode: SIZE) -> Self {
        match opcode {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JmpTrue,
            6 => Opcode::JmpFalse,
            7 => Opcode::JmpLessThan,
            8 => Opcode::JmpEquals,
            9 => Opcode::SetRelativeBase,
            99 => Opcode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn parse_instruction(instruction: SIZE) -> (SIZE, ParamMode, ParamMode, ParamMode) {
    let opcode = instruction % 100;
    let a = ParamMode::from((instruction / 100) % 10);
    let b = ParamMode::from((instruction / 1000) % 10);
    let c = ParamMode::from((instruction / 10000) % 10);
    (opcode, a, b, c)
}

#[derive(PartialEq)]
pub enum State {
    Running,
    Halt,
    Input,
    Output(SIZE),
}

pub struct CPU {
    pub input: VecDeque<SIZE>,
    pub output: Vec<SIZE>,
    pub halt_on_output: bool,
    instruction_pointer: SIZE,
    pub memory: Vec<SIZE>,
    relative_base: SIZE,
    pub allow_print: bool,
}

impl CPU {
    pub fn new(memory: Vec<SIZE>) -> Self {
        CPU {
            input: VecDeque::new(),
            output: Vec::new(),
            halt_on_output: false,
            instruction_pointer: 0,
            memory,
            relative_base: 0,
            allow_print: cfg!(test),
        }
    }

    fn fetch(&mut self) -> SIZE {
        let instruction = self.get(self.instruction_pointer);
        self.instruction_pointer += 1;
        instruction
    }

    fn get(&mut self, addr: SIZE) -> SIZE {
        self.memory_size_check(addr);
        self.memory[addr as usize]
    }

    fn set(&mut self, addr: SIZE, val: SIZE) {
        self.memory_size_check(addr);
        self.memory[addr as usize] = val;
    }

    fn memory_size_check(&mut self, addr: SIZE) {
        if self.memory.get(addr as usize).is_none() {
            self.memory.resize(addr as usize + 1, 0)
        }
    }

    fn read_param(&mut self, mode: ParamMode) -> SIZE {
        let value = self.fetch();

        match mode {
            ParamMode::Position => self.get(value),
            ParamMode::Immediate => value,
            ParamMode::Relative => self.get(self.relative_base + value),
        }
    }

    fn read_params(&mut self, a: ParamMode, b: ParamMode) -> (SIZE, SIZE) {
        (self.read_param(a), self.read_param(b))
    }

    fn write(&mut self, mode: ParamMode, value: SIZE) {
        let addr = self.fetch();

        match mode {
            ParamMode::Position => self.set(addr, value),
            ParamMode::Immediate => {
                panic!("Parameters that an instruction writes to should never be in immediate mode")
            }
            ParamMode::Relative => self.set(self.relative_base + addr, value),
        }
    }

    fn execute(&mut self, instruction: SIZE) -> State {
        let (opcode, a, b, c) = parse_instruction(instruction);

        match Opcode::from(opcode) {
            Opcode::Add => {
                let (a, b) = self.read_params(a, b);
                self.write(c, a + b);
            }
            Opcode::Multiply => {
                let (a, b) = self.read_params(a, b);
                self.write(c, a * b);
            }
            Opcode::Input => {
                match self.input.pop_front() {
                    Some(value) => {
                        if self.allow_print {
                            println!("input: {}", value)
                        }
                        self.write(a, value)
                    }
                    None => {
                        self.instruction_pointer -= 1;
                        return State::Input;
                    }
                };
            }
            Opcode::Output => {
                let a = self.read_param(a);
                if self.allow_print {
                    println!("output: {}", a);
                }
                self.output.push(a);
                if self.halt_on_output {
                    // TODO investigate stopping on N output len() and return the full output
                    // also consider using an enum of 1 output and vec output
                    return State::Output(self.output.pop().unwrap());
                }
            }
            Opcode::JmpTrue => {
                let (a, b) = self.read_params(a, b);
                if a != 0 {
                    self.instruction_pointer = b;
                }
            }
            Opcode::JmpFalse => {
                let (a, b) = self.read_params(a, b);
                if a == 0 {
                    self.instruction_pointer = b;
                }
            }
            Opcode::JmpLessThan => {
                let (a, b) = self.read_params(a, b);
                self.write(c, if a < b { 1 } else { 0 });
            }
            Opcode::JmpEquals => {
                let (a, b) = self.read_params(a, b);
                self.write(c, if a == b { 1 } else { 0 });
            }
            Opcode::SetRelativeBase => {
                let a = self.read_param(a);
                self.relative_base += a;
            }
            Opcode::Halt => return State::Halt,
        }

        State::Running
    }

    pub fn step(&mut self) -> State {
        let instruction = self.fetch();
        self.execute(instruction)
    }

    pub fn run(&mut self, input: Option<SIZE>) -> State {
        // TODO maybe have a run() and run_with_input()
        if let Some(value) = input {
            self.input.push_back(value);
        }

        loop {
            let state = self.step();
            match state {
                State::Running => (),
                _ => return state,
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<SIZE> {
    input
        .split(',')
        .map(|x| x.parse().expect("Not a number"))
        .collect()
}

pub fn parse_code(input: &[SIZE]) -> Vec<SIZE> {
    let mut cpu = CPU::new(input.to_owned());
    cpu.run(None);

    cpu.memory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code_day2() {
        let input = vec![1, 0, 0, 0, 99];
        let result = parse_code(&input);
        assert_eq!(vec![2, 0, 0, 0, 99], result, "simple add");

        let input = vec![2, 3, 0, 3, 99];
        let result = parse_code(&input);
        assert_eq!(vec![2, 3, 0, 6, 99], result, "simple mul");

        let input = vec![2, 4, 4, 5, 99, 0];
        let result = parse_code(&input);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], result, "mul result after 99");

        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = parse_code(&input);
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            result,
            "multiple instructions"
        );
    }

    #[test]
    fn test_parse_instruction() {
        let (opcode, p1, p2, p3) = parse_instruction(1002);
        assert_eq!(opcode, 2);
        assert_eq!(p1, ParamMode::Position);
        assert_eq!(p2, ParamMode::Immediate);
        assert_eq!(p3, ParamMode::Position);

        let (opcode, p1, p2, p3) = parse_instruction(2);
        assert_eq!(opcode, 2);
        assert_eq!(p1, ParamMode::Position);
        assert_eq!(p2, ParamMode::Position);
        assert_eq!(p3, ParamMode::Position);

        let (opcode, p1, p2, p3) = parse_instruction(104);
        assert_eq!(opcode, 4);
        assert_eq!(p1, ParamMode::Immediate);
        assert_eq!(p2, ParamMode::Position);
        assert_eq!(p3, ParamMode::Position);
    }

    fn test_cpu(code: Vec<SIZE>, input: SIZE, expected_output: SIZE) -> bool {
        let mut cpu = CPU::new(code);
        cpu.run(Some(input));

        *cpu.output.last().unwrap() == expected_output
    }

    #[test]
    fn test_io() {
        let code = vec![3, 0, 4, 0, 99];
        let input = 42;
        assert!(test_cpu(code, input, input));

        let code = vec![3, 0, 104, 999, 99];
        let input = 42;
        assert!(test_cpu(code, input, 999));
    }

    #[test]
    fn test_equals() {
        // position mode
        let code = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = 8;
        assert!(test_cpu(code, input, 1));

        // immediate mode
        let code = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input = 8;
        assert!(test_cpu(code, input, 1));
    }

    #[test]
    fn test_less_than() {
        // position mode
        let code = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = 7;
        assert!(test_cpu(code, input, 1));

        // immediate mode
        let code = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input = 7;
        assert!(test_cpu(code, input, 1));
    }

    #[test]
    fn test_jmp() {
        // position mode
        let code = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let input = 42;
        assert!(test_cpu(code, input, 1));

        // immediate mode
        let code = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let input = 42;
        assert!(test_cpu(code, input, 1));
    }

    #[test]
    fn test_large() {
        let code = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let input = 7;
        assert!(test_cpu(code.clone(), input, 999), " < 8");

        let input = 8;
        assert!(test_cpu(code.clone(), input, 1000), " = 8");

        let input = 9;
        assert!(test_cpu(code, input, 1001), " > 8");
    }

    #[test]
    fn test_relative() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut cpu = CPU::new(code.clone());
        cpu.run(None);
        assert_eq!(cpu.output, code);

        let code = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let mut cpu = CPU::new(code);
        cpu.run(None);
        println!("{:?}", cpu.output);
        assert_eq!(cpu.output[0], 1_219_070_632_396_864);

        let code = vec![104, 1_125_899_906_842_624, 99];
        let mut cpu = CPU::new(code);
        cpu.run(None);
        println!("{:?}", cpu.output);
        assert_eq!(cpu.output[0], 1_125_899_906_842_624);
    }
}
