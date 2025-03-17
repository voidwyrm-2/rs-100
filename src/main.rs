use std::env;
use std::fs;
use std::process::exit;

use crate::vm::RS100;

mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("expected 'rs100 <file>'");
        exit(1);
    }

    let _data = match fs::read(args[1].clone()) {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    let mut rs100 = RS100::new(_data, std::io::stdout(), std::io::stdin());

    match rs100.execute() {
        Ok(_) => {
            println!("{}", rs100);
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
