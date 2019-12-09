use adventofcode2019_rs::intcode::IntCode;
use std::convert::TryInto;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim_end();

    let program: IntCode = line.parse()?;

    const PART_1_NOUN: isize = 12;
    const PART_1_VERB: isize = 2;
    const PART_2_TARGET: isize = 19690720;

    // Copy the program
    let mut p1_program = program.clone();
    // Set the variables
    p1_program.memory[1] = PART_1_NOUN;
    p1_program.memory[2] = PART_1_VERB;

    p1_program.run();
    println!("Part 1: {}", p1_program.memory[0]);

    // Brute force solution to part 2
    'outer: for noun in 0isize..program.memory.len().try_into().unwrap() {
        for verb in 0isize..program.memory.len().try_into().unwrap() {
            // Clone the program
            let mut p2_program = program.clone();
            p2_program.memory[1] = noun;
            p2_program.memory[2] = verb;
            p2_program.run();
            if p2_program.memory[0] == PART_2_TARGET {
                let mut p2_program = program.clone();
                p2_program.memory[1] = noun;
                p2_program.memory[2] = verb;
                println!("Part 2: {}", noun * 100 + verb);
                break 'outer;
            }
        }
    }
    Ok(())
}
