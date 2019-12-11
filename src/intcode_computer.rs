use std::collections::VecDeque;
use std::io;

pub type SIZE = i128;

#[derive(PartialEq, Debug)]
enum ParamMode {
  Immediate,
  Position,
}

impl ParamMode {
  fn from(param: SIZE) -> Self {
    match param {
      0 => ParamMode::Position,
      1 => ParamMode::Immediate,
      _ => panic!("Unknown ParamMode"),
    }
  }
}

enum Instruction {
  Add(SIZE, SIZE, SIZE),
  Multiply(SIZE, SIZE, SIZE),
  Input(SIZE),
  Output(SIZE),
  JmpTrue(SIZE, SIZE),
  JmpFalse(SIZE, SIZE),
  JmpLessThan(SIZE, SIZE, SIZE),
  JmpEquals(SIZE, SIZE, SIZE),
  Halt,
}

impl Instruction {
  fn from(instruction: SIZE) -> (SIZE, ParamMode, ParamMode, ParamMode) {
    let opcode = instruction % 100;
    let a = ParamMode::from((instruction / 100) % 10);
    let b = ParamMode::from((instruction / 1000) % 10);
    let c = ParamMode::from((instruction / 10000) % 10);
    (opcode, a, b, c)
  }
}

enum State {
  Running,
  Halted,
  RequireInput,
}

pub struct CPU {
  instruction_pointer: SIZE,
  memory: Vec<SIZE>,
  input: VecDeque<SIZE>,
  pub output: Vec<SIZE>,
  allow_print: bool,
}

impl CPU {
  pub fn new(memory: Vec<SIZE>) -> Self {
    CPU {
      instruction_pointer: 0,
      memory,
      input: VecDeque::new(),
      output: Vec::new(),
      allow_print: cfg!(test),
    }
  }

  fn fetch(&mut self) -> SIZE {
    let instruction = self.get(self.instruction_pointer);
    self.instruction_pointer += 1;
    instruction
  }

  fn get(&self, addr: SIZE) -> SIZE {
    self.memory[addr as usize]
  }

  fn read(&mut self, mode: ParamMode) -> SIZE {
    let val = self.fetch();

    match mode {
      ParamMode::Position => self.get(val),
      ParamMode::Immediate => val,
    }
  }

  fn write(&mut self, mode: ParamMode) -> SIZE {
    let val = self.fetch();

    match mode {
      ParamMode::Position => val,
      ParamMode::Immediate => {
        panic!("Parameters that an instruction writes to will never be in immediate mode")
      }
    }
  }

  fn set(&mut self, addr: SIZE, val: SIZE) {
    self.memory[addr as usize] = val;
  }

  fn decode(&mut self, (opcode, a, b, c): (SIZE, ParamMode, ParamMode, ParamMode)) -> Instruction {
    match opcode {
      1 => Instruction::Add(self.read(a), self.read(b), self.write(c)),
      2 => Instruction::Multiply(self.read(a), self.read(b), self.write(c)),
      3 => Instruction::Input(self.write(a)),
      4 => Instruction::Output(self.read(a)),
      5 => Instruction::JmpTrue(self.read(a), self.read(b)),
      6 => Instruction::JmpFalse(self.read(a), self.read(b)),
      7 => Instruction::JmpLessThan(self.read(a), self.read(b), self.write(c)),
      8 => Instruction::JmpEquals(self.read(a), self.read(b), self.write(c)),
      99 => Instruction::Halt,
      _ => panic!("Unknown opcode"),
    }
  }

  fn execute(&mut self, instruction: SIZE) -> State {
    match self.decode(Instruction::from(instruction)) {
      Instruction::Add(a, b, c) => {
        self.set(c, a + b);
      }
      Instruction::Multiply(a, b, c) => {
        self.set(c, a * b);
      }
      Instruction::Input(a) => {
        match self.input.pop_front() {
          Some(value) => self.set(a, value),
          None => return State::RequireInput,
        };
      }
      Instruction::Output(a) => {
        if self.allow_print {
          println!("output: {}", a);
        }
        self.output.push(a);
      }
      Instruction::JmpTrue(a, b) => {
        if a != 0 {
          self.instruction_pointer = b;
        }
      }
      Instruction::JmpFalse(a, b) => {
        if a == 0 {
          self.instruction_pointer = b;
        }
      }
      Instruction::JmpLessThan(a, b, c) => self.set(c, if a < b { 1 } else { 0 }),
      Instruction::JmpEquals(a, b, c) => self.set(c, if a == b { 1 } else { 0 }),
      Instruction::Halt => return State::Halted,
    }
    State::Running
  }

  fn step(&mut self) -> State {
    let instruction = self.fetch();
    self.execute(instruction)
  }

  pub fn run(&mut self, input: Option<&[SIZE]>) -> Vec<SIZE> {
    if let Some(value) = input {
      for val in value.iter() {
        self.input.push_back(*val);
      }
    }

    loop {
      match self.step() {
        State::Halted => break,
        State::Running => (),
        State::RequireInput => {
          println!("Enter input:");
          let mut input = String::new();
          io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
          match input.parse() {
            Ok(value) => self.input.push_back(value),
            Err(_) => break,
          }
        }
      }
    }

    self.output.clone()
  }
}

pub fn parse_input(input: &str) -> Vec<SIZE> {
  input.split(',').map(|x| x.parse().unwrap()).collect()
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
    let (opcode, p1, p2, p3) = Instruction::from(1002);
    assert_eq!(opcode, 2);
    assert_eq!(p1, ParamMode::Position);
    assert_eq!(p2, ParamMode::Immediate);
    assert_eq!(p3, ParamMode::Position);

    let (opcode, p1, p2, p3) = Instruction::from(2);
    assert_eq!(opcode, 2);
    assert_eq!(p1, ParamMode::Position);
    assert_eq!(p2, ParamMode::Position);
    assert_eq!(p3, ParamMode::Position);

    let (opcode, p1, p2, p3) = Instruction::from(104);
    assert_eq!(opcode, 4);
    assert_eq!(p1, ParamMode::Immediate);
    assert_eq!(p2, ParamMode::Position);
    assert_eq!(p3, ParamMode::Position);
  }

  fn test_cpu(code: Vec<SIZE>, input: SIZE, expected_output: SIZE) -> bool {
    let mut cpu = CPU::new(code);
    let output = cpu.run(Some(&[input]));

    *output.last().unwrap() == expected_output
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
      3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
      1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
      1, 46, 98, 99,
    ];

    let input = 7;
    assert!(test_cpu(code.clone(), input, 999), " < 8");

    let input = 8;
    assert!(test_cpu(code.clone(), input, 1000), " = 8");

    let input = 9;
    assert!(test_cpu(code.clone(), input, 1001), " > 8");
  }
}
