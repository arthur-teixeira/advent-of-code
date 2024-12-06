use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn coords(&self) -> (isize, isize) {
        match self {
            Self::LEFT => (-1, 0),
            Self::RIGHT => (1, 0),
            Self::UP => (0, -1),
            Self::DOWN => (0, 1),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }
}

#[derive(Clone)]
struct Guard {
    pos: (usize, usize),
    direction: Direction,
    seen: HashSet<(usize, usize)>,
    new_obstacle: Option<(usize, usize)>,
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        let mut guard = Guard {
            pos: (x, y),
            direction: Direction::UP,
            seen: HashSet::new(),
            new_obstacle: None,
        };
        guard.seen.insert(guard.pos);
        guard
    }

    fn new_pos(&self) -> (usize, usize) {
        let (x, y) = self.pos;
        let (x_mod, y_mod) = self.direction.coords();

        let new_x = (x as isize + x_mod) as usize;
        let new_y = (y as isize + y_mod) as usize;

        (new_x, new_y)
    }

    fn check_bounds(&self, map: &Map) -> bool {
        let rows = map.len();
        let cols = map[0].len();

        let (new_x, new_y) = self.new_pos();
        new_x < cols && new_y < rows
    }

    fn is_blocked(&self, map: &Map) -> bool {
        let (new_x, new_y) = self.new_pos();
        if !self.check_bounds(map) {
            return true;
        }

        let mut result = map[new_y][new_x] == '#';
        if let Some(obs) = self.new_obstacle {
            result = result || (new_x, new_y) == obs;
        }

        result
    }

    fn walk(&mut self, map: &Map) {
        if self.is_blocked(map) {
            self.direction = self.direction.rotate_right();
            return;
        }
        self.pos = self.new_pos();
    }

    fn track_walk(&mut self, map: &Map) {
        self.walk(map);
        self.seen.insert(self.pos);
    }
}

type Map = Vec<Vec<char>>;

pub fn day6(input: String) {
    let map: Map = input.lines().map(|l| l.chars().collect()).collect();
    assert!(map.len() > 0);

    let part1_result = part1(&map);
    println!("Part 1: {:?}", part1_result.seen.len());
    println!("Part 2: {:?}", part2(map, &part1_result.seen));
}

fn part1(map: &Map) -> Guard {
    let guard_y = map
        .iter()
        .position(|row| row.contains(&'^'))
        .expect("Expected guard to be in the map");
    let guard_x = map[guard_y]
        .iter()
        .position(|e| *e == '^')
        .expect("Expected guard to be in the map");

    let mut guard = Guard::new(guard_x, guard_y);
    while guard.check_bounds(&map) {
        guard.track_walk(&map);
    }

    guard
}

fn check_cycle(map: &Map, mut tortoise: Guard) -> bool {
    let mut hare = tortoise.clone();

    while hare.check_bounds(&map) {
        hare.walk(&map);
        if !hare.check_bounds(&map) {
            break;
        }
        hare.walk(&map);
        tortoise.walk(&map);
        if tortoise.pos == hare.pos && tortoise.direction == hare.direction {
            return true;
        }
    }

    false
}

fn part2(map: Map, part1_seen: &HashSet<(usize, usize)>) -> usize {
    let guard_y = map
        .iter()
        .position(|row| row.contains(&'^'))
        .expect("Expected guard to be in the map");
    let guard_x = map[guard_y]
        .iter()
        .position(|e| *e == '^')
        .expect("Expected guard to be in the map");

    let threads = std::thread::available_parallelism().expect("Expected num threads");
    let possibilities: Vec<(usize, usize)> = part1_seen.iter().map(|s| *s).collect();

    let mut handles = Vec::new();
    let chunks: Vec<Vec<(usize, usize)>> = possibilities
        .chunks(possibilities.len() / threads)
        .map(|c| c.to_vec())
        .collect();

    let map = Arc::new(RwLock::new(map));

    for chunk in chunks {
        let map = map.clone();
        let handle = std::thread::spawn(move || {
            let map = map.read().unwrap();
            let mut cycles = 0;
            for (x, y) in chunk {
                let mut tortoise = Guard::new(guard_x, guard_y);
                tortoise.new_obstacle = Some((x, y));
                if check_cycle(&map, tortoise) {
                    cycles += 1;
                }
            }

            cycles
        });

        handles.push(handle);
    }

    let mut cycles = 0;
    for handle in handles {
        cycles += handle.join().unwrap();
    }

    cycles
}
