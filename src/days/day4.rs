pub fn day4(input: String) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn search_horizontally(input: &str, chars_to_find: &[char]) -> usize {
    input
        .lines()
        .map(|line| {
            let n = chars_to_find.len();
            let mut forward = 0;
            let mut backward = 0;

            let mut count: usize = 0;
            for ch in line.chars() {
                if ch == chars_to_find[forward] {
                    forward = (forward + 1) % n;
                    if forward == 0 {
                        count += 1;
                    }
                } else {
                    forward = if ch == chars_to_find[0] { 1 } else { 0 };
                }

                if ch == chars_to_find[n - 1 - backward] {
                    backward = (backward + 1) % n;
                    if backward == 0 {
                        count += 1;
                    }
                } else {
                    backward = if ch == chars_to_find[n - 1] { 1 } else { 0 };
                }
            }

            count
        })
        .reduce(|acc, cur| acc + cur)
        .unwrap_or(0)
}

fn search_vertically(input: &str, chars_to_find: &[char]) -> usize {
    let as_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    assert!(as_matrix.len() > 0);

    let rows = as_matrix.len();
    let cols = as_matrix[0].len();

    let n = chars_to_find.len();
    let mut forward;
    let mut backward;

    let mut count = 0;
    for i in 0..cols {
        forward = 0;
        backward = 0;
        for j in 0..rows {
            let ch = as_matrix[j][i];
            if ch == chars_to_find[forward] {
                forward = (forward + 1) % n;
                if forward == 0 {
                    count += 1;
                }
            } else {
                forward = if ch == chars_to_find[0] { 1 } else { 0 };
            }

            if ch == chars_to_find[n - 1 - backward] {
                backward = (backward + 1) % n;
                if backward == 0 {
                    count += 1;
                }
            } else {
                backward = if ch == chars_to_find[n - 1] { 1 } else { 0 };
            }
        }
    }

    count
}

fn search_diagonally(input: &str, chars_to_find: &[char], reversed: bool) -> usize {
    let as_matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            if reversed {
                line.chars().rev().collect()
            } else {
                line.chars().collect()
            }
        })
        .collect();
    assert!(as_matrix.len() > 0);
    let n = chars_to_find.len();

    let rows = as_matrix.len();
    let cols = as_matrix[0].len();
    let mut count = 0;

    for sum in 0..(rows + cols - 1) {
        let mut forward = 0;
        let mut backward = 0;
        let row_start = sum.saturating_sub(cols - 1);
        let row_end = (rows - 1).min(sum);
        for row in row_start..row_end + 1 {
            let col = sum - row;
            let ch = as_matrix[row][col];

            if ch == chars_to_find[forward] {
                forward = (forward + 1) % n;
                if forward == 0 {
                    count += 1;
                }
            } else {
                forward = if ch == chars_to_find[0] { 1 } else { 0 };
            }

            if ch == chars_to_find[n - 1 - backward] {
                backward = (backward + 1) % n;
                if backward == 0 {
                    count += 1;
                }
            } else {
                backward = if ch == chars_to_find[n - 1] { 1 } else { 0 };
            }
        }
    }

    count
}

fn part1(input: &str) -> usize {
    let chars_to_find = &['X', 'M', 'A', 'S'];
    search_horizontally(input, chars_to_find)
        + search_vertically(input, chars_to_find)
        + search_diagonally(input, chars_to_find, true)
        + search_diagonally(input, chars_to_find, false)
}

fn part2(input: &str) -> usize {
    let as_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    assert!(as_matrix.len() > 0);

    let rows = as_matrix.len();
    let cols = as_matrix[0].len();

    let mut count = 0;
    for j in 1..cols - 1 {
        for i in 1..rows - 1 {
            let ch = as_matrix[i][j];
            if ch != 'A' {
                continue;
            }

            let right_diag = [as_matrix[i - 1][j - 1], ch, as_matrix[i + 1][j + 1]];
            let right_diag: String = right_diag.iter().collect();

            let left_diag =[as_matrix[i+1][j-1], ch, as_matrix[i-1][j+1]];
            let left_diag: String = left_diag.iter().collect();

            if (right_diag == "SAM" || right_diag == "MAS") && (left_diag == "SAM" || left_diag == "MAS") {
                count +=1;
            }
        }
    }

    count
}

