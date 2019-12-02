use std::io::{self, BufRead};

fn run(noun: usize, verb: usize, prog: &[usize]) -> usize {
    let mut program = prog.to_vec();
    let mut pos = 0;
    program[1] = noun;
    program[2] = verb;
    loop {
        match program[pos] {
            1 => {
                let a = program[pos+1];
                let b = program[pos+2];
                let c = program[pos+3];
                program[c] = program[a] + program[b];
                pos += 4;
            }
            2 => {
                let a = program[pos+1];
                let b = program[pos+2];
                let c = program[pos+3];
                program[c] = program[a] * program[b];
                pos += 4;
            }
            99 => {return program[0]}
            _ => {}
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    
    let program: Vec<usize> = line
        .split(',')
        .map(|i| i.parse())
        .flatten()
        .collect();
    
    const PART_1_NOUN: usize = 12;
    const PART_1_VERB: usize = 2;
    const PART_2_TARGET: usize = 19690720;


    println!("Part 1: {}", run(PART_1_NOUN, PART_1_VERB, &program));
    for v1 in 0..program.len(){
        for v2 in 0..program.len() {
            if run(v1, v2, &program) == PART_2_TARGET {
                println!("Part 2: {}", v1 * 100 + v2);
            }
        }
    }
    Ok(())
}
