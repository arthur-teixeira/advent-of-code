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
                Some(elems) => {
                    for elem in elems {
                        if seen.contains(elem) {
                            return false;
                        }
                    }
                }
                None => continue,
            }
        }

        true
    }

    fn middle(&self) -> usize {
        let idx = (self.0.len() - 1) / 2;
        self.0[idx]
    }

    fn fix(&mut self, rules: &Rules) {
        let mut seen: HashMap<usize, usize> = HashMap::new();

        loop {
            let swaps: Vec<(usize, usize)> = self
                .0
                .iter()
                .enumerate()
                .filter_map(|(i, num)| {
                    seen.insert(*num, i);

                    if let Some(elems) = rules.get(num) {
                        for elem in elems {
                            if let Some(idx) = seen.get(elem) {
                                if i > *idx {
                                    return Some((i, *idx));
                                }
                            }
                        }
                        return None;
                    }

                    None
                })
                .collect();

            if swaps.is_empty() {
                return;
            }

            for swap in swaps {
                let (i, j) = swap;
                self.0.swap(i, j);
            }
        }
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

    fn part1(&self) -> usize {
        self.updates
            .iter()
            .filter(|u| u.is_valid(&self.rules))
            .fold(0, |acc, cur| acc + cur.middle())
    }

    fn part2(&mut self) -> usize {
        self.updates
            .iter_mut()
            .filter(|u| !u.is_valid(&self.rules))
            .map(|u| {
                u.fix(&self.rules);
                u
            })
            .fold(0, |acc, cur| acc + cur.middle())
    }
}

pub fn day5(input: String) {
    let mut a = Rulebook::from_string(&input);
    println!("Part 1: {:?}", a.part1());
    println!("Part 2: {:?}", a.part2());
}

#[cfg(test)]
mod day5_test {
    use super::Rulebook;

    const INPUT: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn test_swaps() {
        let mut rulebook = Rulebook::from_string(INPUT);
        let mut a = rulebook
            .updates
            .iter_mut()
            .filter(|u| !u.is_valid(&rulebook.rules));

        let first = a.next().expect("Should have first");
        assert_eq!(vec![75, 97, 47, 61, 53], first.0);
        first.fix(&rulebook.rules);
        assert_eq!(vec![97, 75, 47, 61, 53], first.0);

        let second = a.next().expect("Should have second");
        assert_eq!(vec![61, 13, 29], second.0);
        second.fix(&rulebook.rules);
        assert_eq!(vec![61, 29, 13], second.0);

        let third = a.next().expect("Should have third");
        assert_eq!(vec![97, 13, 75, 29, 47], third.0);
        third.fix(&rulebook.rules);
        assert_eq!(vec![97, 75, 47, 29, 13], third.0);

        assert_eq!(0, a.count());
    }
}
