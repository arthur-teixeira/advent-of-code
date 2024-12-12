use std::{collections::HashMap, time::Instant};

fn blink_opt(n: usize) -> (usize, Option<usize>) {
    match n {
        0 => (1, None),
        n if n.checked_ilog10().unwrap_or(0) & 1 == 1 => {
            let as_str = n.to_string();
            let len = as_str.len();

            let (first, last) = as_str.split_at(len / 2);
            (first.parse().unwrap(), Some(last.parse().unwrap()))
        }
        n => (n * 2024, None),
    }
}

pub fn day11(input: String) {
    let parse_start = Instant::now();
    let mut freq_count: HashMap<_, _> = HashMap::new();
    input
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .for_each(|n| {
            freq_count.entry(n).and_modify(|v| *v += 1).or_insert(1);
        });

    let parse_end = parse_start.elapsed();
    let part1_start = Instant::now();
    println!("Part 1: {}", solve(&mut freq_count, 25));
    let part1_end = part1_start.elapsed();
    let part2_start = Instant::now();
    println!("Part 2: {}", solve(&mut freq_count, 50));
    let part2_end = part2_start.elapsed();

    println!(
        "Times:\nParse {:?}\n Part1 {:?}\n Part2 {:?}\nOverall {:?}",
        parse_end,
        part1_end,
        part2_end,
        parse_start.elapsed()
    );
}

fn solve(freq_count: &mut HashMap<usize, usize>, rounds: usize) -> usize {
    for _ in 0..rounds {
        let k: Vec<(usize, usize)> = freq_count.iter().map(|(k, v)| (*k, *v)).collect();
        for (num, freq) in k {
            let (first, last) = blink_opt(num);

            freq_count.entry(num).and_modify(|v| *v -= freq);
            freq_count
                .entry(first)
                .and_modify(|e| *e += freq)
                .or_insert(freq);

            if let Some(last) = last {
                freq_count
                    .entry(last)
                    .and_modify(|e| *e += freq)
                    .or_insert(freq);
            }
        }
    }
    freq_count.values().sum::<usize>()
}
