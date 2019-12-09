use std::str::FromStr;
use std::convert::{TryFrom, TryInto};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCode {
    pub memory: Vec<isize>,
    pub inputs: VecDeque<isize>,
    ins_ptr: usize,
    rel_offset: isize,
}
impl FromStr for IntCode {
    type Err = <isize as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the line into integers
        let mut memory = s.split(',').map(|i| i.parse()).collect::<Result<Vec<isize>, _>>()?;
        memory.extend_from_slice(&[0; 1000]);
        Ok(IntCode { memory, inputs: VecDeque::new(), ins_ptr: 0, rel_offset: 0})
    }
}

enum InputParamMode {
    Position,
    Immediate,
    Relative
}

#[derive(Debug)]
struct InvalidParamMode(isize);

impl TryFrom<isize> for InputParamMode {
    type Error = InvalidParamMode;
    fn try_from(i: isize) -> Result<Self, Self::Error> {
        use InputParamMode::*;
        match i {
            0 => Ok(Position),
            1 => Ok(Immediate),
            2 => Ok(Relative),
            invalid => Err(InvalidParamMode(invalid))
        }
    }
}
enum OutputParamMode {
    Position,
    Relative
}

impl TryFrom<isize> for OutputParamMode {
    type Error = InvalidParamMode;
    fn try_from(i: isize) -> Result<Self, Self::Error> {
        use OutputParamMode::*;
        match i {
            0 => Ok(Position),
            2 => Ok(Relative),
            invalid => Err(InvalidParamMode(invalid))
        }
    }
}

enum OpCode {
    Add(InputParamMode, InputParamMode, OutputParamMode),
    Multiply(InputParamMode, InputParamMode, OutputParamMode),
    Input(OutputParamMode),
    Output(InputParamMode),
    JumpIfTrue(InputParamMode, InputParamMode),
    JumpIfFalse(InputParamMode, InputParamMode),
    LessThan(InputParamMode, InputParamMode, OutputParamMode),
    Equals(InputParamMode, InputParamMode, OutputParamMode),
    AdjustRelBase(InputParamMode),
    Halt
}
#[derive(Debug)]
enum OpCodeParseError {
    InvalidParam1Mode(InvalidParamMode),
    InvalidParam2Mode(InvalidParamMode),
    InvalidParam3Mode(InvalidParamMode),
    InvalidOpCode(isize)
}
impl TryFrom<isize> for OpCode {
    type Error = OpCodeParseError;

