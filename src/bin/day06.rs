use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn num_orbiting(orbit_map: &HashMap<String, String>, name: &str) -> usize {
    orbit_map
        .get(name)
        .map(|planet| 1 + num_orbiting(&orbit_map, planet))
        .unwrap_or(0)
}
fn bfs_distance(orbit_map: &HashMap<String, String>, source: &str, dest: &str) -> usize {
    let mut connected_map: HashMap<String, Vec<String>> = orbit_map
        .iter()
        .map(|(k, v)| (k.to_string(), vec![v.to_string()]))
        .collect();
    for (k, v) in orbit_map {
        connected_map
            .entry(v.to_string())
            .and_modify(|c| c.push(k.to_string()))
            .or_insert(vec![k.to_string()]);
    }

    let mut unvisited = VecDeque::new();
    let mut discovered: HashSet<String> = HashSet::new();
    let mut distance: HashMap<&str, usize> = HashMap::new();
    discovered.insert(source.to_string());
    unvisited.push_back(source);
    distance.insert(source, 0);
    while let Some(cur_node) = unvisited.pop_front() {
        let cur_distance = distance.get(cur_node).unwrap().clone();
        if cur_node == dest {
            return cur_distance;
        }
        if let Some(neighbors) = connected_map.get(cur_node) {
            for neighbor in neighbors {
                if !discovered.contains(neighbor) {
                    discovered.insert(neighbor.to_string());
                    unvisited.push_back(neighbor);
                    distance.insert(neighbor, cur_distance + 1);
                }
            }
        }
    }
    return 0;
}

fn main() {
    let stdin = io::stdin();
    let orbit_map: HashMap<String, String> = stdin
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let mut line = line.split(')').take(2);
            let sun = line.next().unwrap().to_string();
            let earth = line.next().unwrap().to_string();
            (earth, sun)
        })
        .collect();
    println!(
        "Part 1: {}",
        orbit_map
            .keys()
            .map(|planet| num_orbiting(&orbit_map, planet))
            .sum::<usize>()
    );
    //println!("Part 2: {}", bfs_distance(&orbit_map, "YOU", "SAN") - 2)
    println!("Part 2: {}", bfs_distance(&orbit_map, "YOU", "SAN") - 2)
}
