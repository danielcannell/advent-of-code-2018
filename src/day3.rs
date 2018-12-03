use std::ops::{Index, IndexMut};
use std::str::FromStr;

use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let lines: Vec<Claim> = include_str!("data/day3.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<Claim>) -> u32 {
    let width = lines.iter().map(|c| c.left + c.width).max().unwrap();
    let height = lines.iter().map(|c| c.top + c.height).max().unwrap();
    let mut cloth = Cloth::new(width as usize, height as usize);

    for claim in lines {
        cloth.raster(claim);
    }

    cloth.contested()
}

fn part2(lines: &Vec<Claim>) -> u32 {
    let width = lines.iter().map(|c| c.left + c.width).max().unwrap();
    let height = lines.iter().map(|c| c.top + c.height).max().unwrap();
    let mut cloth = Cloth::new(width as usize, height as usize);

    for claim in lines {
        cloth.raster(claim);
    }

    for claim in lines {
        if cloth.check(claim) {
            return claim.id;
        }
    }

    0
}

struct Cloth {
    stride: usize,
    squares: Box<[u8]>,
}

impl Cloth {
    fn new(width: usize, height: usize) -> Cloth {
        Cloth {
            stride: width,
            squares: vec![0; width * height].into_boxed_slice(),
        }
    }

    fn inc(&mut self, x: usize, y: usize) {
        self[(x, y)] = self[(x, y)].saturating_add(1);
    }

    fn contested(&self) -> u32 {
        self.squares.iter().filter(|x| **x > 1).count() as u32
    }

    fn raster(&mut self, claim: &Claim) {
        for i in claim.left..(claim.left + claim.width) {
            for j in claim.top..(claim.top + claim.height) {
                self.inc(i as usize, j as usize);
            }
        }
    }

    fn check(&self, claim: &Claim) -> bool {
        for i in claim.left..(claim.left + claim.width) {
            for j in claim.top..(claim.top + claim.height) {
                if self[(i as usize, j as usize)] != 1 {
                    return false;
                }
            }
        }

        true
    }
}

impl Index<(usize, usize)> for Cloth {
    type Output = u8;

    fn index(&self, idx: (usize, usize)) -> &u8 {
        &self.squares[idx.0 + idx.1 * self.stride]
    }
}

impl IndexMut<(usize, usize)> for Cloth {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut u8 {
        &mut self.squares[idx.0 + idx.1 * self.stride]
    }
}

struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let get = |idx| -> Result<u32, Self::Err> {
            Ok(caps
                .get(idx)
                .ok_or_else(|| err_msg("Invalid format"))?
                .as_str()
                .parse()?)
        };

        Ok(Claim {
            id: get(1)?,
            left: get(2)?,
            top: get(3)?,
            width: get(4)?,
            height: get(5)?,
        })
    }
}
