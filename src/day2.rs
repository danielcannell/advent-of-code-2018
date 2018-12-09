// Day 2: Inventory Management System

pub fn solve() {
    let lines: Vec<&str> = include_str!("data/day2.txt").lines().collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u32 {
    let mut dup2 = 0u32;
    let mut dup3 = 0u32;

    for id in lines {
        let mut counts = [0u32; 256];
        for letter in id.bytes() {
            counts[letter as usize] += 1;
        }

        if counts.contains(&2) {
            dup2 += 1;
        }
        if counts.contains(&3) {
            dup3 += 1;
        }
    }

    dup2 * dup3
}

fn part2(lines: &[&str]) -> String {
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let (id1, id2) = (lines[i], lines[j]);

            let matching: String = id1
                .chars()
                .zip(id2.chars())
                .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                .collect();

            if matching.len() + 1 == id1.len() {
                return matching;
            }
        }
    }

    String::new()
}
