use std::collections::{HashSet, VecDeque};

struct Report(Vec<usize>);

impl Report {
    fn from_line(line: &str) -> Self {
        Self(line.split(' ').map(|elem| elem.parse().unwrap()).collect())
    }

    fn is_safe(&self) -> bool {
        let mut descending = false;
        for (i, level) in self.0.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let prev_level = self.0[i - 1];
            if i == 1 {
                descending = level < &prev_level;
            }

            if level.abs_diff(prev_level) > 3 {
                return false;
            }
            if (descending && level >= &prev_level) || (!descending && level <= &prev_level) {
                return false;
            }
        }

        true
    }

    fn ignore_at(&self, i: usize) -> Vec<usize> {
        let mut new = Vec::new();
        new.copy_from_slice(&self.0[0..i]);
        new.copy_from_slice(&self.0[i + 1..]);

        new
    }

    fn is_safe_dampened(&self) -> bool {
        let mut descending = false;
        let mut tested: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        let mut is_valid = false;
        let mut first = true;
        'outer: while !is_valid || first {
            if !first && queue.len() == 0 {
                break;
            }

            first = false;
            let i_to_ignore = queue.pop_front();
            let report;
            if let Some(ignored) = i_to_ignore {
                let new = self.ignore_at(ignored);
                report = &new;
            } else {
                report = &self.0;
            }

            'inner: for (i, level) in report.iter().enumerate() {
                if i == 0 {
                    continue 'inner;
                }

                let prev_level = self.0[i - 1];
                if i == 1 {
                    descending = level < &prev_level;
                }

                if level.abs_diff(prev_level) > 3 {
                    if !tested.contains(&i) {
                        tested.insert(i);
                        queue.push_back(i);
                    }
                    if !tested.contains(&(i - 1)) {
                        tested.insert(i - 1);
                        queue.push_back(i - 1);
                    }
                    continue 'outer;
                }

                if (descending && level >= &prev_level) || (!descending && level <= &prev_level) {
                    if !tested.contains(&i) {
                        tested.insert(i);
                        queue.push_back(i);
                    }
                    if !tested.contains(&(i - 1)) {
                        tested.insert(i - 1);
                        queue.push_back(i - 1);
                    }

                    continue 'outer;
                }

                is_valid = true;
                break 'outer;
            }
        }

        is_valid
    }
}

pub fn day2(input: String) {
    let reports = input.lines().map(|line| Report::from_line(line)).collect();
    println!("Part 1: {}", part1(&reports));
    println!("Part 2: {}", part2(&reports));
}

fn part1(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|r| r.is_safe())
        .fold(0, |acc, _| acc + 1)
}

fn part2(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|r| r.is_safe_dampened())
        .fold(0, |acc, _| acc + 1)
}
