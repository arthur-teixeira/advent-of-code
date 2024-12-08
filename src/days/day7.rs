use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Operation {
    PLUS,
    MULT,
}

impl Operation {
    fn execute(&self, a: usize, b: usize) -> usize {
        match self {
            Self::PLUS => a + b,
            Self::MULT => a * b,
        }
    }
}

const OPERATIONS: [Operation; 2] = [Operation::PLUS, Operation::MULT];

#[derive(Debug)]
struct Equation {
    target: usize,
    components: Vec<usize>,
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(": ");

        let target: usize = parts.next().unwrap().parse().unwrap();
        let mut components: Vec<usize> = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();

        components.reverse();

        Self { target, components }
    }

    fn test(&self) -> usize {
        for combination in std::iter::repeat(OPERATIONS.iter().cloned())
            .take(self.components.len() - 1)
            .multi_cartesian_product()
        {
            let mut stack = Vec::new();
            stack.extend_from_slice(&self.components);

            for op in &combination {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(op.execute(a, b));
            }

            assert!(stack.len() == 1);
            if stack[0] == self.target {
                return self.target
            }
        }

        0
    }
}

pub fn day7(input: String) {
    let part1 = input
        .lines()
        .map(|line| Equation::from_line(line))
        .fold(0, |acc, cur| acc + cur.test());

    println!("Part 1: {part1}");
}
