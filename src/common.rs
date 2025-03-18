use std::{
    cmp::Ordering,
    convert::{From, Into, TryFrom},
    ops::{Add, Sub},
};

pub const INSTRUCTION_SIZE: usize = 4;
// instruction format
/*
00 00 00 00
|  |  |  |
|  |  |  +---- operand B
|  +--+------- operand A
+------------- flags and opcode, see below
*/
/*
0 0 0 0 0 0 0 0
| | | | | | | |
| | | | +-+-+-+--- opcode
| | | +----------- first operand is literal or not
| | +------------- reserved
| +--------------- reserved
+----------------- reserved
*/

pub type Error = String;

#[derive(Clone, Eq, Ord)]
pub struct Num {
    value: i16,
}

impl Num {
    pub fn new() -> Num {
        Num { value: 0 }
    }

    pub fn from_u8(a: u8, b: u8) -> Num {
        Num::from(((a as i16) << 8) | b as i16)
    }

    pub fn neg(&self) -> Num {
        Num::from(-self.value)
    }

    pub fn to_u8(&self) -> (u8, u8) {
        ((self.value >> 8) as u8, (self.value & 0xff) as u8)
    }
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        Num::from(self.value + rhs.value)
    }
}

impl Sub for Num {
    type Output = Num;

    fn sub(self, rhs: Self) -> Self::Output {
        Num::from(self.value - rhs.value)
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value > other.value {
            Some(Ordering::Greater)
        } else if self.value < other.value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl From<i16> for Num {
    fn from(value: i16) -> Self {
        if value > 999 {
            Num { value: 999 }
        } else if value < -999 {
            Num { value: -999 }
        } else {
            Num { value }
        }
    }
}

impl Into<i16> for Num {
    fn into(self) -> i16 {
        self.value
    }
}

impl Into<usize> for Num {
    fn into(self) -> usize {
        self.value as usize
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub enum Opcode {
    Nop,
    Mov,
    Swp,
    Sav,
    Add,
    Sub,
    Neg,
    Jmp,
    Jez,
    Jnz,
    Jgz,
    Jlz,
    Jro,
}

impl TryFrom<u8> for Opcode {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Nop),
            1 => Ok(Opcode::Mov),
            2 => Ok(Opcode::Swp),
            3 => Ok(Opcode::Sav),
            4 => Ok(Opcode::Add),
            5 => Ok(Opcode::Sub),
            6 => Ok(Opcode::Neg),
            7 => Ok(Opcode::Jmp),
            8 => Ok(Opcode::Jez),
            9 => Ok(Opcode::Jnz),
            10 => Ok(Opcode::Jgz),
            11 => Ok(Opcode::Jlz),
            12 => Ok(Opcode::Jro),
            _ => Err(format!("invalid opcode {}", value)),
        }
    }
}

#[derive(Clone)]
pub enum Dest {
    Last,
    Acc,
    Up,
    Down,
}

impl Default for Dest {
    fn default() -> Self {
        return Dest::Last;
    }
}

impl TryFrom<u8> for Dest {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Dest::Last),
            1 => Ok(Dest::Acc),
            2 => Ok(Dest::Up),
            3 => Ok(Dest::Down),
            _ => Err(format!("{} is not a valid Dest value", value)),
        }
    }
}

impl Into<u8> for Dest {
    fn into(self) -> u8 {
        match self {
            Dest::Last => 0,
            Dest::Acc => 1,
            Dest::Up => 2,
            Dest::Down => 3,
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
