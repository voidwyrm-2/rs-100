use std::io::Write;

use crate::common::{Error, Num};

pub trait Module {
    fn read(&mut self) -> Result<Num, Error>;
    fn write(&mut self, n: Num) -> Option<Error>;
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
