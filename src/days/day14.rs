use itertools::Itertools;

#[derive(Debug)]
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

    fn walk(&mut self, times: isize, grid_size: (isize, isize)) {
        let (x, y) = self.pos;
        let (dx, dy) = self.v;
        let dx = dx * times;
        let dy = dy * times;

        let (grid_x, grid_y) = grid_size;
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
}

pub fn day14(input: String) {
    let mut robots = input.lines().map(Robot::from_line).collect_vec();
    let r = &mut robots[0];
    // let grid_size = (101, 103);
    let grid_size = (11, 7);

    println!("Guard vel is {:?}", r.v);
    for _ in 0..10 {
        r.walk(1, grid_size);
        println!("Guard moved to {:?}", r.pos);
    }
}
