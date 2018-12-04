// Day 4: Repose Record

use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Timelike, Utc};
use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let mut lines: Vec<Record> = include_str!("data/day4.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    lines.sort_by_key(|record| record.datetime);
    let guards = parse_records(&lines);

    println!("Part 1: {}", part1(&guards));
    println!("Part 2: {}", part2(&guards));
}

fn part1(guards: &HashMap<u32, Guard>) -> u32 {
    let (id, guard) = guards.iter().max_by_key(|(_, g)| g.time_asleep()).unwrap();
    let (minute, _) = guard.sleepyest_minute();
    id * minute
}

fn part2(guards: &HashMap<u32, Guard>) -> u32 {
    let (id, (minute, _)) = guards
        .iter()
        .map(|(id, g)| (id, g.sleepyest_minute()))
        .max_by_key(|(_, (_, s))| *s)
        .unwrap();

    id * minute
}

fn parse_records(records: &Vec<Record>) -> HashMap<u32, Guard> {
    let mut guards = HashMap::new();
    let mut guard = &mut Guard::new();
    let mut start = 0;

    for record in records {
        match record.event {
            Event::BeginShift(id) => {
                guard = guards.entry(id).or_insert(Guard::new());
            }
            Event::FallAsleep => {
                start = record.datetime.minute();
            }
            Event::WakeUp => {
                guard.naps.push(Nap::new(start, record.datetime.minute()));
            }
        }
    }

    guards
}

struct Guard {
    naps: Vec<Nap>,
}

impl Guard {
    fn new() -> Guard {
        Guard { naps: Vec::new() }
    }

    fn time_asleep(&self) -> u32 {
        self.naps.iter().map(|n| n.end - n.start).sum()
    }

    fn sleepyest_minute(&self) -> (u32, u32) {
        let mut mins = [0u32; 60];

        for nap in &self.naps {
            for min in nap.start..nap.end {
                mins[min as usize] += 1;
            }
        }

        let (minute, &duration) = mins.iter().enumerate().max_by_key(|&(_, dur)| dur).unwrap();
        (minute as u32, duration)
    }
}

struct Nap {
    start: u32,
    end: u32,
}

impl Nap {
    fn new(start: u32, end: u32) -> Nap {
        Nap { start, end }
    }
}

enum Event {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

struct Record {
    datetime: DateTime<Utc>,
    event: Event,
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\[(.*?)\] (?:Guard #(\d+) begins shift|(falls asleep)|(wakes up))")
            .unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let datetime_str = caps
            .get(1)
            .ok_or_else(|| err_msg("Invalid format"))?
            .as_str();
        let datetime = Utc.datetime_from_str(datetime_str, "%Y-%m-%d %H:%M")?;

        let event = {
            if let Some(group) = caps.get(2) {
                Event::BeginShift(group.as_str().parse()?)
            } else if let Some(_) = caps.get(3) {
                Event::FallAsleep
            } else if let Some(_) = caps.get(4) {
                Event::WakeUp
            } else {
                return Err(err_msg("Invalid format"));
            }
        };

        Ok(Record { datetime, event })
    }
}
