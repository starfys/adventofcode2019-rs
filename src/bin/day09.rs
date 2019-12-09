use std::io::{self, BufRead};
use adventofcode2019_rs::intcode::IntCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim_end();

    let mut p1_program: IntCode = line.parse()?;
    let mut p2_program = p1_program.clone();
    
    let inputs = vec![1];
    p1_program.add_inputs(inputs.into_iter());
    println!("Part 1: {}", p1_program.last().unwrap());
    
    let inputs = vec![2];
    p2_program.add_inputs(inputs.into_iter());
    println!("Part 2: {}", p2_program.last().unwrap());

    Ok(())
}
