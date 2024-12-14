use std::io::stdin;

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    pos: (isize, isize),
    v: (isize, isize),
}

impl Robot {
    fn parse_part(p: &str) -> (isize, isize) {
        let mut nums = p[2..].split(',');

        (
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        )
    }

    fn from_line(line: &str) -> Self {
        let mut parts = line.split(" ");
        let pos = parts.next().unwrap();
        let vel = parts.next().unwrap();

        assert!(parts.next().is_none());

        Self {
            pos: Self::parse_part(pos),
            v: Self::parse_part(vel),
        }
    }

    fn walk(&mut self, times: isize, grid_bounds: (isize, isize)) {
        let (x, y) = self.pos;
        let (dx, dy) = self.v;
        let (grid_x, grid_y) = grid_bounds;

        let new_x = (x + dx * times).rem_euclid(grid_x + 1);
        let new_y = (y + dy * times).rem_euclid(grid_y + 1);

        self.pos = (new_x, new_y);
    }

    fn get_quadrant(
        &self,
        counter: &mut (usize, usize, usize, usize),
        grid_bounds: (isize, isize),
    ) {
        let (x, y) = self.pos;
        let (grid_x, grid_y) = grid_bounds;
        let divider_x = grid_x / 2;
        let divider_y = grid_y / 2;

        if x < divider_x && y < divider_y {
            counter.0 += 1;
        } else if x < divider_x && y > divider_y {
            counter.1 += 1;
        } else if x > divider_x && y < divider_y {
            counter.2 += 1;
        } else if x > divider_x && y > divider_y {
            counter.3 += 1;
        }
    }
}

pub fn day14(input: String) {
    draw_iter(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn draw_iter(input: &str) {
    let grid_bounds = (100, 102);
    let bots = input
        .lines()
        .map(|line| Robot::from_line(line))
        .collect_vec();
    let mut i = 1;
    while i <= 7051 {
        let mut bs = bots.clone();
        bs.iter_mut().for_each(|b| {
            b.walk(i, grid_bounds);
        });
        draw(&bs, grid_bounds);
        println!("Walked {i}");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let c = input.chars().next().unwrap();
        match c {
            'c' => i += 100,
            ' ' => i += 1000,
            't' => i += 10,
            's' => break,
            _ => i += 1,
        }
    }
}

fn part1(input: &str) -> usize {
    let grid_bounds = (100, 102);
    let quadrants = input
        .lines()
        .map(|line| {
            let mut r = Robot::from_line(line);
            r.walk(100, grid_bounds);
            r
        })
        .fold((0, 0, 0, 0), |mut acc, cur| {
            cur.get_quadrant(&mut acc, grid_bounds);
            acc
        });

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn part2(input: &str) -> usize {
    let grid_bounds = (100, 102);
    let bots = input
        .lines()
        .map(|line| Robot::from_line(line))
        .collect_vec();

    let mut x_variances = vec![];
    let mut y_variances = vec![];
    for i in 0..103 {
        let mut bs = bots.clone();

        let bs = bs
            .iter_mut()
            .map(|b| {
                b.walk(i + 1, grid_bounds);
                b.to_owned()
            })
            .collect_vec();

        let x_var = x_variance(&bs);
        let y_var = y_variance(&bs);
        x_variances.push((i + 1, x_var));
        y_variances.push((i + 1, y_var));
    }

    let mut bx = 0;
    let mut min_var_x = usize::MAX;
    for (sec, var) in &x_variances {
        if *var < min_var_x {
            bx = *sec;
            min_var_x = *var;
        }
    }

    let mut by = 0;
    let mut min_var_y = usize::MAX;
    for (sec, var) in &y_variances {
        if *var < min_var_y {
            by = *sec;
            min_var_y = *var;
        }
    }

    let (h, w) = (grid_bounds.1 + 1, grid_bounds.0 + 1);
    let t = bx + (((51 * (by - bx)).rem_euclid(h)) * w);

    let mut bots = bots.clone();
    bots.iter_mut()
        .map(|b| {
            b.walk(t, grid_bounds);
            b.to_owned()
        })
        .collect_vec();

    t as usize
}

fn x_variance(bots: &Vec<Robot>) -> usize {
    let n = bots.len();
    let xs = bots.iter().map(|b| b.pos.0 as usize).collect_vec();
    let avg = xs.iter().sum::<usize>() / n;

    let mut var = 0;
    for i in 0..n {
        var += avg.abs_diff(xs[i]).pow(2);
    }

    var /= n;

    var
}

fn y_variance(bots: &Vec<Robot>) -> usize {
    let n = bots.len();
    let ys = bots.iter().map(|b| b.pos.1 as usize).collect_vec();
    let avg = ys.iter().sum::<usize>() / n;

    let mut var = 0;
    for i in 0..n {
        var += avg.abs_diff(ys[i]).pow(2);
    }

    var /= n;
    var
}

fn draw(bots: &Vec<Robot>, grid_bounds: (isize, isize)) {
    let mut buf = String::new();

    let (x, y) = grid_bounds;
    for j in 0..=y {
        for i in 0..=x {
            let in_pos = bots.iter().filter(|b| b.pos == (i, j)).count();
            if in_pos > 0 {
                buf.push('*');
            } else {
                buf.push(' ');
            }
        }

        buf.push('\n');
    }

    // clear
    print!("\x1B[2J");
    println!("{buf}");
}
