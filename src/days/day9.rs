#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Content {
    Free,
    Occupied(usize),
}

pub fn day9(input: String) {
    let mut disk: Vec<Content> = Vec::new();
    let mut id = 0;
    input.trim().chars().enumerate().for_each(|(idx, c)| {
        let is_free = idx & 1 == 1;
        let space = c.to_digit(10).unwrap() as usize;

        let new_elems;
        if is_free {
            new_elems = vec![Content::Free; space];
        } else {
            new_elems = vec![Content::Occupied(id); space];
            id += 1;
        }

        disk.extend_from_slice(&new_elems);
    });

    println!("Part 1: {}", part1(&mut disk));
}

fn part1(disk: &mut Vec<Content>) -> usize {
    let mut end = disk.len() - 1;
    let mut start = 0;

    while end > start {
        let val = match disk[end] {
            Content::Occupied(id) => id,
            Content::Free => {
                end -= 1;
                continue;
            }
        };

        match disk[start] {
            Content::Occupied(_) => {
                start += 1;
                continue;
            }
            Content::Free => {
                disk[start] = Content::Occupied(val);
                disk[end] = Content::Free;
                start += 1;
                end -= 1;
            }
        }
    }

    let mut acc: usize = 0;
    for i in 0..=start {
        let cur = match disk[i] {
            Content::Occupied(n) => n,
            Content::Free => unreachable!(),
        };

        acc += cur * i;
    }

    acc
}
