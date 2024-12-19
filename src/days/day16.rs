use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::BuildHasher,
};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Start,
    End,
    Free,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Free,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_rotated(self, other: Direction) -> bool {
        match self {
            Up | Down => [Left, Right].contains(&other),
            Left | Right => [Up, Down].contains(&other),
        }
    }

    fn cost_change(self, other: Direction) -> usize {
        if self == other {
            0
        } else if self.is_rotated(other) {
            1000
        } else {
            2000
        }
    }
}

use Direction::{Down, Left, Right, Up};
const ALL_DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

struct Grid {
    tiles: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut rows = 0;
        let tiles = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(Tile::new).collect_vec()
            })
            .collect_vec();

        let cols = tiles.len() / rows;
        Grid { rows, cols, tiles }
    }

    fn next_pos(&self, pos: usize, dir: Direction) -> usize {
        match dir {
            Direction::Up => pos - self.cols,
            Direction::Down => pos + self.cols,
            Direction::Left => pos - 1,
            Direction::Right => pos + 1,
        }
    }

    fn allowed(&self, pos: usize, dir: Direction) -> bool {
        !match dir {
            Up => pos < self.cols,
            Right => pos % self.cols == self.cols - 1,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
        }
    }

    fn start(&self) -> usize {
        self.tiles.iter().position(|t| *t == Tile::Start).unwrap()
    }

    fn end(&self) -> usize {
        self.tiles.iter().position(|t| *t == Tile::End).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    direction: Direction,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_smallest_cost(grid: &Grid, start: usize, start_direction: Direction, end: usize) -> usize {
    let mut seen: HashSet<(usize, Direction)> = HashSet::new();
    let mut distance: HashMap<(usize, Direction), usize> = HashMap::new();
    let mut smallest_cost = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        direction: start_direction,
        cost: 0,
    });

    while let Some(Node {
        pos,
        direction,
        cost,
    }) = queue.pop()
    {
        seen.insert((pos, direction));

        if pos == end {
            smallest_cost = cost.min(smallest_cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            if !grid.allowed(pos, d) {
                return None;
            }

            let next_pos = grid.next_pos(pos, d);
            if seen.contains(&(next_pos, d)) {
                return None;
            }

            if grid.tiles[next_pos] == Tile::Wall {
                return None;
            }

            let next_cost = cost + 1 + direction.cost_change(d);
            if let Some(&prevcost) = distance.get(&(next_pos, d)) {
                if prevcost <= next_cost {
                    return None;
                }
            }

            if next_cost >= smallest_cost {
                return None;
            }

            distance.insert((next_pos, d), next_cost);
            Some(Node {
                pos: next_pos,
                direction: d,
                cost: next_cost,
            })
        }));
    }

    smallest_cost
}

pub fn day16(input: String) {
    let grid = Grid::parse(&input);
    println!(
        "{:?}",
        find_smallest_cost(&grid, grid.start(), Right, grid.end())
    );
}
