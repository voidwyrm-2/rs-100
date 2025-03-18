use std::env;
use std::fs;
use std::process::exit;

mod common;
mod emitter;
mod vm;

use crate::vm::RS100;

fn main() {
    let mut args: Vec<String> = env::args().collect();

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

    args.remove(0);
    args.remove(0);

    let mut rs100 = RS100::new(_data, std::io::stdout(), std::io::stdin());

    rs100.set_print_as_number(
        args.contains(&"-n".to_string()) || args.contains(&"--num".to_string()),
    );

    match rs100.execute() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
