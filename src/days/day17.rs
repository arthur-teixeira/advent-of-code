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

#[derive(Debug)]
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
    let mut machine = parse(&input);
    machine.run();
    println!("{}", machine.out.iter().join(","));
}
