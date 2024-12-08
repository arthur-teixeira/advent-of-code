use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Operation {
    PLUS,
    MULT,
    CONCAT,
}

impl Operation {
    fn execute(&self, a: usize, b: usize) -> usize {
        match self {
            Self::PLUS => a + b,
            Self::MULT => a * b,
            Self::CONCAT => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

const OPERATIONS_PART1: [Operation; 2] = [Operation::PLUS, Operation::MULT];

#[derive(Debug)]
struct Equation {
    target: usize,
    components: Vec<usize>,
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(": ");

        let target: usize = parts.next().unwrap().parse().unwrap();
        let components: Vec<usize> = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();

        // components.reverse();

        Self { target, components }
    }

    fn test(&mut self, ops: &[Operation]) -> usize {
        for combination in std::iter::repeat(ops)
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
                return self.target;
            }
        }

        0
    }

    fn part_two(&self) -> usize {
        let mut ops = Vec::new();
        test(
            self.target,
            self.target,
            &self.components,
            &self.components,
            &mut ops,
        )
    }
}

// Last op could be || iff last digit(s) of target equals last elem
// Last op could be * iff target is divisible by last elem
// else op could be +
// Construct list of ops
// Evaluate result
fn test(
    original_target: usize,
    targets: Vec<usize>,
    original_components: &[usize],
    components: &[usize],
    ops: &mut Vec<Vec<Operation>>,
) -> usize {
    // if components.len() == 1 {
    //     println!(
    //         "Finished test with target {}, components {:?}, ops {:?}",
    //         original_target, original_components, ops
    //     );
    //
    //     let mut stack = Vec::new();
    //     stack.extend_from_slice(&original_components);
    //     stack.reverse();
    //
    //     while ops.len() > 0 {
    //         let op = ops.pop().unwrap();
    //         let a = stack.pop().unwrap();
    //         let b = stack.pop().unwrap();
    //         stack.push(op.execute(a, b));
    //         println!("Stack head: {:?}", stack.last());
    //     }
    //
    //     assert!(stack.len() == 1);
    //     println!("Finished test with result {:?}", stack);
    //     if stack[0] == original_target {
    //         return original_target;
    //     }
    //
    //     return 0;
    // }

    let last = components.last().unwrap();
    let num_digits = last.to_string().len();
    let target_piece = target % (10_usize.pow(num_digits as u32));

    let new_targets = Vec::new();

    let mut possible_ops = Vec::new();
    if target_piece == *last {
        possible_ops.push(Operation::CONCAT);
        let target_string = target.to_string();
        new_targets.push(target_string
            .split(&target_piece.to_string())
            .next()
            .unwrap()
            .parse()
            .unwrap());
    }
    if target % last == 0 {
        possible_ops.push(Operation::MULT);
        new_targets.push(target / last);
    } else {
        possible_ops.push(Operation::PLUS);
        new_targets.push(target - last);
    }

    test(
        original_target,
        new_targets,
        original_components,
        &components[0..components.len() - 1],
        ops,
    )
}

pub fn day7(input: String) {
    let part1 = input
        .lines()
        .map(|line| Equation::from_line(line))
        .fold(0, |acc, mut cur| acc + cur.test(&OPERATIONS_PART1));
    println!("Part 1: {part1}");

    let part2 = input
        .lines()
        .map(|line| Equation::from_line(line))
        .fold(0, |acc, cur| acc + cur.part_two());
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod day7_test {
    use super::test;

    fn run(target: usize, components: &[usize]) -> usize {
        let mut ops = Vec::new();
        test(target, target, components, components, &mut ops)
    }

    #[test]
    fn test_part2() {
        // assert_eq!(156, run(156, &vec![15, 6]));
        assert_eq!(7290, run(7290, &vec![6, 8, 6, 15]));
    }
}
