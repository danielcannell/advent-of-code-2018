use std::str::FromStr;

use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let lines: Vec<Instruction> = include_str!("data/day7.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[Instruction]) -> String {
    let mut sleigh = Sleigh::new(lines);

    while let Some(step) = sleigh.next_step() {
        sleigh.complete(step);
    }

    sleigh.sequence()
}

fn part2(lines: &[Instruction]) -> u32 {
    let mut sleigh = Sleigh::new(lines);
    let mut helpers = Helpers::new(5);
    let mut time = 0;

    while !sleigh.done() || !helpers.done() {
        while let Some(elf) = helpers.get_idle() {
            if let Some(step) = sleigh.next_step() {
                let duration = step as u32 - 'A' as u32 + 61;
                elf.give(step, duration);
            } else {
                break;
            }
        }

        for c in helpers.work() {
            sleigh.complete(c);
        }

        time += 1;
    }

    time
}

#[derive(Debug, Clone)]
enum Elf {
    Idle,
    Busy(char, u32),
}

impl Elf {
    fn new() -> Elf {
        Elf::Idle
    }

    fn work(&mut self) -> Option<char> {
        if let &mut Elf::Busy(c, t) = self {
            if t == 1 {
                *self = Elf::Idle;
                return Some(c);
            } else {
                *self = Elf::Busy(c, t - 1);
            }
        }

        None
    }

    fn give(&mut self, step: char, t: u32) {
        *self = Elf::Busy(step, t)
    }

    fn is_idle(&self) -> bool {
        if let Elf::Idle = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Helpers {
    elves: Vec<Elf>,
}

impl Helpers {
    fn new(count: usize) -> Helpers {
        Helpers {
            elves: vec![Elf::new(); count],
        }
    }

    fn get_idle(&mut self) -> Option<&mut Elf> {
        for elf in &mut self.elves {
            if elf.is_idle() {
                return Some(elf);
            }
        }

        None
    }

    fn work(&mut self) -> Vec<char> {
        self.elves.iter_mut().filter_map(Elf::work).collect()
    }

    fn done(&self) -> bool {
        self.elves.iter().all(Elf::is_idle)
    }
}

struct Sleigh {
    instructions: Vec<Instruction>,
    remaining: Vec<char>,
    done: Vec<char>,
}

impl Sleigh {
    fn new(instructions: &[Instruction]) -> Sleigh {
        Sleigh {
            instructions: instructions.to_vec(),
            remaining: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            done: Vec::new(),
        }
    }

    fn next_step(&mut self) -> Option<char> {
        'outer: for i in 0..self.remaining.len() {
            let step = self.remaining[i];

            for instr in &self.instructions {
                if instr.1 == step && !self.done.contains(&instr.0) {
                    continue 'outer;
                }
            }

            self.remaining.remove(i);
            return Some(step);
        }

        None
    }

    fn complete(&mut self, step: char) {
        self.done.push(step)
    }

    fn sequence(&self) -> String {
        self.done.iter().collect()
    }

    fn done(&self) -> bool {
        self.remaining.is_empty()
    }
}

#[derive(Debug, Clone)]
struct Instruction(char, char);

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let get = |idx| -> Result<char, Self::Err> {
            Ok(caps
                .get(idx)
                .ok_or_else(|| err_msg("Invalid format"))?
                .as_str()
                .parse()?)
        };

        Ok(Instruction(get(1)?, get(2)?))
    }
}
