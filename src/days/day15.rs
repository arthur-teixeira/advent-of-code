use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Kind {
    Robot,
    Box,
    Wall,
    Empty,
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
    movable: bool,
}

impl Object {
    fn new(c: char) -> Self {
        let kind = Kind::from_char(c);

        Self {
            movable: match kind {
                Kind::Robot => true,
                Kind::Box => true,
                Kind::Empty => true,
                Kind::Wall => false,
            },
            kind,
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

    fn do_move(&mut self, pos: (isize, isize), m: &Move) -> bool {
        let (x, y) = pos;
        let Move(dx, dy) = *m;
        let new_pos = (x + dx, y + dy);
        let (new_x, new_y) = new_pos;
        let obj = self
            .at(x, y)
            .expect("Expected position to be within bounds");

        if obj.kind == Kind::Empty {
            return true;
        }
        if obj.movable {
            let moved = self.do_move(new_pos, m);
            if moved {
                self.values[new_x as usize][new_y as usize] = self.values[x as usize][y as usize];
                self.values[x as usize][y as usize] = Object {
                    movable: true,
                    kind: Kind::Empty,
                };

                match obj.kind {
                    Kind::Robot => {
                        self.bot_position = new_pos;
                    }
                    _ => {}
                };
                return true;
            } else {
                return false;
            }
        }

        return false;
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
                    _ => 0,
                };
            }
        }

        acc
    }
}

pub fn day15(input: String) {
    let (mut grid, moves) = parse(&input);
    println!("Part 1: {}", part1(&mut grid, moves));
}

fn parse(input: &str) -> (Grid, Vec<Move>) {
    let mut input = input.split("\n\n");
    let mat = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().map(Object::new).collect_vec())
        .collect_vec();

    let rows = mat.len() as isize;
    let cols = mat[0].len() as isize;

    let bot_y = mat
        .iter()
        .position(|row| row.iter().position(|o| o.kind == Kind::Robot).is_some())
        .expect("Expected guard to be in the map");

    let bot_x = mat[bot_y]
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

fn part1(grid: &mut Grid, moves: Vec<Move>) -> isize {
    for m in moves {
        grid.do_move(grid.bot_position, &m);
    }

    grid.get_gps_sum()
}
