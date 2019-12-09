use std::io::{self, BufRead};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let input: Vec<char> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()?
        .chars()
        .collect();

    // Split input into layers
    let layers: Vec<_> = input.chunks(WIDTH * HEIGHT).collect();

    // Get the layer with the least zeroes
    let min_layer = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|x| **x == '0').count())
        .unwrap();
    // Calculate part 1 answer
    let n1 = min_layer.iter().filter(|x| **x == '1').count();
    let n2 = min_layer.iter().filter(|x| **x == '2').count();
    println!("Part 1: {}", n1 * n2);

    let image = layers
        .iter()
        .rev()
        .fold(['2'; WIDTH * HEIGHT], |mut image, layer| {
            for (i_pix, l_pix) in image.iter_mut().zip(layer.iter()) {
                if *l_pix != '2' {
                    *i_pix = *l_pix
                }
            }
            image
        });
    println!("Part 2:");
    for row in 0..HEIGHT {
        println!(
            "{}",
            image[row * WIDTH..(row + 1) * WIDTH]
                .iter()
                .collect::<String>()
                .replace('0', " ")
                .replace('1', "█")
                .replace('2', " ")
        )
    }
    Ok(())
}
