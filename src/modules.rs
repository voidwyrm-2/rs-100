use std::convert::{Into, TryFrom};
use std::io::Write;

use crate::common::{Error, Num};

pub fn get_module(m: String, num: bool) -> Result<Box<dyn Module>, Error> {
    let s = m.to_lowercase();
    match s.as_str() {
        "none" => Ok(Box::new(NoneModule::new())),
        "stdin" => Ok(Box::new(ConsoleInputModule::new())),
        "stdout" => Ok(Box::new(ConsoleOutputModule::new(num))),
        "stack-" => Ok(Box::new(StackModule::new())),
        _ => Err(format!("'{}' is not a valid module", s)),
    }
}

pub trait Module {
    fn read(&mut self) -> Result<Num, Error>;
    fn write(&mut self, n: Num) -> Option<Error>;
}

pub struct NoneModule {}

impl NoneModule {
    pub fn new() -> NoneModule {
        NoneModule {}
    }
}

impl Module for NoneModule {
    fn read(&mut self) -> Result<Num, Error> {
        Ok(Num::from(0))
    }

    fn write(&mut self, _n: Num) -> Option<Error> {
        None
    }
}

pub struct ConsoleInputModule {}

impl ConsoleInputModule {
    pub fn new() -> ConsoleInputModule {
        ConsoleInputModule {}
    }
}

impl Module for ConsoleInputModule {
    fn read(&mut self) -> Result<Num, Error> {
        let mut s = String::new();
        match std::io::stdin().read_line(&mut s) {
            Ok(_) => Ok(Num::from_u8(0, s.as_bytes()[0])),
            Err(e) => Err(e.to_string()),
        }
    }

    fn write(&mut self, _n: Num) -> Option<Error> {
        None
    }
}

pub struct ConsoleOutputModule {
    print_as_number: bool,
}

impl ConsoleOutputModule {
    pub fn new(print_as_number: bool) -> ConsoleOutputModule {
        ConsoleOutputModule { print_as_number }
    }
}

impl Module for ConsoleOutputModule {
    fn read(&mut self) -> Result<Num, Error> {
        Ok(Num::from(0))
    }

    fn write(&mut self, n: Num) -> Option<Error> {
        let (a, b) = n.to_u8();
        let n_bytes = &[a, b];

        let s = format!("{}", n);
        let s_bytes = s.as_bytes();

        match std::io::stdout().write(if self.print_as_number {
            s_bytes
        } else {
            n_bytes
        }) {
            Ok(_) => None,
            Err(e) => Some(e.to_string()),
        }
    }
}

struct StackModule {
    stack: Vec<Num>,
}

impl StackModule {
    pub fn new() -> StackModule {
        StackModule { stack: Vec::new() }
    }
}

impl Module for StackModule {
    fn read(&mut self) -> Result<Num, Error> {
        if self.stack.len() == 0 {
            Err("stack underflow".to_string())
        } else {
            Ok(self.stack.pop().unwrap())
        }
    }
    fn write(&mut self, n: Num) -> Option<Error> {
        self.stack.push(n);
        None
    }
}
