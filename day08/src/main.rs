use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseError {
    UnknownOperation(String),
    BadArgument(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::BadArgument(e)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    ACC(i64),
    JMP(i64),
    NOP(i64),
}

impl Instruction {
    fn toggle(self) -> Option<Instruction> {
        match self {
            Instruction::ACC(_argument) => None,
            Instruction::JMP(argument) => Some(Instruction::NOP(argument)),
            Instruction::NOP(argument) => Some(Instruction::JMP(argument)),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, argument) = s.split_at(3);
        let argument = &argument[1..];
        let argument: i64 = argument.parse()?;
        match operation {
            "acc" => Ok(Instruction::ACC(argument)),
            "jmp" => Ok(Instruction::JMP(argument)),
            "nop" => Ok(Instruction::NOP(argument)),
            _ => Err(ParseError::UnknownOperation(operation.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Result<Vec<_>, _> = s.lines().map(|s| s.parse::<Instruction>()).collect();
        Ok(Program {
            instructions: instructions?,
        })
    }
}

struct Interpreter {
    program: Program,
    accumulator: i64,
    instruction_pointer: usize,
    seen_instructions: HashSet<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum StepOutcome {
    Continue,
    InfiniteLoop(i64),
    Terminate(i64),
}

impl Interpreter {
    fn new(program: Program) -> Self {
        Interpreter {
            program,
            accumulator: 0,
            instruction_pointer: 0,
            seen_instructions: HashSet::new(),
        }
    }

    fn step(&mut self) -> StepOutcome {
        if self.seen_instructions.contains(&self.instruction_pointer) {
            return StepOutcome::InfiniteLoop(self.accumulator);
        }
        self.seen_instructions.insert(self.instruction_pointer);
        let instruction = match self.program.instructions.get(self.instruction_pointer) {
            Some(instruction) => instruction,
            None => return StepOutcome::Terminate(self.accumulator),
        };
        match instruction {
            Instruction::ACC(argument) => {
                self.accumulator += argument;
                self.instruction_pointer += 1;
            }
            Instruction::JMP(argument) => {
                self.instruction_pointer = ((self.instruction_pointer as i64) + argument)
                    .try_into()
                    .expect("JMP out of bounds");
            }
            Instruction::NOP(_argument) => {
                self.instruction_pointer += 1;
            }
        };
        StepOutcome::Continue
    }
}

fn part1(program: Program) -> i64 {
    let mut interpreter = Interpreter::new(program);
    loop {
        match interpreter.step() {
            StepOutcome::Continue => (),
            StepOutcome::InfiniteLoop(accumulator) => break accumulator,
            StepOutcome::Terminate(accumulator) => break accumulator,
        }
    }
}

fn part2(program: Program) -> i64 {
    for index in 0..program.instructions.len() {
        let mut toggled_program = Program {
            instructions: program.instructions.clone()
        };
        if let Some(toggled_instruction) = toggled_program.instructions[index].toggle() {
            toggled_program.instructions[index] = toggled_instruction;
        } else {
            continue;
        }


        let mut interpreter = Interpreter::new(toggled_program);
        loop {
            match interpreter.step() {
                StepOutcome::Continue => (),
                StepOutcome::InfiniteLoop(_accumulator) => break,
                StepOutcome::Terminate(accumulator) => return accumulator,
            }
        }
    }
    panic!("No solution found!");
}

fn main() -> Result<(), Box<dyn Error>> {
    let program: Program = fs::read_to_string("input")?.parse()?;
    println!("{}", part1(program.clone()));
    println!("{}", part2(program.clone()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_instruction() {
        assert_eq!("acc 0".parse::<Instruction>().unwrap(), Instruction::ACC(0));
        assert_eq!("acc 1".parse::<Instruction>().unwrap(), Instruction::ACC(1));
        assert_eq!(
            "acc +1".parse::<Instruction>().unwrap(),
            Instruction::ACC(1)
        );
        assert_eq!(
            "acc -1".parse::<Instruction>().unwrap(),
            Instruction::ACC(-1)
        );
        assert_eq!(
            "acc 65536".parse::<Instruction>().unwrap(),
            Instruction::ACC(65536)
        );
        assert_eq!(
            "jmp -10".parse::<Instruction>().unwrap(),
            Instruction::JMP(-10)
        );
        assert_eq!(
            "jmp +12".parse::<Instruction>().unwrap(),
            Instruction::JMP(12)
        );
        assert_eq!(
            "nop +1234".parse::<Instruction>().unwrap(),
            Instruction::NOP(1234)
        );
    }

    #[test]
    fn test_parse_program() {
        assert_eq!(
            "".parse::<Program>().unwrap(),
            Program {
                instructions: vec![]
            }
        );
        assert_eq!(
            "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"
            .trim()
            .parse::<Program>()
            .unwrap(),
            Program {
                instructions: vec![
                    Instruction::NOP(0),
                    Instruction::ACC(1),
                    Instruction::JMP(4),
                    Instruction::ACC(3),
                    Instruction::JMP(-3),
                    Instruction::ACC(-99),
                    Instruction::ACC(1),
                    Instruction::JMP(-4),
                    Instruction::ACC(6),
                ]
            }
        );
    }

    #[test]
    fn test_step_program() {
        let mut interpreter = Interpreter::new(Program {
            instructions: vec![
                Instruction::NOP(0),
                Instruction::ACC(1),
                Instruction::JMP(4),
                Instruction::ACC(3),
                Instruction::JMP(-3),
                Instruction::ACC(-99),
                Instruction::ACC(1),
                Instruction::JMP(-4),
                Instruction::ACC(6),
            ],
        });
        assert_eq!(0, interpreter.accumulator);
        assert_eq!(0, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(0, interpreter.accumulator);
        assert_eq!(1, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(1, interpreter.accumulator);
        assert_eq!(2, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(1, interpreter.accumulator);
        assert_eq!(6, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(2, interpreter.accumulator);
        assert_eq!(7, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(2, interpreter.accumulator);
        assert_eq!(3, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(5, interpreter.accumulator);
        assert_eq!(4, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::Continue, interpreter.step());
        assert_eq!(5, interpreter.accumulator);
        assert_eq!(1, interpreter.instruction_pointer);

        assert_eq!(StepOutcome::InfiniteLoop(5), interpreter.step());
    }

    #[test]
    fn test_part1() {
        let program = Program {
            instructions: vec![
                Instruction::NOP(0),
                Instruction::ACC(1),
                Instruction::JMP(4),
                Instruction::ACC(3),
                Instruction::JMP(-3),
                Instruction::ACC(-99),
                Instruction::ACC(1),
                Instruction::JMP(-4),
                Instruction::ACC(6),
            ],
        };

        assert_eq!(5, part1(program));
    }

    #[test]
    fn test_part2() {
        let program = Program {
            instructions: vec![
                Instruction::NOP(0),
                Instruction::ACC(1),
                Instruction::JMP(4),
                Instruction::ACC(3),
                Instruction::JMP(-3),
                Instruction::ACC(-99),
                Instruction::ACC(1),
                Instruction::JMP(-4),
                Instruction::ACC(6),
            ],
        };

        assert_eq!(8, part2(program));
    }
}
