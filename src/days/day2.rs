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

    pub fn is_safe_dampened(&self) -> bool {
        let mut tested: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();

        tested.insert(0);
        queue.push_back(0);

        let mut first = true;
        let report = &self.0;
        let mut is_valid = false;
        'outer: while first || queue.len() > 0 {
            first = false;
            let i_to_ignore = queue.pop_front();

            let mut descending: Option<bool> = None;
            for i in 1..report.len() {
                let prev_level;
                let level = report[i];
                if let Some(ignored) = i_to_ignore {
                    if i == ignored {
                        continue;
                    }

                    if i - 1 == ignored {
                        if i == 1 {
                            continue;
                        }

                        prev_level = report[i - 2];
                    } else {
                        prev_level = report[i - 1];
                    }
                } else {
                    prev_level = report[i - 1];
                }

                if level.abs_diff(prev_level) > 3 || level == prev_level {
                    if !tested.contains(&(i - 1)) {
                        tested.insert(i - 1);
                        queue.push_back(i - 1);
                    }
                    if !tested.contains(&i) {
                        tested.insert(i);
                        queue.push_back(i);
                    }
                    is_valid = false;
                    continue 'outer;
                }

                match descending {
                    Some(is_descending) => {
                        if (is_descending && level >= prev_level)
                            || (!is_descending && level <= prev_level)
                        {
                            if !tested.contains(&(i - 1)) {
                                tested.insert(i - 1);
                                queue.push_back(i - 1);
                            }
                            if !tested.contains(&i) {
                                tested.insert(i);
                                queue.push_back(i);
                            }

                            is_valid = false;
                            continue 'outer;
                        } else {
                            is_valid = true;
                        }
                    }
                    None => {
                        descending = Some(level < prev_level);
                        continue;
                    }
                }
            }

            if is_valid {
                return true;
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

#[cfg(test)]
mod day2_tests {
    use super::Report;

    #[test]
    fn part2_edge_cases() {
        assert_eq!(
            Report(vec![19, 20, 21, 22, 23, 25, 26, 30]).is_safe_dampened(),
            false
        );
        assert_eq!(
            Report(vec![40, 41, 41, 43, 45, 43]).is_safe_dampened(),
            false
        );

        assert_eq!(
            Report(vec![43, 40, 41, 44, 45, 46, 48, 51]).is_safe_dampened(),
            true
        );
    }
}
