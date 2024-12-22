use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Node {
    Free,
    Corrupted,
}

struct Grid {
    size: usize,
    nodes: Vec<Node>,
}

impl Grid {
    fn at(&self, pos: (usize, usize)) -> &Node {
        &self.nodes[pos.0 * self.size + pos.1]
    }

    fn at_mut(&mut self, pos: (usize, usize)) -> &mut Node {
        &mut self.nodes[pos.0 * self.size + pos.1]
    }

    fn draw(&self) {
        for j in 0..self.size {
            for i in 0..self.size {
                let node = self.at((i, j));
                // let node = self.nodes[i * self.cols + j];
                print!(
                    "{}",
                    match node {
                        Node::Free => '.',
                        Node::Corrupted => '#',
                    }
                );
            }

            println!()
        }
    }

    fn add_corrupted_nodes(&mut self, nodes: &[(usize, usize)]) {
        for node in nodes {
            *self.at_mut(*node) = Node::Corrupted;
        }
    }

    fn end(&self) -> (usize, usize) {
        let s = self.size - 1;
        (s, s)
    }
}

pub fn day18(input: String) {
    let p1_cutoff = 1024;
    let size = 71;
    let nodes = get_nodes_from_input(&input);
    let mut grid = parse(&nodes[..p1_cutoff], size);
    let p1 = bfs(&grid);
    println!("Part 1: {}", p1.len());
    println!("Part 2: {:?}", part2(&mut grid, &nodes[p1_cutoff..], p1));
}

fn part2(
    grid: &mut Grid,
    mut remaining_nodes: &[(usize, usize)],
    initial_path: Vec<(usize, usize)>,
) -> (usize, usize) {
    let mut path_nodes: HashSet<(usize, usize)> = HashSet::from_iter(initial_path);

    'outer: loop {
        for i in 0..remaining_nodes.len() {
            let cur_node = remaining_nodes[i];
            if path_nodes.contains(&cur_node) {
                grid.add_corrupted_nodes(&remaining_nodes[0..=i]);
                let new_path = bfs(grid);
                if new_path.len() > 0 {
                    path_nodes = HashSet::from_iter(new_path);
                    remaining_nodes = &remaining_nodes[i + 1..];
                    continue 'outer;
                } else {
                    return cur_node;
                }
            };
        }
    }
}

fn get_nodes_from_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();

            (x, y)
        })
        .collect_vec()
}

fn parse(occupied_nodes: &[(usize, usize)], size: usize) -> Grid {
    let mut nodes = Vec::with_capacity(size * size);

    for i in 0..size {
        for j in 0..size {
            if occupied_nodes.contains(&(i, j)) {
                nodes.push(Node::Corrupted);
            } else {
                nodes.push(Node::Free);
            }
        }
    }

    Grid { size, nodes }
}

fn bfs(grid: &Grid) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let dest = grid.end();
    queue.push_back(((0, 0), vec![]));
    let mut seen = HashSet::new();

    while let Some((pos, path)) = queue.pop_front() {
        let (pos_x, pos_y) = pos;
        match grid.at(pos) {
            Node::Corrupted => continue,
            _ => {}
        };

        if pos == dest {
            return path;
        }

        if pos_y >= 1 {
            let up = (pos_x, pos_y - 1);
            if seen.insert(up) {
                let mut np = path.clone();
                np.push(up);
                queue.push_back((up, np));
            }
        }

        if pos_y + 1 < grid.size {
            let down = (pos_x, pos_y + 1);
            if seen.insert(down) {
                let mut np = path.clone();
                np.push(down);
                queue.push_back((down, np));
            }
        }

        if pos_x >= 1 {
            let left = (pos_x - 1, pos_y);
            if seen.insert(left) {
                let mut np = path.clone();
                np.push(left);
                queue.push_back((left, np));
            }
        }

        if pos_x + 1 < grid.size {
            let right = (pos_x + 1, pos_y);
            if seen.insert(right) {
                let mut np = path.clone();
                np.push(right);
                queue.push_back((right, np));
            }
        }
    }

    vec![]
}
