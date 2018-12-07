use std::collections::HashSet;

pub fn solve() {
    let polymer = include_str!("data/day5.txt").trim().as_bytes();

    println!("Part 1: {}", part1(&polymer[..]));
    println!("Part 2: {}", part2(&polymer[..]));
}

fn part1(polymer: &[u8]) -> usize {
    react(polymer.to_owned()).len()
}

fn part2(polymer: &[u8]) -> usize {
    let units: HashSet<u8> = polymer.iter().map(u8::to_ascii_lowercase).collect();

    let mut lengths = Vec::new();

    for u in units {
        let mut reduced_polymer = polymer.to_owned();
        reduced_polymer.retain(|c| !c.eq_ignore_ascii_case(&u));
        lengths.push(react(reduced_polymer).len());
    }

    lengths.into_iter().min().unwrap()
}

fn react(polymer: Vec<u8>) -> Vec<u8> {
    let mut polymer = polymer;

    let mut i = 0;

    while i + 1 < polymer.len() {
        if units_react(polymer[i], polymer[i + 1]) {
            polymer.remove(i);
            polymer.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    polymer
}

fn units_react(a: u8, b: u8) -> bool {
    (a ^ b) == 32
}
