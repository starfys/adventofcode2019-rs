use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::io::{self, BufRead, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let line =  stdin.lock().lines().next().unwrap().unwrap();
    let mut line = line.split('-');
    let lbound: usize = line.next().unwrap().parse().unwrap();
    let rbound: usize = line.next().unwrap().parse().unwrap();
    let mut p1_count = 0;
    let mut p2_count = 0;
    for num in lbound..=rbound {
        let num = num.to_string(); 
        let mut last_digit: char = '/';
        let mut streak: usize = 0;

        let mut has_pair: bool = false;
        let mut has_lone_pair: bool = false;
        
        let mut ascends = true;
        for digit in num.chars() {
            if digit < last_digit {
                ascends = false;
                break
            }
            else if digit == last_digit {
                has_pair = true;
                streak += 1;
            }
            else {
                if streak == 2 {
                    has_lone_pair = true;
                }
                streak = 1;
            }
            last_digit = digit;
        }
        if ascends && has_pair {
            p1_count += 1;
        }
        if ascends && (has_lone_pair || streak == 2) {
            p2_count += 1;
        }
    }

    println!("Part 1: {}", p1_count);
    println!("Part 2: {}", p2_count);
    Ok(())
}
