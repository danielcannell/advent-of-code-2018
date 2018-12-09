use std::collections::HashMap;
use std::str::FromStr;

use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let lines: Vec<Point> = include_str!("data/day6.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &[Point]) -> i32 {
    let x_span = (
        lines.iter().map(|p| p.x).min().unwrap(),
        lines.iter().map(|p| p.x).max().unwrap(),
    );
    let y_span = (
        lines.iter().map(|p| p.y).min().unwrap(),
        lines.iter().map(|p| p.y).max().unwrap(),
    );

    let mut grid = Part1Grid::new(
        x_span.0 as usize,
        y_span.0 as usize,
        (x_span.1 - x_span.0) as usize,
        (y_span.1 - y_span.0) as usize,
    );

    for (id, point) in lines.iter().enumerate() {
        grid.add(id, point);
    }

    let areas = grid.areas();
    *areas.values().max().unwrap()
}

fn part2(lines: &[Point]) -> i32 {
    let x_span = (
        lines.iter().map(|p| p.x).min().unwrap(),
        lines.iter().map(|p| p.x).max().unwrap(),
    );
    let y_span = (
        lines.iter().map(|p| p.y).min().unwrap(),
        lines.iter().map(|p| p.y).max().unwrap(),
    );

    let mut grid = Part2Grid::new(
        x_span.0 as usize,
        y_span.0 as usize,
        (x_span.1 - x_span.0) as usize,
        (y_span.1 - y_span.0) as usize,
    );

    for point in lines {
        grid.add(point);
    }

    grid.area(10000)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Part1Grid {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    dists: Box<[i32]>,
    ids: Box<[usize]>,
}

impl Part1Grid {
    fn new(left: usize, top: usize, width: usize, height: usize) -> Part1Grid {
        Part1Grid {
            left,
            top,
            width,
            height,
            dists: vec![std::i32::MAX; width * height].into_boxed_slice(),
            ids: vec![std::usize::MAX; width * height].into_boxed_slice(),
        }
    }

    fn add(&mut self, id: usize, point: &Point) {
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                let dist = (point.x - x as i32).abs() + (point.y - y as i32).abs();
                let idx = (x - self.left) + (y - self.top) * self.width;

                if self.dists[idx] > dist {
                    self.dists[idx] = dist;
                    self.ids[idx] = id;
                } else if self.dists[idx] == dist {
                    self.ids[idx] = std::usize::MAX;
                }
            }
        }
    }

    fn areas(&self) -> HashMap<usize, i32> {
        let mut result = HashMap::new();

        for i in 0..(self.width * self.height) {
            *result.entry(self.ids[i]).or_insert(0) += 1;
        }

        for i in 0..self.width {
            result.remove(&self.ids[i]);
            result.remove(&self.ids[i + (self.height - 1) * self.width]);
        }
        for i in 0..self.height {
            result.remove(&self.ids[i * self.width]);
            result.remove(&self.ids[i * self.width + (self.width - 1)]);
        }

        result.remove(&std::usize::MAX);
        result
    }
}

struct Part2Grid {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    dists: Box<[i32]>,
}

impl Part2Grid {
    fn new(left: usize, top: usize, width: usize, height: usize) -> Part2Grid {
        Part2Grid {
            left,
            top,
            width,
            height,
            dists: vec![0; width * height].into_boxed_slice(),
        }
    }

    fn add(&mut self, point: &Point) {
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                let dist = (point.x - x as i32).abs() + (point.y - y as i32).abs();
                let idx = (x - self.left) + (y - self.top) * self.width;
                self.dists[idx] += dist;
            }
        }
    }

    fn area(&self, max_dist: i32) -> i32 {
        self.dists.iter().filter(|d| **d < max_dist).count() as i32
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+), (\d+)").unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let get = |idx| -> Result<i32, Self::Err> {
            Ok(caps
                .get(idx)
                .ok_or_else(|| err_msg("Invalid format"))?
                .as_str()
                .parse()?)
        };

        Ok(Point {
            x: get(1)?,
            y: get(2)?,
        })
    }
}
