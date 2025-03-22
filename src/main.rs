use clap::Parser;
use std::fs;
use std::process::exit;

mod common;
mod compiler;
mod modules;
mod vm;

use crate::compiler::lexer::Lexer;
use crate::vm::RS100;

const VERSION: &str = "1.4";

/// An emulator for the TIS-100 assembly dialect
#[derive(Parser, Debug)]
struct Args {
    /// The file to execute
    #[arg(short, long)]
    file: String,

    /// If true, prints the outputs as numbers instead of characters
    #[arg(short, long, default_value_t = false)]
    num: bool,

    /// If true, prints the current version and exits
    #[arg(short, long, default_value_t = false)]
    version: bool,

    /// The module for the `up` direction
    #[arg(short, long, default_value_t = ("stdin").to_string())]
    up: String,

    /// The module for the `down` direction
    #[arg(short, long, default_value_t = ("stdout").to_string())]
    down: String,

    /// The module for the `left` direction
    #[arg(short, long, default_value_t = ("none").to_string())]
    left: String,

    /// The module for the `right` direction
    #[arg(short, long, default_value_t = ("none").to_string())]
    right: String,
}

fn proxy_main() -> Result<(), crate::common::Error> {
    let args = Args::parse();

    if args.version {
        println!("RS-100 version {}", VERSION);
        return Ok(());
    }

    let data: Vec<u8> = if args.file.ends_with(".tis") {
        let text = match fs::read_to_string(args.file) {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };

        let mut lexer = Lexer::new(text);

        let _tokens = lexer.lex()?;

        Vec::new()
    } else {
        match fs::read(args.file) {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        }
    };

    let mut rs100 = RS100::new(
        data,
        modules::get_module(args.up, args.num)?,
        modules::get_module(args.down, args.num)?,
        modules::get_module(args.left, args.num)?,
        modules::get_module(args.right, args.num)?,
    );

    rs100.execute()
}

fn main() {
    match proxy_main() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
