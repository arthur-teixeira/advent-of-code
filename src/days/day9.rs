pub fn day9(input: String) {
    let mut disk: Vec<isize> = Vec::new();
    let mut id = 0;
    input.trim().chars().enumerate().for_each(|(idx, c)| {
        let is_free = idx & 1 == 1;
        let space = c.to_digit(10).unwrap() as usize;

        let new_elems;
        if is_free {
            new_elems = vec![-1; space];
        } else {
            new_elems = vec![id; space];
            id += 1;
        }

        disk.extend_from_slice(&new_elems);
    });

    println!("Part 1: {}", part1(disk));
    println!("Part 2: {}", part2(input));
}

fn part1(mut disk: Vec<isize>) -> usize {
    let mut end = disk.len() - 1;
    let mut start = 0;

    while end > start {
        let val = match disk[end] {
            -1 => {
                end -= 1;
                continue;
            }
            id => id,
        };

        match disk[start] {
            -1 => {
                disk[start] = val;
                disk[end] = -1;
                start += 1;
                end -= 1;
            }
            _ => {
                start += 1;
                continue;
            }
        }
    }

    let mut acc: usize = 0;
    for i in 0..=start {
        let cur = disk[i];
        assert!(cur >= 0);
        acc += (cur as usize) * i;
    }

    acc
}

#[derive(Clone, Debug)]
enum Content {
    // length
    Free(usize),
    // ID, length
    Occupied(usize, usize),
}

fn part2(input: String) -> usize {
    let mut disk: Vec<Content> = Vec::new();
    let mut id = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let is_free = i & 1 == 1;
        let space = c.to_digit(10).unwrap() as usize;

        if is_free {
            disk.push(Content::Free(space));
        } else {
            disk.push(Content::Occupied(id, space));
            id += 1;
        }
    }

    let mut i = 0;
    while i < disk.len() {
        let available_space = match disk[i] {
            Content::Free(space) => space,
            _ => {
                i += 1;
                continue;
            }
        };

        for j in (i..disk.len()).rev() {
            if j < i {
                unreachable!();
            }
            match disk[j] {
                Content::Occupied(id, space) if space <= available_space => {
                    disk[i] = Content::Occupied(id, space);
                    disk[j] = Content::Free(space);

                    if available_space > space {
                        let a = &[Content::Free(available_space - space)];
                        disk.splice(i + 1..i + 1, a.iter().cloned());
                    } else {
                        i += 1;
                    }

                    break;
                }
                _ => continue,
            }
        }

        i += 1;
    }

    let mut idx = 0;
    let mut sum = 0;
    for e in disk {
        match e {
            Content::Free(space) => {
                idx += space;
            },
            Content::Occupied(id, space) => {
                // id = 2
                // 0 * 2 + 1 * 2 + 2 * 2 + 3 * 2
                // =
                // 2(0 + 1 + 2 + 3)
                sum += id * (idx..idx+space).sum::<usize>();
                idx += space;
            }
        }
    }

    sum
}
