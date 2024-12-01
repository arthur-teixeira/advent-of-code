pub fn day1(input: String) {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            left.push(line[0].parse().unwrap());
            right.push(line[1].parse().unwrap());

            (left, right)
        });

    left.sort();
    right.sort();

    let diffs = left.into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r));

    println!("Part 1: {:?}", diffs);
}
