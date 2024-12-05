use std::collections::{HashMap, HashSet};

type Rules = HashMap<usize, Vec<usize>>;

#[derive(Debug)]
struct Update(Vec<usize>);

impl Update {
    fn is_valid(&self, rules: &Rules) -> bool {
        let mut seen: HashSet<usize> = HashSet::new();

        for num in &self.0 {
            seen.insert(*num);
            match rules.get(num) {
                None => continue,
                Some(num_rules) => {
                    for elem in num_rules {
                        if seen.contains(elem) {
                            return false;
                        }
                    }
                }
            }
            // let num_rules = rules.get(num).unwrap();
        }

        true
    }

    fn middle(&self) -> usize {
        let idx = (self.0.len() - 1)/2;
        self.0[idx]
    }
}

#[derive(Debug)]
struct Rulebook {
    rules: Rules,
    updates: Vec<Update>,
}

impl Rulebook {
    fn from_string(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let rules = parts.next().unwrap();
        let updates = parts.next().unwrap();

        let rules = rules.lines().fold(Rules::new(), |mut acc, cur| {
            let mut parts = cur.split("|");
            let before: usize = parts.next().unwrap().parse().unwrap();
            let after: usize = parts.next().unwrap().parse().unwrap();

            acc.entry(before)
                .and_modify(|v| v.push(after))
                .or_insert(vec![after]);

            acc
        });

        let updates = updates
            .lines()
            .map(|line| Update(line.split(',').map(|e| e.parse().unwrap()).collect()))
            .collect();

        Self { rules, updates }
    }

    fn calculate_answer(&self) -> usize {
        self.updates
            .iter()
            .filter(|u| u.is_valid(&self.rules))
            .fold(0, |acc, cur| acc + cur.middle())
    }
}

pub fn day5(input: String) {
    let a = Rulebook::from_string(&input);
    println!("Part 1: {:?}", a.calculate_answer());
}
