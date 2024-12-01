use std::collections::HashMap;

pub fn day1(input: String) {
    let (left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            left.push(line[0].parse().unwrap());
            right.push(line[1].parse().unwrap());

            (left, right)
        });

    println!("Part 1: {:?}", part1(&left, &right));
    println!("Part 2: {:?}", part2(&left, &right));
}

fn part1(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut left = left.clone();
    left.sort();
    let mut right = right.clone();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r))
}

fn part2(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let right_vec = VectorizedList::new(right);
    right_vec.similarity(left)
}

struct VectorizedList(HashMap<usize, usize>);

impl VectorizedList {
    fn similarity(&self, other: &Vec<usize>) -> usize {
        other
            .iter()
            .map(|n| {
                let freq = self.0.get(n).unwrap_or(&0);
                n * freq
            })
            .fold(0, |acc, cur| acc + cur)
    }

    fn new(l: &Vec<usize>) -> VectorizedList {
        let mut vec = VectorizedList(HashMap::new());
        l.iter().for_each(|n| {
            vec.0.entry(*n).and_modify(|c| *c += 1).or_insert(1);
        });

        vec
    }
}