    fn try_from(i: isize) -> Result<Self, Self::Error> {
        use OpCode::*;
        use OpCodeParseError::*;
        // Get the last 2 digits of the op code
        let opcode = i % 100;
        match opcode {
            1 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                let c_mode = ((i % 100000) / 10000).try_into().map_err(InvalidParam3Mode)?;
                Ok(Add(a_mode, b_mode, c_mode))
            }
            2 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                let c_mode = ((i % 100000) / 10000).try_into().map_err(InvalidParam3Mode)?;
                Ok(Multiply(a_mode, b_mode, c_mode))
            },
            3 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                Ok(Input(a_mode))
            },
            4 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                Ok(Output(a_mode))
            },
            5 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                Ok(JumpIfTrue(a_mode, b_mode))
            },
            6 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                Ok(JumpIfFalse(a_mode, b_mode))
            },
            7 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                let c_mode = ((i % 100000) / 10000).try_into().map_err(InvalidParam3Mode)?;
                Ok(LessThan(a_mode, b_mode, c_mode))
            },
            8 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                let b_mode = ((i % 10000) / 1000).try_into().map_err(InvalidParam2Mode)?;
                let c_mode = ((i % 100000) / 10000).try_into().map_err(InvalidParam3Mode)?;
                Ok(Equals(a_mode, b_mode, c_mode))
            },
            9 => {
                let a_mode = ((i % 1000) / 100).try_into().map_err(InvalidParam1Mode)?;
                Ok(AdjustRelBase(a_mode))
            }
            99 => {
                Ok(Halt)
            }
            invalid => Err(InvalidOpCode(invalid))
        }
    }
}
pub enum InstructionResult {
    Nothing,
    Output(isize),
    Halt,
}
impl IntCode {
    pub fn run(&mut self) {
        loop {
            use InstructionResult::*;
            match self.run_next_instruction() {
                Nothing => {},
                Output(i) => println!("{}", i),
                Halt => break
            }
        }
    }
    fn run_next_instruction(&mut self) -> InstructionResult {
        use OpCode::*;
        let opcode: OpCode = self.get_next().unwrap().try_into().unwrap();
        match opcode {
            Add(a_mode,b_mode,c_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b = self.get_input(b_mode, b).unwrap();
                let c = self.get_next().unwrap();
                let c = self.get_output(c_mode, c).unwrap();
                *c = a + b;
            }
            Multiply(a_mode, b_mode, c_mode)  => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b = self.get_input(b_mode, b).unwrap();
                let c = self.get_next().unwrap();
                let c = self.get_output(c_mode, c).unwrap();
                *c = a * b;
            },
            Input(a_mode) => {
                let input =  self.inputs.pop_front().unwrap();
                let a = self.get_next().unwrap();
                let a = self.get_output(a_mode, a).unwrap();
                *a = input;
                
            }
            Output(a_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                return InstructionResult::Output(a);
            },
            JumpIfTrue(a_mode, b_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b: usize = self.get_input(b_mode, b).unwrap().try_into().unwrap();
                if a != 0 {
                    self.ins_ptr = b;
                }
            },
            JumpIfFalse(a_mode, b_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b: usize = self.get_input(b_mode, b).unwrap().try_into().unwrap();
                if a == 0 {
                    self.ins_ptr = b;
                }

            },
            LessThan(a_mode, b_mode, c_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b = self.get_input(b_mode, b).unwrap();
                let c = self.get_next().unwrap();
                let c = self.get_output(c_mode, c).unwrap();
                if a < b {
                    *c = 1;
                }
                else {
                    *c = 0;
                }
            },
            Equals(a_mode, b_mode, c_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                let b = self.get_next().unwrap();
                let b = self.get_input(b_mode, b).unwrap();
                let c = self.get_next().unwrap();
                let c = self.get_output(c_mode, c).unwrap();
                if a == b {
                    *c = 1;
                }
                else {
                    *c = 0;
                }
            },
            AdjustRelBase(a_mode) => {
                let a = self.get_next().unwrap();
                let a = self.get_input(a_mode, a).unwrap();
                self.rel_offset += a;
            }
            Halt => return InstructionResult::Halt,
        }
        InstructionResult::Nothing
    }
    pub fn add_inputs(&mut self, i: impl Iterator<Item=isize>) {
        for i in i {
            self.inputs.push_back(i);
        }
    }
    fn get_next(&mut self ) -> Option<isize> {
        let result = self.memory.get(self.ins_ptr).map(|x| *x);
        self.ins_ptr += 1;
        result
    }
    fn get_input(&self, mode: InputParamMode, param: isize) -> Result<isize, ParamAccessError> {
        use InputParamMode::*;
        use ParamAccessError::*;
        match mode {
            Position => {
                let index: usize = param.try_into().map_err(InvalidAddress)?;
                self.memory.get(index).map(|x| *x).ok_or_else(|| OutOfBounds(index))
            },
            Immediate => Ok(param),
            Relative => {
                let index: usize = (param + self.rel_offset).try_into().map_err(InvalidAddress)?;
                self.memory.get(index).map(|x| *x).ok_or_else(|| OutOfBounds(index))
            },
        }
    }
    fn get_output(&mut self, mode: OutputParamMode, param: isize) -> Result<&mut isize, ParamAccessError> { 
        use OutputParamMode::*;
        use ParamAccessError::*;
        match mode {
            Position => {
                let index: usize = param.try_into().map_err(InvalidAddress)?;
                self.memory.get_mut(index).ok_or_else(|| OutOfBounds(index))
            },
            Relative => {
                let index: usize = (param + self.rel_offset).try_into().map_err(InvalidAddress)?;
                self.memory.get_mut(index).ok_or_else(|| OutOfBounds(index))
            },
        }
    }
}
impl Iterator for IntCode {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use InstructionResult::*;
            match self.run_next_instruction() {
                Nothing => {},
                Output(i) => return Some(i),
                Halt => return None,
            }
        }
    }
}
#[derive(Debug)]
enum ParamAccessError {
    InvalidAddress(<usize as TryFrom<isize>>::Error),
    OutOfBounds(usize)
}
