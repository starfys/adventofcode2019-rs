use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::io::{self, BufRead,};

fn walk_board<'a>(instructions: impl Iterator<Item = &'a str>) -> HashMap<(isize, isize), usize> {
    // Current coordinates
    let mut x: isize = 0;
    let mut y: isize = 0;
    // Current step
    let mut step: usize = 0;
    // Visited coordinates
    let mut visited = HashMap::new();

    // Iterate over instructions
    for instruction in instructions {
        // Parse the instruction
        let mut instruction = instruction.chars();
        let direction = instruction.next().unwrap();
        let magnitude: usize = instruction.collect::<String>().parse().unwrap();
        // Walk in the direction
        for _ in 0..magnitude*1000 {
            // Update once in desired direction
            match direction {
                'L' => x -= 1,
                'D' => y -= 1,
                'U' => y += 1,
                'R' => x += 1,
                _ => {}
            }
            step += 1;
            visited.entry((x, y)).or_insert(step);
        }
    }

    visited
}

struct HashMapIntersection<'a, T, U, V> {
    lhs: &'a HashMap<T, U>,
    rhs: hash_map::Iter<'a, T, V>,
}
impl<'a, T, U, V> HashMapIntersection<'a, T, U, V> {
    fn new(lhs: &'a HashMap<T, U>, rhs: &'a HashMap<T, V>) -> Self {
        HashMapIntersection {
            lhs,
            rhs: rhs.iter(),
        }
    }
}
impl<'a, T, U, V> Iterator for HashMapIntersection<'a, T, U, V>
where
    T: Eq + Hash,
{
    type Item = (&'a T, (&'a U, &'a V));
    fn next(&mut self) -> Option<Self::Item> {
        // Take off the right hand iterator
        while let Some((rk, rv)) = self.rhs.next() {
            if let Some(lv) = self.lhs.get(rk) {
                return Some((rk, (lv, rv)));
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let wire_1_visited = walk_board(lines[0].split(","));
    let wire_2_visited = walk_board(lines[1].split(","));
    
    // Get the intersection of points visited
    let both_visited: Vec<_> = HashMapIntersection::new(&wire_1_visited, &wire_2_visited).collect();
    
    let part_1_answer = both_visited
        .iter()
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("Part 1: {:?}", part_1_answer);
    
    let part_2_answer = both_visited
        .iter()
        .map(|(_, (s1, s2))| *s1 + *s2)
        .min()
        .unwrap();
    println!("Part 2: {:?}", part_2_answer);
    Ok(())
}
