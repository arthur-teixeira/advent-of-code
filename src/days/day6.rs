use std::collections::HashSet;

#[derive(Debug)]
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

struct Guard {
    pos: (usize, usize),
    direction: Direction,
    seen: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        let mut guard = Guard {
            pos: (x, y),
            direction: Direction::UP,
            seen: HashSet::new(),
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
        assert!(self.check_bounds(map));
        map[new_y][new_x] == '#'
    }

    fn walk(&mut self, map: &Map) {
        if self.is_blocked(map) {
            self.direction = self.direction.rotate_right();
        }
        self.pos = self.new_pos();
        self.seen.insert(self.pos);
    }
}

type Map = Vec<Vec<char>>;

pub fn day6(input: String) {
    let map: Map = input.lines().map(|l| l.chars().collect()).collect();
    assert!(map.len() > 0);

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
        guard.walk(&map);
    }

    println!("Part 1: {:?}", guard.seen.len());
}