#[cfg(test)]
mod day4_test {
    use crate::days::day4::{part1, search_diagonally, search_horizontally, search_vertically, part2};

    type Searcher = fn(&str, &[char]) -> usize;
    const CHARS_TO_FIND: &[char] = &['X', 'M', 'A', 'S'];
    const HORIZONTAL: Searcher = search_horizontally;
    const VERTICAL: Searcher = search_vertically;

    fn right_diag(input: &str, chars_to_find: &[char]) -> usize {
        search_diagonally(input, chars_to_find, false)
    }
    fn left_diag(input: &str, chars_to_find: &[char]) -> usize {
        search_diagonally(input, chars_to_find, true)
    }

    fn diag(input: &str, chars_to_find: &[char]) -> usize {
        right_diag(input, chars_to_find) + left_diag(input, chars_to_find)
    }

    const DIAGONAL: Searcher = diag;

    fn search(s: Searcher, input: &str) -> usize {
        s(input, CHARS_TO_FIND)
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(3, search(HORIZONTAL, "XMASXMASXXXSAMX"));
        assert_eq!(3, search(HORIZONTAL, "SAMXXMAXMASAMX"));
        assert_eq!(2, search(HORIZONTAL, "XAMXAMXAMSAMXSAMX"));
        assert_eq!(0, search(HORIZONTAL, ""));
        assert_eq!(0, search(HORIZONTAL, "XXXXXXXXXXXXXXXXXXAMX"));
        assert_eq!(2, search(HORIZONTAL, "XMASAMX"));
        assert_eq!(1, search(HORIZONTAL, "XXMAS"));
        assert_eq!(1, search(HORIZONTAL, "SSAMX"));
    }

    #[test]
    fn test_vertical() {
        assert_eq!(1, search(VERTICAL, "X\nM\nA\nS\n"));
        assert_eq!(2, search(VERTICAL, "XX\nMM\nAA\nSS\n"));
        assert_eq!(1, search(VERTICAL, "S\nA\nM\nX\n"));
        assert_eq!(2, search(VERTICAL, "SS\nAA\nMM\nXX\n"));
        assert_eq!(2, search(VERTICAL, "XS\nMA\nAM\nSX\n"));
        assert_eq!(1, search(VERTICAL, "XA\nMM\nAX\nSX\n"));
        assert_eq!(0, search(VERTICAL, "X\nM\nAS\n"));
        assert_eq!(0, search(VERTICAL, "XMAS\n"));
    }

    #[test]
    fn test_diagonal() {
        assert_eq!(1, search(DIAGONAL, "...X\n..M.\n.A..\nS..."));
        assert_eq!(1, search(DIAGONAL, "...S\n..A.\n.M..\nX..."));
        assert_eq!(2, search(DIAGONAL, "...SX\n..AM.\n.MA..\nXS..."));
        assert_eq!(1, search(DIAGONAL, "...X\n..M.\n.A..\nS..."));
        assert_eq!(1, search(DIAGONAL, "...S\n..A.\n.M..\nX..."));
        assert_eq!(2, search(DIAGONAL, "...SX\n..AM.\n.MA..\nXS..."));
        assert_eq!(1, search(DIAGONAL, "X....\n.M...\n..A..\n...S.\n"));
        assert_eq!(
            4,
            search(DIAGONAL, "XS......SX\n.MA....AM.\n..AM..MA..\n...SXXS...")
        );
    }

    #[test]
    fn test_example() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(18, part1(input));
    }

    #[test]
    fn test_xmas() {
        assert_eq!(1, part2("M.S\n.A.\nM.S"));
        assert_eq!(1, part2( "S.M\n.A.\nS.M"));
        assert_eq!(1, part2("M.M\n.A.\nS.S"));
        assert_eq!(1, part2("S.S\n.A.\nM.M"));
    }
}
