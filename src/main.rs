use clap::Parser;
use std::fs;
use std::process::exit;

mod common;
mod emitter;
mod modules;
mod vm;

use crate::modules::{ConsoleInputModule, ConsoleOutputModule};
use crate::vm::RS100;

/// An emulator for the TIS-100 assembly dialect
#[derive(Parser, Debug)]
struct Args {
    /// The file to execute
    #[arg(short, long)]
    file: String,

    /// If true, prints the outputs as numbers instead of characters
    #[arg(short, long, default_value_t = false)]
    num: bool,
}

fn main() {
    let args = Args::parse();

    let _data = match fs::read(args.file) {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let mut rs100 = RS100::new(
        _data,
        Box::new(ConsoleInputModule::new()),
        Box::new(ConsoleOutputModule::new(args.num)),
    );

    match rs100.execute() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
