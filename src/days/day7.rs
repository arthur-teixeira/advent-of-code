use std::time::Instant;

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

        Self { target, components }
    }
}

pub fn day7(input: String) {
    let parse_start = Instant::now();
    let eqs: Vec<_> = input.lines().map(Equation::from_line).collect();
    println!("Parsing took {:?}", parse_start.elapsed());

    let start = Instant::now();
    let part1 = eqs
        .iter()
        .filter(|e| is_solvable_part1(e))
        .fold(0, |acc, cur| acc + cur.target);

    println!("Part 1: {part1} in {:?}", start.elapsed());
    let start = Instant::now();
    let part2 = eqs
        .iter()
        .filter(|e| is_solvable_part2(e))
        .fold(0, |acc, cur| acc + cur.target);
    println!("Part 2: {part2} in {:?}", start.elapsed());
    println!("Overall: {:?}", parse_start.elapsed());
}

fn is_solvable_part1(eq: &Equation) -> bool {
    if eq.target == 0 && eq.components.len() == 0 {
        return true;
    }

    if eq.components.len() == 0 {
        return false;
    }

    let target = eq.target;
    let component = *eq.components.last().unwrap();

    if target == component {
        return true;
    }

    if target % component == 0 {
        let remaining = target / component;
        if remaining == 1 {
            return true;
        }

        if is_solvable_part1(&Equation {
            target: remaining,
            components: eq.components[0..eq.components.len() - 1].to_owned(),
        }) {
            return true;
        }
    }

    if target > component {
        return is_solvable_part1(&Equation {
            target: eq.target - component,
            components: eq.components[0..eq.components.len() - 1].to_owned(),
        });
    }

    false
}

fn is_solvable_part2(eq: &Equation) -> bool {
    if eq.target == 0 && eq.components.len() == 0 {
        return true;
    }

    if eq.components.len() == 0 {
        return false;
    }

    let target = eq.target;
    let component = *eq.components.last().unwrap();

    if target == component {
        return true;
    }

    let num_digits = component.to_string().len();
    let target_piece = target % (10_usize.pow(num_digits as u32));

    if target_piece == component {
        let target_string = target.to_string();
        let remaining = target_string[0..target_string.len() - num_digits]
            .parse()
            .unwrap();

        if is_solvable_part2(&Equation {
            target: remaining,
            components: eq.components[0..eq.components.len() - 1].to_owned(),
        }) {
            return true;
        };
    }

    if target % component == 0 {
        let remaining = target / component;
        if remaining == 1 {
            return true;
        }

        if is_solvable_part2(&Equation {
            target: remaining,
            components: eq.components[0..eq.components.len() - 1].to_owned(),
        }) {
            return true;
        }
    }

    if target > component {
        return is_solvable_part2(&Equation {
            target: eq.target - component,
            components: eq.components[0..eq.components.len() - 1].to_owned(),
        });
    }

    false
}
