use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let lines: Vec<Light> = include_str!("data/day10.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sky = Sky::new(&lines);

    let mut min_t = 0;
    let mut min_sky = sky.clone();
    let mut min_cost = min_sky.cost();

    for t in 1.. {
        sky.step();

        let cost = sky.cost();
        if cost < min_cost {
            min_t = t;
            min_sky = sky.clone();
            min_cost = cost;
        } else {
            break;
        }
    }

    println!("Part 1:");
    min_sky.print();

    println!("Part 2: {}", min_t);
}

#[derive(Clone)]
struct Sky {
    lights: Vec<Light>,
}

impl Sky {
    fn new(lights: &[Light]) -> Sky {
        Sky {
            lights: lights.to_owned().to_vec(),
        }
    }

    fn step(&mut self) {
        for light in &mut self.lights {
            light.p += light.v;
        }
    }

    // I guess all the points will be close together at the right time?
    fn cost(&self) -> i32 {
        let (_, shape) = self.bounding_box();
        shape.x + shape.y
    }

    fn bounding_box(&self) -> (Vec2, Vec2) {
        let minx = self.lights.iter().map(|l| l.p.x).min().unwrap();
        let maxx = self.lights.iter().map(|l| l.p.x).max().unwrap();
        let miny = self.lights.iter().map(|l| l.p.y).min().unwrap();
        let maxy = self.lights.iter().map(|l| l.p.y).max().unwrap();

        (Vec2::new(minx, miny), Vec2::new(maxx - minx, maxy - miny))
    }

    fn print(&self) {
        let (corner, shape) = self.bounding_box();

        let mut pixels = vec![vec![' '; shape.x as usize + 1]; shape.y as usize + 1];

        for l in &self.lights {
            let p = l.p - corner;
            pixels[p.y as usize][p.x as usize] = '+';
        }

        for row in pixels {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Light {
    p: Vec2,
    v: Vec2,
}

impl Light {
    fn new(p: Vec2, v: Vec2) -> Light {
        Light { p, v }
    }
}

impl FromStr for Light {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
            .unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let get = |idx| -> Result<i32, Self::Err> {
            Ok(caps
                .get(idx)
                .ok_or_else(|| err_msg("Invalid format"))?
                .as_str()
                .parse()?)
        };

        Ok(Light::new(
            Vec2::new(get(1)?, get(2)?),
            Vec2::new(get(3)?, get(4)?),
        ))
    }
}
