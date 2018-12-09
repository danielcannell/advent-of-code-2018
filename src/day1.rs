// Day 1: Chronal Calibration

use std::collections::HashSet;

pub fn solve() {
    let lines: Vec<i32> = include_str!("data/day1.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[i32]) -> i32 {
    lines.iter().sum()
}

fn part2(lines: &[i32]) -> i32 {
    let mut freq = 0;

    let mut seen = HashSet::new();
    seen.insert(freq);

    for delta in lines.iter().cycle() {
        freq += delta;
        if !seen.insert(freq) {
            return freq;
        }
    }

    unreachable!();
}
