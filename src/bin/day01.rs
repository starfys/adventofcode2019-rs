use std::io::{self, BufRead};

fn fuel(mass: i64) -> i64 {
    return (mass / 3 - 2).max(0);
}

fn fuel_r(mass: i64) -> i64 {
    if mass <= 0 {
        return 0;
    } else {
        return fuel(mass) + fuel_r(fuel(mass));
    }
}

fn main() {
    let stdin = io::stdin();
    let (part_1, part_2) = stdin
        .lock()
        .lines()
        .flatten()
        .map(|l| l.parse())
        .flatten()
        .map(|mass| (fuel(mass), fuel_r(mass)))
        .fold((0, 0), |(af, af_r), (f, f_r)| (af + f, af_r + f_r));

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
