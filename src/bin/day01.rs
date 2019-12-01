use std::io::{self, BufRead};

fn fuel(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn fuel_r(mass: i64) -> i64 {
    if mass <= 0 {
        0
    } else {
        fuel(mass) + fuel_r(fuel(mass))
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

#[cfg(test)]
mod test {
    use crate::{fuel, fuel_r};

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
    #[test]
    fn test_fuel_r() {
        assert_eq!(fuel_r(14), 2);
        assert_eq!(fuel_r(1969), 966);
        assert_eq!(fuel_r(100756), 50346);
    }
}
