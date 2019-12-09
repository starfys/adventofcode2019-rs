use adventofcode2019_rs::intcode::IntCode;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::iter;

const NUM_AMPS: usize = 5;

fn amplify(program: &IntCode, phases: &[&isize]) -> Option<isize> {
    // Create a set of amplifiers
    let mut amps: Vec<_> = iter::repeat(program.clone()).take(NUM_AMPS).collect();
    // Set the phase for each amp
    for (amp, phase) in amps.iter_mut().zip(phases) {
        let inputs = vec![**phase];
        amp.add_inputs(inputs.into_iter());
    }
    // Iterate until the amps no longer return a value
    let mut last_output = 0;
    loop {
        for amp in &mut amps {
            let inputs = vec![last_output];
            amp.add_inputs(inputs.into_iter());
            if let Some(output) = amp.next() {
                last_output = output;
            } else {
                return Some(last_output);
            }
        }
    }
}

fn best_permutation<'a>(program: &IntCode, phases: &'a [isize]) -> Option<(isize, Vec<&'a isize>)> {
    phases
        .iter()
        .permutations(5)
        .map(|permutation| {
            let output = amplify(program, permutation.as_slice()).unwrap();
            (output, permutation)
        })
        .max()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim_end();

    let program: IntCode = line.parse()?;
    let part_1 = best_permutation(&program, &[0, 1, 2, 3, 4]).unwrap();
    println!("Part 1: {}", part_1.0);
    let part_2 = best_permutation(&program, &[5, 6, 7, 8, 9]).unwrap();
    println!("Part 2: {}", part_2.0);

    Ok(())
}
