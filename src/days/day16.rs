use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

    fn draw_path(&self, path: Vec<PathNode>) {
        for i in 0..self.tiles.len() {
            if i % self.cols == 0 {
                println!();
            }

            let v = match path.iter().find(|pn| pn.pos == i) {
                Some(pn) => match pn.dir {
                    Up => '^',
                    Down => 'v',
                    Left => '<',
                    Right => '>',
                },
                None => match self.tiles[i] {
                    Tile::End => 'E',
                    Tile::Wall => '#',
                    Tile::Free => '.',
                    Tile::Start => 'S',
                },
            };

            print!("{v}");
        }

        println!()
    }

    fn draw_best_nodes(&self, nodes: &HashSet<usize>) {
        for i in 0..self.tiles.len() {
            if i % self.cols == 0 {
                println!();
            }

            let v = match nodes.contains(&i) {
                true => 'O',
                false => match self.tiles[i] {
                    Tile::End => 'E',
                    Tile::Wall => '#',
                    Tile::Free => '.',
                    Tile::Start => 'S',
                },
            };

            print!("{v}");
        }

        println!()
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

#[derive(Clone, Debug)]
struct PathNode {
    pos: usize,
    dir: Direction,
}

fn find_all_best_paths(
    grid: &Grid,
    start: usize,
    start_direction: Direction,
    end: usize,
) -> (usize, Vec<Vec<PathNode>>) {
    let mut distance: HashMap<(usize, Direction), usize> = HashMap::new();
    let mut predecessors: HashMap<(usize, Direction), Vec<(usize, Direction)>> = HashMap::new();
    let mut smallest_cost = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        direction: start_direction,
        cost: 0,
    });
    distance.insert((start, start_direction), 0);

    while let Some(Node {
        pos,
        direction,
        cost,
    }) = queue.pop()
    {
        if cost > *distance.get(&(pos, direction)).unwrap_or(&usize::MAX) {
            continue;
        }

        if pos == end {
            smallest_cost = cost.min(smallest_cost);
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            let next_pos = grid.next_pos(pos, d);
            if !grid.allowed(pos, d) || grid.tiles[next_pos] == Tile::Wall {
                return None;
            }

            let next_cost = cost + 1 + direction.cost_change(d);
            if next_cost > smallest_cost && pos != end {
                return None;
            }

            if match distance.get(&(next_pos, d)) {
                Some(&existing_cost) => match next_cost.cmp(&existing_cost) {
                    Ordering::Less => true,
                    Ordering::Equal => false,
                    Ordering::Greater => return None,
                },
                None => true,
            } {
                distance.insert((next_pos, d), next_cost);
                predecessors.insert((next_pos, d), vec![(pos, direction)]);
            } else {
                if let Some(pred_list) = predecessors.get_mut(&(next_pos, d)) {
                    if !pred_list.contains(&(pos, direction)) {
                        pred_list.push((pos, direction));
                    }
                }
            }

            distance.insert((next_pos, d), next_cost);
            Some(Node {
                pos: next_pos,
                direction: d,
                cost: next_cost,
            })
        }));
    }

    let mut all_paths = vec![];
    let mut stack = VecDeque::new();

    for end_direction in ALL_DIRECTIONS
        .iter()
        .filter(|&&d| distance.contains_key(&(end, d)))
    {
        stack.push_back((
            vec![PathNode {
                pos: end,
                dir: *end_direction,
            }],
            (end, *end_direction),
        ));
    }

    while let Some((mut cur_path, cur_node)) = stack.pop_back() {
        if cur_node == (start, start_direction) {
            cur_path.reverse();
            all_paths.push(cur_path);
        } else if let Some(prev_nodes) = predecessors.get(&cur_node) {
            for &(prev_pos, prev_dir) in prev_nodes {
                let mut new_path = cur_path.clone();
                new_path.push(PathNode {
                    pos: prev_pos,
                    dir: prev_dir,
                });
                stack.push_back((new_path, (prev_pos, prev_dir)));
            }
        }
    }

    (smallest_cost, all_paths)
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::parse(&input);
    let (smallest_cost, all_paths) = find_all_best_paths(&grid, grid.start(), Right, grid.end());

    let mut unique_nodes = HashSet::new();
    for path in all_paths {
        unique_nodes.extend(path.iter().map(|node| node.pos));
        grid.draw_path(path);
    }

    grid.draw_best_nodes(&unique_nodes);

    (smallest_cost, unique_nodes.len())
}

pub fn day16(input: String) {
    let (p1, p2) = solve(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
