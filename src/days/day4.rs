
pub fn day4(input: String) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("Part 1: {}", part1(&matrix));
    println!("Part 2: {}", part2(&matrix));
}

fn part2(matrix: &[Vec<char>]) -> usize {
    assert!(matrix.len() > 0);

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;
    for j in 1..cols - 1 {
        for i in 1..rows - 1 {
            let ch = matrix[i][j];
            if ch != 'A' {
                continue;
            }

            let right_diag = [matrix[i - 1][j - 1], ch, matrix[i + 1][j + 1]];
            let right_diag: String = right_diag.iter().collect();

            let left_diag = [matrix[i + 1][j - 1], ch, matrix[i - 1][j + 1]];
            let left_diag: String = left_diag.iter().collect();

            if (right_diag == "SAM" || right_diag == "MAS")
                && (left_diag == "SAM" || left_diag == "MAS")
            {
                count += 1;
            }
        }
    }

    count
}

fn check_char_at(matrix: &[Vec<char>], expected: char, i: isize, j: isize) -> bool {
    match matrix.get(i as usize) {
        Some(row) => match row.get(j as usize) {
            Some(c) => *c == expected,
            None => false,
        },
        None => false,
    }
}

fn part1(matrix: &[Vec<char>]) -> usize {
    let rows = matrix.len();
    assert!(matrix.len() > 0);
    let cols = matrix[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), // up left Diagonal
        (1, 1),   // down right Diagonal
        (0, 1),   // Right
        (0, -1),  // Left
        (1, -1),  // up right diagonal
        (-1, 1),  // down left diagonal
        (1, 0),   //up
        (-1, 0),  // down
    ];

    let mut count = 0;
    for j in 0..cols {
        for i in 0..rows {
            if matrix[i][j] == 'X' {
                for (mod_i, mod_j) in &directions {
                    if check_char_at(&matrix, 'M', i as isize + mod_i * 1, j as isize + mod_j * 1)
                        && check_char_at(
                            &matrix,
                            'A',
                            i as isize + mod_i * 2,
                            j as isize + mod_j * 2,
                        )
                        && check_char_at(
                            &matrix,
                            'S',
                            i as isize + mod_i * 3,
                            j as isize + mod_j * 3,
                        )
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod day4_test {
    use crate::days::day4::{part1 as p1, part2 as p2};

    fn part1(input: &str) -> usize {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        p1(&matrix)
    }
    fn part2(input: &str) -> usize {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        p2(&matrix)
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(3, part1("XMASXMASXXXSAMX"));
        assert_eq!(3, part1("SAMXXMAXMASAMX"));
        assert_eq!(2, part1("XAMXAMXAMSAMXSAMX"));
        assert_eq!(0, part1("XXXXXXXXXXXXXXXXXXAMX"));
        assert_eq!(2, part1("XMASAMX"));
        assert_eq!(1, part1("XXMAS"));
        assert_eq!(1, part1("SSAMX"));
    }

    #[test]
    fn test_vertical() {
        assert_eq!(1, part1("X\nM\nA\nS\n"));
        assert_eq!(2, part1("XX\nMM\nAA\nSS\n"));
        assert_eq!(1, part1("S\nA\nM\nX\n"));
        assert_eq!(2, part1("SS\nAA\nMM\nXX\n"));
        assert_eq!(2, part1("XS\nMA\nAM\nSX\n"));
        assert_eq!(1, part1("XA\nMM\nAX\nSX\n"));
        assert_eq!(0, part1("X\nM\nAS\n"));
        assert_eq!(1, part1("XMAS\n"));
    }

    #[test]
    fn test_diagonal() {
        assert_eq!(1, part1("...X\n..M.\n.A..\nS..."));
        assert_eq!(1, part1("...S\n..A.\n.M..\nX..."));
        assert_eq!(2, part1("...SX\n..AM.\n.MA..\nXS..."));
        assert_eq!(1, part1("...X\n..M.\n.A..\nS..."));
        assert_eq!(1, part1("...S\n..A.\n.M..\nX..."));
        assert_eq!(2, part1("...SX\n..AM.\n.MA..\nXS..."));
        assert_eq!(1, part1("X....\n.M...\n..A..\n...S.\n"));
        assert_eq!(4, part1("XS......SX\n.MA....AM.\n..AM..MA..\n...SXXS..."));
    }

    #[test]
    fn test_example() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(18, part1(input));
    }

    #[test]
    fn test_xmas() {
        assert_eq!(1, part2("M.S\n.A.\nM.S"));
        assert_eq!(1, part2("S.M\n.A.\nS.M"));
        assert_eq!(1, part2("M.M\n.A.\nS.S"));
        assert_eq!(1, part2("S.S\n.A.\nM.M"));
    }
}
