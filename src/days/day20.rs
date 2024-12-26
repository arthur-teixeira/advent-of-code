use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

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

use Direction::{Down, Left, Right, Up};
const ALL_DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    size: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Cheat(usize, usize);
// impl PartialEq for Cheat {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0 && self.1 == other.1 || self.1 == other.0 && self.0 == other.1
//     }
// }

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut n = 0;
        let tiles = input
            .lines()
            .flat_map(|l| {
                n += 1;
                l.chars().map(Tile::new).collect_vec()
            })
            .collect_vec();

        Grid { size: n, tiles }
    }

    fn next_pos(&self, pos: usize, dir: Direction) -> usize {
        match dir {
            Direction::Up => pos - self.size,
            Direction::Down => pos + self.size,
            Direction::Left => pos - 1,
            Direction::Right => pos + 1,
        }
    }

    fn allowed(&self, pos: usize, dir: Direction) -> bool {
        !match dir {
            Up => pos < self.size,
            Right => pos % self.size == self.size - 1,
            Down => pos / self.size == self.size - 1,
            Left => pos % self.size == 0,
        }
    }

    fn draw_cheat(&self, cheat: &Cheat, path: &Vec<usize>) {
        let Cheat(c1, c2) = cheat;
        for i in 0..self.tiles.len() {
            if i % self.size == 0 {
                println!();
            }

            let v = match self.tiles[i] {
                _ if i == *c1 => '1',
                _ if i == *c2 => '2',
                _ if path.contains(&i) => 'O',
                Tile::End => 'E',
                Tile::Wall => '#',
                Tile::Free => '.',
                Tile::Start => 'S',
            };

            print!("{v}");
        }

        println!()
    }

    fn start(&self) -> usize {
        self.tiles.iter().position(|t| *t == Tile::Start).unwrap()
    }

    fn end(&self) -> usize {
        self.tiles.iter().position(|t| *t == Tile::End).unwrap()
    }

    fn is_valid(&self, initial_pos: usize, next_pos: usize) -> bool {
        if self.tiles[next_pos] == Tile::Wall {
            return false;
        }
        for dir in ALL_DIRECTIONS {
            if self.allowed(next_pos, dir) {
                let next_next = self.next_pos(next_pos, dir);
                if self.tiles[next_next] != Tile::Wall && next_next != initial_pos {
                    return true;
                }
            }
        }

        false
    }

    fn cheats(&self) -> Vec<Cheat> {
        // Two walls, that don't hit another wall after
        // One wall and one normal tile
        // One normal tile and one wall, as long as there is no wall after
        let mut cheats = Vec::new();
        for i in 0..self.tiles.len() {
            for dir in ALL_DIRECTIONS {
                if self.allowed(i, dir) {
                    let next_pos = self.next_pos(i, dir);
                    // Is valid if at least one position after cheat is not a wall or previous
                    // position and is within bounds
                    if self.is_valid(i, next_pos) {
                        let cheat = Cheat(i, self.next_pos(i, dir));
                        if !cheats.contains(&cheat) {
                            cheats.push(cheat);
                        }
                    }
                };
            }
        }

        cheats
    }
}

fn bfs(grid: &Grid, cheat: &Cheat) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let dest = grid.end();
    queue.push_back((grid.start(), vec![]));
    let mut seen = HashSet::new();

    while let Some((pos, path)) = queue.pop_front() {
        // We can only ignore c2 if coming from c1
        let prev = if path.len() > 2 {
            Some(&path[path.len() - 2])
        } else {
            None
        };

        let &Cheat(c1, c2) = cheat;
        let at_first_cheat_node = pos == c1;
        let ignore_wall = match grid.tiles[pos] {
            // If we have a cheat, check to see if we are at the first node
            // If it is the first node, ignore that it is a wall
            // If it is the second node, check if we are coming from the first node
            // If so, ignore that it is a wall
            // Else, ignore cheat and discard path
            // TODO: If we reach the first cheat node, force path to go to second cheat node
            Tile::Free => true,
            Tile::Start => true,
            Tile::End => true,
            Tile::Wall if pos == c1 => match prev {
                Some(&prev) => !(prev == c2),
                None => true,
            },
            Tile::Wall if pos == c2 => match prev {
                Some(&prev) => {
                    println!("Going through second cheat node, previous node is {prev}, first cheat node is {c1}");
                    c1 == prev
                }
                None => {
                    println!("Reached second cheat node, but there is no prev");
                    true
                }
            },
            Tile::Wall if pos != c1 && pos != c2 => false,
            Tile::Wall => unreachable!(),
        };

        if !ignore_wall {
            continue;
        }

        if pos == dest {
            return path;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&dir| {
            if grid.allowed(pos, dir) {
                let next_pos = grid.next_pos(pos, dir);
                if at_first_cheat_node && next_pos != c2 {
                    return None;
                }

                if seen.insert(next_pos) {
                    let mut np = path.clone();
                    np.push(next_pos);
                    return Some((next_pos, np));
                }
            }

            return None;
        }));
    }

    vec![]
}

pub fn day20(input: String) {
    let grid = Grid::parse(&input);
    let cheats = grid.cheats();
    let no_cheats_path = bfs(&grid, &Cheat(0, 0));
    let start_cost = no_cheats_path.len();
    println!("Initial cost: {start_cost}");
    println!("Possible cheats: {}", cheats.len());
    let threads = std::thread::available_parallelism().expect("Expected num threads");
    let chunks = cheats
        .chunks(cheats.len() / threads)
        .map(|c| c.to_vec())
        .collect_vec();

    let grid = Arc::new(grid);
    let mut handles = vec![];
    for chunk in chunks {
        let grid = grid.clone();
        let handle = std::thread::spawn(move || {
            let mut econ_map = HashMap::new();
            for cheat in chunk.iter() {
                let with_cheat = bfs(&grid, &cheat);
                let n = with_cheat.len();
                let diff = start_cost - n;
                if diff > 0 && n > 0 {
                    econ_map.entry(diff).and_modify(|v| *v += 1).or_insert(1);
                }
            }

            econ_map
                .iter()
                .fold(0, |acc, (&k, v)| if k >= 100 { acc + v } else { acc })
        });
        handles.push(handle);
    }

    let mut acc = 0;
    for h in handles {
        acc += h.join().unwrap();
    }

    println!("Part 1: {acc}");
}
