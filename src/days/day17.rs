use std::cmp;

use itertools::Itertools;

enum Op {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Op {
    fn parse(op: usize, operand: usize) -> Self {
        match op {
            0 => Op::Adv(operand),
            1 => Op::Bxl(operand),
            2 => Op::Bst(operand),
            3 => Op::Jnz(operand),
            4 => Op::Bxc(operand),
            5 => Op::Out(operand),
            6 => Op::Bdv(operand),
            7 => Op::Cdv(operand),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    ops: Vec<usize>,
    out: Vec<usize>,
}

impl Machine {
    fn run(&mut self) {
        while let Some(()) = self.exec() {}
    }

    fn exec(&mut self) -> Option<()> {
        let op = *self.ops.get(self.ip)?;
        self.ip += 1;
        let operand = *self.ops.get(self.ip)?;
        self.ip += 1;
        let op = Op::parse(op, operand);
        self.run_op(op);

        Some(())
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            op if op <= 3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 | _ => unreachable!(),
        }
    }

    fn run_op(&mut self, op: Op) {
        match op {
            Op::Adv(operand) => {
                let denominator = usize::pow(2, self.get_combo_operand(operand) as u32);
                self.a /= denominator;
            }
            Op::Bxl(operand) => {
                self.b ^= operand;
            }
            Op::Bst(operand) => {
                self.b = self.get_combo_operand(operand) % 8;
            }
            Op::Jnz(operand) => {
                if self.a != 0 {
                    self.ip = operand;
                }
            }
            Op::Bxc(_) => {
                self.b ^= self.c;
            }
            Op::Out(operand) => self.out.push(self.get_combo_operand(operand) % 8),
            Op::Bdv(operand) => {
                let denominator = usize::pow(2, self.get_combo_operand(operand) as u32);
                self.b = self.a / denominator;
            }
            Op::Cdv(operand) => {
                let denominator = u64::pow(2, self.get_combo_operand(operand) as u32);
                self.c = self.a / denominator as usize;
            }
        }
    }
}

fn parse(input: &str) -> Machine {
    let mut parts = input.split("\n\n");
    let mut registers = parts.next().unwrap().lines();

    let ops = parts
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|c| c.trim().parse().unwrap())
        .collect_vec();

    let a = registers
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let b = registers
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let c = registers
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();

    Machine {
        a,
        b,
        c,
        ip: 0,
        ops,
        out: vec![],
    }
}

pub fn day17(input: String) {
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut machine = parse(input);
    machine.run();
    println!("Part 1: {}", machine.out.iter().join(","));
}

fn is_eq(a: &[usize], b: &[usize]) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}

fn cmp(machine: &Machine, a: usize, cmp_index: usize, possible: &mut Vec<usize>) -> usize {
    for i in 0..8 {
        let new_a = (a << 3) | i;
        let mut cur = machine.clone();
        cur.a = new_a;
        cur.run();

        let out = cur.out.clone();
        if is_eq(&out, &cur.ops[cmp_index..]) {
            if cur.out == cur.ops {
                possible.push(new_a);
            } else {
                cmp(machine, new_a, cmp_index - 1, possible);
            }
        }
    }

    *possible.iter().min().unwrap_or(&0)
}

fn part2(input: &str) {
    let machine = parse(input);
    let mut possible = vec![];
    println!(
        "Part 2: {:?}",
        cmp(&machine, 0, machine.ops.len() - 1, &mut possible)
    );
}
