use std::env;
use std::fs::File;
use std::io::prelude::*;

use day2b::*;

fn main() -> std::io::Result<()> {
    let input_file = if let Some(arg1) = env::args().nth(1) {
        arg1
    } else {
        println!("cargo <:input_file>");
        return Ok(());
    };
    let mut input = File::open(input_file).unwrap();
    let mut instructions = String::new();
    input
        .read_to_string(&mut instructions)
        .ok()
        .expect("read error");
    let opcodes: Vec<i64> = instructions
        .split(",")
        .map(|s| s.trim().parse())
        .filter_map(Result::ok)
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut modified_opcodes = opcodes.clone();
            modified_opcodes[1] = noun;
            modified_opcodes[2] = verb;
            let machine = create_machine();
            let result = machine.run(&modified_opcodes);
            if result == 19690720 {
                println!("found: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }
    Ok(())
}
