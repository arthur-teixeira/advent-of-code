use std::{
    fmt::{write, Display},
    thread::sleep,
    time::Duration,
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Kind {
    Robot,
    Box,
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl Kind {
    fn from_char(c: char) -> Self {
        match c {
            '@' => Kind::Robot,
            'O' => Kind::Box,
            '#' => Kind::Wall,
            '.' => Kind::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Object {
    kind: Kind,
}

impl Object {
    fn movable(&self) -> bool {
        match self.kind {
            Kind::Wall => false,
            _ => true,
        }
    }
    fn new(c: char) -> Self {
        let kind = Kind::from_char(c);

        Self { kind }
    }

    fn new_part2(c: char) -> Vec<Self> {
        match c {
            '@' => vec![Self { kind: Kind::Robot }, Self { kind: Kind::Empty }],
            'O' => vec![
                Self {
                    kind: Kind::BoxLeft,
                },
                Self {
                    kind: Kind::BoxRight,
                },
            ],
            '#' => vec![Self { kind: Kind::Wall }, Self { kind: Kind::Wall }],
            '.' => vec![Self { kind: Kind::Empty }, Self { kind: Kind::Empty }],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move(isize, isize);

impl Move {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self(-1, 0)),
            '>' => Some(Self(0, 1)),
            'v' => Some(Self(1, 0)),
            '<' => Some(Self(0, -1)),
            '\n' => None,
            _ => unreachable!(),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self(-1, 0) => '^',
            Self(0, 1) => '>',
            Self(1, 0) => 'v',
            Self(0, -1) => '<',
            _ => unreachable!(),
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Grid {
    rows: isize,
    cols: isize,
    bot_position: (isize, isize),
    values: Vec<Vec<Object>>,
}
impl Grid {
    fn at(&self, x: isize, y: isize) -> Option<Object> {
        self.values
            .get(x as usize)
            .and_then(|row| row.get(y as usize))
            .and_then(|obj| Some(*obj))
    }

    fn can_move(&self, pos: (isize, isize), m: &Move, checking_side: bool) -> bool {
        let (x, y) = pos;
        let Move(dx, dy) = *m;

        let obj = self
            .at(x, y)
            .expect("Expected position to be within bounds");

        if obj.kind == Kind::Empty {
            return true;
        }
        if obj.kind == Kind::Wall {
            return false;
        }

        match obj.kind {
            Kind::Empty => true,
            Kind::BoxLeft if !checking_side => {
                assert!(self.at(x, y + 1).unwrap().kind == Kind::BoxRight);
                if dx == 0 && dy == 1 {
                    obj.movable() && self.can_move((x, y + 1), m, true)
                } else {
                    obj.movable()
                        && self.can_move((x + dx, y + dy), m, false)
                        && self.can_move((x, y + 1), m, true)
                }
            }
            Kind::BoxRight if !checking_side => {
                assert!(self.at(x, y - 1).unwrap().kind == Kind::BoxLeft);
                if dx == 0 && dy == -1 {
                    obj.movable() && self.can_move((x, y - 1), m, true)
                } else {
                    obj.movable()
                        && self.can_move((x + dx, y + dy), m, false)
                        && self.can_move((x, y - 1), m, true)
                }
            }
            _ => obj.movable() && self.can_move((x + dx, y + dy), m, false),
        }
    }

    fn do_move(&mut self, pos: (isize, isize), m: &Move, moving_side: bool) {
        let (x, y) = pos;
        let Move(dx, dy) = *m;
        let obj = self
            .at(x, y)
            .expect("Expected position to be within bounds");

        if obj.kind == Kind::Empty {
            return;
        }

        if obj.kind == Kind::BoxLeft && !moving_side {
            let rhs = self.at(x, y + 1).expect("Expected box right hand side");
            assert!(rhs.kind == Kind::BoxRight);
            self.do_move((x, y + 1), m, true);
        }

        if obj.kind == Kind::BoxRight && !moving_side {
            let lhs = self.at(x, y - 1).expect("Expected box left hand side");
            assert!(lhs.kind == Kind::BoxLeft);
            self.do_move((x, y - 1), m, true);
        }

        if !obj.movable() {
            return;
        }

        let new_pos = (x + dx, y + dy);
        let (new_x, new_y) = new_pos;
        self.do_move(new_pos, m, false);

        self.values[new_x as usize][new_y as usize] = self.values[x as usize][y as usize];
        self.values[x as usize][y as usize] = Object { kind: Kind::Empty };

        if obj.kind == Kind::Robot {
            self.bot_position = new_pos;
        }
    }

    fn draw(&self) {
        print!("\x1B[2J");
        for x in 0..self.rows {
            for y in 0..self.cols {
                let elem = match self.values[x as usize][y as usize].kind {
                    Kind::Box => 'O',
                    Kind::Wall => '#',
                    Kind::Robot => '@',
                    Kind::Empty => '.',
                    Kind::BoxLeft => '[',
                    Kind::BoxRight => ']',
                };

                print!("{elem}");
            }
            println!();
        }
    }

    fn get_gps_sum(&self) -> isize {
        let mut acc = 0;
        for x in 0..self.rows {
            for y in 0..self.cols {
                acc += match self.at(x, y).expect("Expected to be within bounds").kind {
                    Kind::Box => (100 * x) + y,
                    Kind::BoxLeft => (100 * x) + y,
                    _ => 0,
                };
            }
        }

        acc
    }
}

pub fn day15(input: String) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str, part: u8) -> (Grid, Vec<Move>) {
    let mut input = input.split("\n\n");
    let mat;
    if part == 1 {
        mat = input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().map(Object::new).collect_vec())
            .collect_vec();
    } else {
        mat = input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().map(Object::new_part2).flatten().collect_vec())
            .collect_vec();
    }

    let rows = mat.len() as isize;
    let cols = mat[0].len() as isize;

    let bot_x = mat
        .iter()
        .position(|row| row.iter().position(|o| o.kind == Kind::Robot).is_some())
        .expect("Expected guard to be in the map");

    let bot_y = mat[bot_x]
        .iter()
        .position(|o| o.kind == Kind::Robot)
        .expect("Expected guard to be in the map");

    let g = Grid {
        rows,
        bot_position: (bot_x as isize, bot_y as isize),
        cols,
        values: mat,
    };

    let moves = input
        .next()
        .unwrap()
        .chars()
        .map(Move::from_char)
        .flatten()
        .collect_vec();

    (g, moves)
}

fn part1(input: &str) -> isize {
    let (mut grid, moves) = parse(&input, 1);
    solve(&mut grid, moves)
}

fn part2(input: &str) -> isize {
    let (mut grid, moves) = parse(&input, 2);
    solve(&mut grid, moves)
}

fn solve(grid: &mut Grid, moves: Vec<Move>) -> isize {
    for m in moves {
        if grid.can_move(grid.bot_position, &m, false) {
            grid.do_move(grid.bot_position, &m, false);
        }
    }

    grid.get_gps_sum()
}

