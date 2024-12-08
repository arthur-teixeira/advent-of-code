use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Location = (isize, isize);
#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    map: Vec<Vec<char>>,
    unique_frequencies: HashSet<char>,
    frequencies_locations: HashMap<char, Vec<Location>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = map.len();
        let cols = map[0].len();

        let mut unique_frequencies = HashSet::new();
        let mut frequencies_locations: HashMap<char, Vec<Location>> = HashMap::new();
        for i in 0..rows {
            for j in 0..cols {
                let c = map[i][j];
                if c != '.' {
                    unique_frequencies.insert(c);
                    frequencies_locations
                        .entry(c)
                        .and_modify(|v| v.push((i as isize, j as isize)))
                        .or_insert(vec![(i as isize, j as isize)]);
                }
            }
        }

        Self {
            rows,
            cols,
            unique_frequencies,
            map,
            frequencies_locations,
        }
    }

    fn add_antinode(&self, antinodes: &mut HashSet<Location>, a: Location, b: Location, loc: Location) {
        let (i, j) = loc;
        if i >= 0 && i < self.rows as isize && j >= 0 && j < self.cols as isize {
            if loc != a && loc != b {
                antinodes.insert(loc);
            }
        }
    }

    fn get_antinodes(&self) -> usize {
        let mut antinodes: HashSet<Location> = HashSet::new();
        for locs in self.frequencies_locations.values() {
            for c in locs.iter().combinations(2) {
                let (x1, y1) = c[0];
                let (x2, y2) = c[1];

                let diff = (*x1 as isize - *x2 as isize, *y1 as isize - *y2 as isize);
                let (x_diff, y_diff) = diff;
                let a1 = (x1 + x_diff, y1 + y_diff);
                let a2 = (x1 - x_diff, y1 - y_diff);
                let b1 = (x2 + x_diff, y2 + y_diff);
                let b2 = (x2 - x_diff, y2 - y_diff);

                self.add_antinode(&mut antinodes, *c[0], *c[1], a1);
                self.add_antinode(&mut antinodes, *c[0], *c[1], a2);
                self.add_antinode(&mut antinodes, *c[0], *c[1], b1);
                self.add_antinode(&mut antinodes, *c[0], *c[1], b2);
            }
        }

        antinodes.len()
    }
}
pub fn day8(input: String) {
    let map = Map::new(&input);
    println!("Part 1:{:?}", map.get_antinodes());
}
