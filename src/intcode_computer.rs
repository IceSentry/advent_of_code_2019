use std::io;

#[derive(PartialEq, Debug)]
enum ParamMode {
  Immediate,
  Position,
}

impl ParamMode {
  fn from(param: i32) -> Self {
    match param {
      0 => ParamMode::Position,
      1 => ParamMode::Immediate,
      _ => panic!("Unknown ParamMode"),
    }
  }
}

enum Instruction {
  Add(i32, i32, i32),
  Multiply(i32, i32, i32),
  Input(i32),
  Output(i32),
  JmpTrue(i32, i32),
  JmpFalse(i32, i32),
  JmpLessThan(i32, i32, i32),
  JmpEquals(i32, i32, i32),
  Halt,
}

impl Instruction {
  fn from(instruction: i32) -> (i32, ParamMode, ParamMode, ParamMode) {
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
}

pub struct CPU {
  instruction_pointer: i32,
  memory: Vec<i32>,
  input: Option<i32>,
  pub output: Vec<i32>,
}

impl CPU {
  pub fn new(memory: Vec<i32>, input: Option<i32>) -> Self {
    CPU {
      instruction_pointer: 0,
      memory,
      input,
      output: Vec::new(),
    }
  }

  fn fetch(&mut self) -> i32 {
    let instruction = self.get(self.instruction_pointer);
    self.instruction_pointer += 1;
    instruction
  }

  fn get(&self, addr: i32) -> i32 {
    self.memory[addr as usize]
  }

  fn get_param_value(&mut self, mode: ParamMode) -> i32 {
    let val = self.fetch();

    match mode {
      ParamMode::Position => self.get(val),
      ParamMode::Immediate => val,
    }
  }

  fn set(&mut self, addr: i32, val: i32) {
    self.memory[addr as usize] = val;
  }

  fn decode(&mut self, (opcode, a, b, _c): (i32, ParamMode, ParamMode, ParamMode)) -> Instruction {
    match opcode {
      1 => Instruction::Add(
        self.get_param_value(a),
        self.get_param_value(b),
        self.fetch(),
      ),
      2 => Instruction::Multiply(
        self.get_param_value(a),
        self.get_param_value(b),
        self.fetch(),
      ),
      3 => Instruction::Input(self.fetch()),
      4 => Instruction::Output(self.get_param_value(a)),
      5 => Instruction::JmpTrue(self.get_param_value(a), self.get_param_value(b)),
      6 => Instruction::JmpFalse(self.get_param_value(a), self.get_param_value(b)),
      7 => Instruction::JmpLessThan(
        self.get_param_value(a),
        self.get_param_value(b),
        self.fetch(),
      ),
      8 => Instruction::JmpEquals(
        self.get_param_value(a),
        self.get_param_value(b),
        self.fetch(),
      ),
      99 => Instruction::Halt,
      _ => panic!("Unknown opcode"),
    }
  }

  fn execute(&mut self, instruction: i32) -> State {
    match self.decode(Instruction::from(instruction)) {
      Instruction::Add(a, b, c) => {
        self.set(c, a + b);
      }
      Instruction::Multiply(a, b, c) => {
        self.set(c, a * b);
      }
      Instruction::Input(a) => {
        let value = match self.input {
          Some(value) => value,
          None => {
            println!("Enter input:");
            let mut input = String::new();
            io::stdin()
              .read_line(&mut input)
              .expect("Failed to read line");
            match input.parse() {
              Ok(value) => value,
              Err(_) => return State::Halted,
            }
          }
        };

        self.set(a, value)
      }
      Instruction::Output(a) => {
        println!("output: {}", a);
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

  pub fn run(&mut self) {
    loop {
      match self.step() {
        State::Halted => break,
        State::Running => (),
      }
    }
  }
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn parse_code(input: &[i32]) -> Vec<i32> {
  let mut cpu = CPU::new(input.to_owned(), None);
  cpu.run();

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

  fn test_cpu(code: Vec<i32>, input: i32, expected_output: i32) -> bool {
    let mut cpu = CPU::new(code, Some(input));

    cpu.run();
    let output = cpu.output.last().unwrap();

    *output == expected_output
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
