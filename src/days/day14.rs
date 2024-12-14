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

        let dx = (dx * times) % grid_x;
        let dy = (dy * times) % grid_y;

        let grid_x = grid_x - 1;
        let grid_y = grid_y - 1;

        let mut new_x = x + dx;
        if new_x < 0 {
            new_x = new_x + grid_x + 1;
        } else if new_x > grid_x {
            new_x = (new_x % grid_x) - 1;
        }

        let mut new_y = y + dy;
        if new_y < 0 {
            new_y = new_y + grid_y + 1;
        } else if new_y > grid_y {
            new_y = (new_y % grid_y) - 1;
        }
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

        // q1, x < div && y < div
        // q2, x < div && y > div
        // q3, x > div && y < div
        // q4, x > div && y > div
        if x < divider_x && y < divider_y {
            println!("Robot position: {:?}", self.pos);
            println!("Robot in Q1");
            counter.0 += 1;
        } else if x < divider_x && y > divider_y {
            println!("Robot position: {:?}", self.pos);
            println!("Robot in Q2");
            counter.1 += 1;
        } else if x > divider_x && y < divider_y {
            println!("Robot position: {:?}", self.pos);
            println!("Robot in Q3");
            counter.2 += 1;
        } else if x > divider_x && y > divider_y {
            println!("Robot position: {:?}", self.pos);
            println!("Robot in Q4");
            counter.3 += 1;
        }
    }
}

pub fn day14(input: String) {
    // let grid_bounds = (101, 103);
    let grid_bounds = (10, 6);
    let bots = input
        .lines()
        .map(|line| {
            let mut r = Robot::from_line(line);
            r.walk(100, grid_bounds);
            r
        })
        .collect_vec();

    draw(&bots, grid_bounds);
    let quadrants = bots.iter().fold((0, 0, 0, 0), |mut acc, cur| {
        cur.get_quadrant(&mut acc, grid_bounds);
        acc
    });

    let safety_factor = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
    println!("Quadrants: {:?}", quadrants);
    println!("Part 1: {safety_factor}");
}

fn draw(bots: &Vec<Robot>, grid_bounds: (isize, isize)) {
    let (x, y) = grid_bounds;
    let divider_x = x / 2;
    let divider_y = y / 2;

    for j in 0..=y {
        for i in 0..=x {
            if i == divider_x {
                print!(" ");
                continue;
            }
            if j == divider_y {
                print!(" ");
                continue;
            }
            let in_pos = bots.iter().filter(|b| b.pos == (i, j)).count();
            if in_pos > 0 {
                print!("{in_pos}");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod day14_test {
    use super::Robot;

    #[test]
    fn test_walk() {
        let r = Robot {
            pos: (2, 4),
            v: (2, -3),
        };
        let grid_bounds = (11, 6);

        for i in 0..10000 {
            let mut r1 = r.clone();
            r1.walk(i, grid_bounds);

            let mut r2 = r.clone();
            for _ in 0..i {
                r2.walk(1, grid_bounds);
            }

            assert_eq!(r1, r2, "failed with i = {}", i);
        }
    }
}
