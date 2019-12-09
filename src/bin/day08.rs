use std::collections::{hash_map, HashMap, HashSet};
use std::hash::Hash;
use std::io::{self, BufRead, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines =  stdin.lock().lines().next().unwrap().unwrap();
    let mut layers: Vec<[[u32; 25]; 6]> = Vec::new();
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let mut cur_layer = [[0; WIDTH]; HEIGHT];
    let mut cur_row = 0;
    let mut cur_col = 0;
    for c in lines.chars() {
        let c: u32 = c.to_string().parse().unwrap();
        cur_layer[cur_row][cur_col] = c;
        cur_col += 1;
        if cur_col == WIDTH {
            cur_col = 0;
            cur_row += 1;
            if cur_row == HEIGHT {
                layers.push(cur_layer);
                cur_layer = [[0; WIDTH]; HEIGHT];
                cur_row = 0;
            }
        }
    }
    let min_layer = layers.iter().min_by_key(|layer| layer.iter().map(|row| row.iter().filter(|x| **x == 0).count()).sum::<usize>()).unwrap();
    let n1 = min_layer.iter().map(|row| row.iter().filter(|x| **x == 1).count()).sum::<usize>();
    let n2 = min_layer.iter().map(|row| row.iter().filter(|x| **x == 2).count()).sum::<usize>();
    println!("{}", n1*n2); 
    
    let mut bottom = layers[99];

    for layer in layers[0..99].iter().rev() {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if layer[row][col] != 2 {
                    bottom[row][col] = layer[row][col]
                }
            }
        }
    }
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            print!("{}", if bottom[row][col] == 0 {' '} else { 'O'});
        }
        println!();
    }
    Ok(())
}
