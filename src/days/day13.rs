use std::time::Instant;

use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn parse_line(input: &str, start: usize) -> (isize, isize) {
        let values = &mut input[start..].split(", ");
        let x = values.next().unwrap().parse().unwrap();
        let y = values.next().unwrap()[2..].parse().unwrap();

        (x, y)
    }

    fn for_part2(&mut self) -> &Self {
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
        self
    }

    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let btn_a = lines.next().unwrap();
        let a = Self::parse_line(btn_a, 12);

        let btn_b = lines.next().unwrap();
        let b = Self::parse_line(btn_b, 12);
        let prize = lines.next().unwrap();
        let prize = Self::parse_line(prize, 9);

        assert!(lines.next().is_none());

        Self { a, b, prize }
    }

    fn solve_system(&self) -> (usize, usize) {
        // c1 * self.a.0 + c2 * self.b.0 = self.prize.0;
        // c1 * self.a.1 + c2 * self.b.1 = self.prize.1;

        let determinant = (self.a.0 * self.b.1) - (self.b.0 * self.a.1);
        let x_determinant = (self.prize.0 * self.b.1) - (self.prize.1 * self.b.0);
        let y_determinant = (self.a.0 * self.prize.1) - (self.a.1 * self.prize.0);

        let c1 = x_determinant / determinant;
        let c2 = y_determinant / determinant;

        if c1 * self.a.0 + c2 * self.b.0 == self.prize.0
            && c1 * self.a.1 + c2 * self.b.1 == self.prize.1
        {
            assert!(c1 >= 0 && c2 >= 0);
            (c1 as usize, c2 as usize)
        } else {
            (0, 0)
        }
    }

    fn calculate_tokens(&self) -> usize {
        let (c1, c2) = self.solve_system();
        (3 * c1) + c2
    }
}

pub fn day13(input: String) {
    let start = Instant::now();
    let mut machines = input.split("\n\n").map(Machine::new).collect_vec();
    println!("Parsing took {:?}", start.elapsed());
    let p1 = Instant::now();
    println!("Part 1: {:?}", part1(&machines));
    println!("Part 1 took {:?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2: {:?}", part2(&mut machines));
    println!("Part 2 took {:?}", p2.elapsed());

    println!("Overall: {:?}", start.elapsed());
}

fn part1(machines: &Vec<Machine>) -> usize {
    machines
        .iter()
        .fold(0, |acc, cur| acc + cur.calculate_tokens())
}

fn part2(machines: &mut Vec<Machine>) -> usize {
    machines
        .iter_mut()
        .fold(0, |acc, cur| acc + cur.for_part2().calculate_tokens())
}
