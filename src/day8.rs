use std::collections::VecDeque;

pub fn solve() {
    let node: Node = include_str!("data/day8.txt")
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<VecDeque<i32>>()
        .into();

    println!("Part 1: {}", part1(&node));
    println!("Part 2: {}", part2(&node));
}

fn part1(node: &Node) -> i32 {
    node.metadata_sum()
}

fn part2(node: &Node) -> i32 {
    node.value()
}

struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>,
}

impl From<VecDeque<i32>> for Node {
    fn from(mut spec: VecDeque<i32>) -> Node {
        Node::from_spec(&mut spec)
    }
}

impl Node {
    fn from_spec(spec: &mut VecDeque<i32>) -> Node {
        let n_children = spec.pop_front().unwrap();
        let n_metadata = spec.pop_front().unwrap();

        let mut children = Vec::new();
        for _ in 0..n_children {
            children.push(Node::from_spec(spec));
        }

        let mut metadata = Vec::new();
        for _ in 0..n_metadata {
            metadata.push(spec.pop_front().unwrap());
        }

        Node { metadata, children }
    }

    fn metadata_sum(&self) -> i32 {
        self.metadata.iter().sum::<i32>()
            + self.children.iter().map(|c| c.metadata_sum()).sum::<i32>()
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let mut v = 0;

            for &i in &self.metadata {
                if i >= 1 && i <= self.children.len() as i32 {
                    v += self.children[(i - 1) as usize].value();
                }
            }

            v
        }
    }
}
