use std::str::FromStr;

use failure::{err_msg, Error};
use regex::Regex;

pub fn solve() {
    let game_setup: GameSetup = include_str!("data/day9.txt").trim().parse().unwrap();
    println!("Part 1: {}", part1(game_setup));
    println!("Part 2: {}", part2(game_setup));
}

fn part1(game_setup: GameSetup) -> u32 {
    let mut game = Game::new(game_setup);
    game.play()
}

fn part2(game_setup: GameSetup) -> u32 {
    let game_setup = GameSetup {
        num_marbles: 100 * game_setup.num_marbles,
        ..game_setup
    };
    let mut game = Game::new(game_setup);
    game.play()
}

#[derive(Clone)]
struct Marble {
    left: usize,
    right: usize,
}

impl Marble {
    fn new() -> Marble {
        Marble { left: 0, right: 0 }
    }
}

struct Game {
    setup: GameSetup,
    board: Vec<Marble>,
    scores: Vec<u32>,
    current_idx: usize,
    next_marble: u32,
    next_player: u32,
}

impl Game {
    fn new(setup: GameSetup) -> Game {
        Game {
            setup: setup,
            board: vec![Marble::new(); (setup.num_marbles + 1) as usize],
            scores: vec![0; setup.num_players as usize],
            current_idx: 0,
            next_marble: 1,
            next_player: 0,
        }
    }

    fn left(&mut self, count: usize) {
        for _ in 0..count {
            self.current_idx = self.board[self.current_idx].left;
        }
    }

    fn right(&mut self, count: usize) {
        for _ in 0..count {
            self.current_idx = self.board[self.current_idx].right;
        }
    }

    fn remove(&mut self) -> u32 {
        let idx = self.current_idx;
        let left_idx = self.board[idx].left;
        let right_idx = self.board[idx].right;

        self.board[idx].left = idx;
        self.board[idx].right = idx;

        self.board[left_idx].right = right_idx;
        self.board[right_idx].left = left_idx;

        self.current_idx = right_idx;
        idx as u32
    }

    fn insert_right(&mut self, idx: u32) {
        let idx = idx as usize;
        let left_idx = self.current_idx;
        let right_idx = self.board[left_idx].right;

        self.board[idx].left = left_idx;
        self.board[idx].right = right_idx;

        self.board[right_idx].left = idx;
        self.board[left_idx].right = idx;

        self.current_idx = idx;
    }

    fn play(&mut self) -> u32 {
        while !self.done() {
            self.step();
        }

        self.highest_score()
    }

    fn highest_score(&self) -> u32 {
        *self.scores.iter().max().unwrap()
    }

    fn step(&mut self) {
        if self.next_marble % 23 != 0 {
            self.right(1);
            self.insert_right(self.next_marble);
        } else {
            self.left(7);
            self.scores[self.next_player as usize] += self.next_marble + self.remove();
        }

        self.next_marble += 1;
        self.next_player = (self.next_player + 1) % self.setup.num_players;
    }

    fn done(&self) -> bool {
        self.next_marble >= self.setup.num_marbles
    }
}

#[derive(Copy, Clone)]
struct GameSetup {
    num_players: u32,
    num_marbles: u32,
}

impl FromStr for GameSetup {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let caps = re.captures(s).ok_or_else(|| err_msg("Invalid format"))?;

        let get = |idx| -> Result<u32, Self::Err> {
            Ok(caps
                .get(idx)
                .ok_or_else(|| err_msg("Invalid format"))?
                .as_str()
                .parse()?)
        };

        Ok(GameSetup {
            num_players: get(1)?,
            num_marbles: get(2)?,
        })
    }
}
