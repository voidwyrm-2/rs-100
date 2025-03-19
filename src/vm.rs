use crate::common::*;
use crate::modules::Module;

pub struct RS100 {
    data: Vec<u8>,
    up: Box<dyn Module>,
    down: Box<dyn Module>,
    last_dst: Dest,
    pc: usize,
    acc: Num,
    bak: Num,
}

impl RS100 {
    pub fn new(data: Vec<u8>, up: Box<dyn Module>, down: Box<dyn Module>) -> RS100 {
        return RS100 {
            data,
            up,
            down,
            last_dst: Dest::Down,
            pc: 0,
            acc: Num::new(),
            bak: Num::new(),
        };
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        while self.pc < self.data.len() {
            match self.get_opcode()? {
                Opcode::Nop => self.pc += INSTRUCTION_SIZE,
                Opcode::Mov => {
                    let (a, b, _) = self.get_operands();

                    self.set(b, a)?;
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Swp => {
                    let temp = self.bak.clone();
                    self.bak = self.acc.clone();
                    self.acc = temp;
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Sav => {
                    self.bak = self.acc.clone();
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Add => {
                    let (a, _, _) = self.get_operands();

                    self.set(Dest::Acc, self.acc.clone() + a)?;
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Sub => {
                    let (a, _, _) = self.get_operands();

                    self.set(Dest::Acc, self.acc.clone() - a)?;
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Neg => {
                    self.acc = self.acc.neg();
                    self.pc += INSTRUCTION_SIZE;
                }
                Opcode::Jmp => {
                    let (_, _, a) = self.get_operands();
                    self.pc = a as usize;
                }
                Opcode::Jez => {
                    let (_, _, a) = self.get_operands();
                    if self.acc == Num::from(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                Opcode::Jnz => {
                    let (_, _, a) = self.get_operands();
                    if self.acc != Num::from(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                Opcode::Jgz => {
                    let (_, _, a) = self.get_operands();
                    if self.acc > Num::from(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                Opcode::Jlz => {
                    let (_, _, a) = self.get_operands();
                    if self.acc < Num::from(0) {
                        self.pc = a as usize;
                    } else {
                        self.pc += INSTRUCTION_SIZE;
                    }
                }
                Opcode::Jro => {
                    self.pc = self.acc.clone().into();
                }
            }
        }

        Ok(())
    }

    fn get_opcode(&self) -> Result<Opcode, Error> {
        Opcode::try_from(self.data[self.pc] & 0b1111)
    }

    fn get_operands(&mut self) -> (Num, Dest, u32) {
        (
            if self.is_literal() {
                Num::from_u8(self.data[self.pc + 1], self.data[self.pc + 2])
            } else {
                self.get(Dest::try_from(self.data[self.pc + 2]).unwrap_or_default())
                    .expect("failed to obtain destination operand")
            },
            Dest::try_from(self.data[self.pc + 3]).unwrap_or_default(),
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
                self.last_dst = dst;
                self.up.read()
            }
            Dest::Down => {
                self.last_dst = dst;
                self.up.read()
            }
            Dest::Left | Dest::Right => {
                self.last_dst = dst;
                Ok(Num::from(0))
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
                self.up.write(n);
                Ok(())
            }
            Dest::Down => {
                self.last_dst = dst;
                self.down.write(n);
                Ok(())
            }
            Dest::Left | Dest::Right => {
                self.last_dst = dst;
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for RS100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}, {}{}", "{", self.acc, self.bak, "}")
    }
}
