use std::io::{Read, Stdin, Stdout, Write};

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_string(),
        }
    }

    fn new_from_string(msg: String) -> Error {
        Error { msg }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Num {
    value: i16,
}

impl Num {
    pub fn new() -> Num {
        Num { value: 0 }
    }

    pub fn from_i16(n: i16) -> Num {
        if n > 999 {
            Num { value: 999 }
        } else if n < -999 {
            Num { value: -999 }
        } else {
            Num { value: n }
        }
    }

    pub fn from_u8(a: u8, b: u8) -> Num {
        Num::from_i16(((a as i16) << 8) | b as i16)
    }

    pub fn add(&self, n: Num) -> Num {
        Num::from_i16(self.value + n.value)
    }

    pub fn sub(&self, n: Num) -> Num {
        Num::from_i16(self.value - n.value)
    }

    pub fn neg(&self) -> Num {
        Num::from_i16(-self.value)
    }

    pub fn eq16(&self, n: i16) -> bool {
        self.value == n
    }

    pub fn gt16(&self, n: i16) -> bool {
        self.value > n
    }

    pub fn lt16(&self, n: i16) -> bool {
        self.value < n
    }

    pub fn eq(&self, n: Num) -> bool {
        self.eq16(n.value)
    }

    pub fn gt(&self, n: Num) -> bool {
        self.gt16(n.value)
    }

    pub fn lt(&self, n: Num) -> bool {
        self.lt16(n.value)
    }

    pub fn real(&self) -> i16 {
        self.value
    }

    pub fn char(&self) -> (char, char) {
        (
            char::from_u32((self.value >> 8) as u32).unwrap_or_default(),
            char::from_u32((self.value & 0xff) as u32).unwrap_or_default(),
        )
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Clone for Num {
    fn clone(&self) -> Self {
        Num::from_i16(self.value)
    }
}

enum Dest {
    Last,
    Acc,
    Up,
    Down,
}

impl Dest {
    fn from_u8(n: u8) -> Dest {
        match n {
            0 => Dest::Last,
            1 => Dest::Acc,
            2 => Dest::Up,
            3 => Dest::Down,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dest::Last => "Last",
                Dest::Acc => "Acc",
                Dest::Up => "Up",
                Dest::Down => "Down",
            }
        )
    }
}

impl Clone for Dest {
    fn clone(&self) -> Self {
        match self {
            Dest::Last => Dest::Last,
            Dest::Acc => Dest::Acc,
            Dest::Up => Dest::Up,
            Dest::Down => Dest::Down,
        }
    }
}

const INSTRUCTION_SIZE: usize = 4;
/*

*/

pub struct RS100 {
    data: Vec<u8>,
    output: Stdout,
    input: Stdin,
    last_dst: Dest,
    pc: usize,
    acc: Num,
    bak: Num,
}

impl RS100 {
    pub fn new(data: Vec<u8>, output: Stdout, input: Stdin) -> RS100 {
        return RS100 {
            data,
            output,
            input,
            last_dst: Dest::Down,
            pc: 0,
            acc: Num::new(),
            bak: Num::new(),
        };
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        while self.pc < self.data.len() {
            match self.get_opcode() {
                0 => self.pc += INSTRUCTION_SIZE, // NOP
                1 => {
                    // MOV
                    let (a, b, _) = self.get_operands();

                    println!("{}, {}", a, b);

                    self.set(b, a)?;
                    self.pc += INSTRUCTION_SIZE;
                }
                2 => {
                    // SWP
                    println!("{}", self);
                    let temp = self.bak.clone();
                    self.bak = self.acc.clone();
                    self.acc = temp;
                    self.pc += INSTRUCTION_SIZE;
                }
                3 => {
                    // SAV
                    self.bak = self.acc.clone();
                    self.pc += INSTRUCTION_SIZE;
                }
                4 => {
                    // ADD
                    let (a, _, _) = self.get_operands();

                    self.set(Dest::Acc, self.acc.add(a))?;
                    self.pc += INSTRUCTION_SIZE;
                }
                5 => {
                    // SUB
                    let (a, _, _) = self.get_operands();

                    self.set(Dest::Acc, self.acc.sub(a))?;
                    self.pc += INSTRUCTION_SIZE;
                }
                6 => {
                    // NEG
                    self.acc = self.acc.neg();
                    self.pc += INSTRUCTION_SIZE;
                }
                7 => {
                    // JMP
                    let (_, _, a) = self.get_operands();
                    self.pc = a as usize;
                }
                8 => {
                    // JEZ
                    let (_, _, a) = self.get_operands();
                    if self.acc.eq16(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                9 => {
                    // JNZ
                    let (_, _, a) = self.get_operands();
                    if !self.acc.eq16(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                10 => {
                    // JGZ
                    let (_, _, a) = self.get_operands();
                    if self.acc.gt16(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                11 => {
                    // JLZ
                    let (_, _, a) = self.get_operands();
                    if self.acc.lt16(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                12 => {
                    // JRO
                    self.pc = self.acc.real() as usize;
                }
                _ => {
                    return Err(Error::new_from_string(format!(
                        "invalid opcode {}",
                        self.data[self.pc]
                    )))
                }
            }
        }

        Ok(())
    }

    fn get_opcode(&self) -> u8 {
        self.data[self.pc] & 0b1111
    }

    fn get_operands(&mut self) -> (Num, Dest, u32) {
        (
            if self.is_literal() {
                Num::from_u8(self.data[self.pc + 1], self.data[self.pc + 2])
            } else {
                self.get(Dest::from_u8(self.data[self.pc + 2]))
                    .expect("failed to obtain destination operand")
            },
            Dest::from_u8(self.data[self.pc + 3]),
            (u32::from(self.data[self.pc + 1]) << 24)
                | (u32::from(self.data[self.pc + 2]) << 8)
                | u32::from(self.data[self.pc + 3]),
        )
    }

    fn get_flags(&self) -> u8 {
        (self.data[self.pc] & 0b11110000) >> 4
    }

    fn is_literal(&self) -> bool {
        self.get_flags() & 0b1 == 1
    }

    fn get(&mut self, dst: Dest) -> Result<Num, Error> {
        match dst {
            Dest::Last => self.get(self.last_dst.clone()),
            Dest::Acc => Ok(self.acc.clone()),
            Dest::Up => {
                let mut s = String::new();

                match self.input.read_to_string(&mut s) {
                    Ok(_) => {
                        self.last_dst = dst;
                        Ok(Num::from_i16(s.as_bytes()[0].into()))
                    }
                    Err(e) => Err(Error::new_from_string(e.to_string())),
                }
            }
            Dest::Down => {
                self.last_dst = dst;
                Ok(Num::from_i16(0))
            }
        }
    }

    fn set(&mut self, dst: Dest, n: Num) -> Result<(), Error> {
        match dst {
            Dest::Last => self.set(self.last_dst.clone(), n),
            Dest::Acc => {
                self.acc = n;
                Ok(())
            }
            Dest::Up => {
                self.last_dst = dst;
                Ok(())
            }
            Dest::Down => {
                let (a, b) = n.char();
                match self.output.write(&[a as u8, b as u8]) {
                    Ok(_) => {
                        self.last_dst = dst;
                        Ok(())
                    }
                    Err(e) => return Err(Error::new_from_string(e.to_string())),
                }
            }
        }
    }
}

impl std::fmt::Display for RS100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}, {}{}", "{", self.acc, self.bak, "}")
    }
}
