use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
    time::Instant,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    kind: char,
    id: usize,
    borders: usize,
    corners: usize,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

pub fn day12(input: String) {
    let mut seen = HashSet::new();
    let start = Instant::now();
    let graph = parse(input);
    println!("Parse time {:?}", start.elapsed());
    let solve_time = Instant::now();
    let (part1, part2) = graph
        .nodes
        .iter()
        .map(|node| bfs(Rc::clone(&node), &mut seen))
        .flatten()
        .fold((0, 0), |(p1, p2), (area, perim, sides)| {
            (p1 + area * perim, p2 + area * sides)
        });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Solved in {:?}", solve_time.elapsed());
}

fn compare_at(c: char, i: isize, j: isize, matrix: &Vec<Vec<Rc<RefCell<Node>>>>) -> bool {
    if !(i >= 0 && j >= 0) {
        return false;
    }

    matrix
        .get(i as usize)
        .and_then(|row| row.get(j as usize))
        .and_then(|n| Some(n.borrow().kind == c))
        .unwrap_or(false)
}

fn check_corners(j: usize, i: usize, matrix: &Vec<Vec<Rc<RefCell<Node>>>>) -> usize {
    let kind = matrix[i][j].borrow().kind;
    let i = i as isize;
    let j = j as isize;

    let up = compare_at(kind, i - 1, j, matrix);
    let down = compare_at(kind, i + 1, j, matrix);
    let up_left = compare_at(kind, i - 1, j - 1, matrix);
    let up_right = compare_at(kind, i - 1, j + 1, matrix);
    let down_right = compare_at(kind, i + 1, j + 1, matrix);
    let down_left = compare_at(kind, i + 1, j - 1, matrix);
    let right = compare_at(kind, i, j + 1, matrix);
    let left = compare_at(kind, i, j - 1, matrix);

    let result = !(up || right) as usize
        + !(up || left) as usize
        + !(down || right) as usize
        + !(down || left) as usize
        + (left && down && !down_left) as usize
        + (right && down && !down_right) as usize
        + (up && left && !up_left) as usize
        + (up && right && !up_right) as usize;

    result
}

fn parse(input: String) -> Graph {
    let mut id = 0;
    let node_matrix: Vec<Vec<Rc<RefCell<Node>>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let a = Rc::new(RefCell::new(Node {
                        kind: c,
                        id,
                        borders: 0,
                        corners: 0,
                        neighbors: vec![],
                    }));
                    id += 1;
                    a
                })
                .collect()
        })
        .collect();

    let mut graph = Graph { nodes: vec![] };
    let cols = node_matrix[0].len();
    let rows = node_matrix.len();

    for i in 0..rows {
        for j in 0..cols {
            let node = Rc::clone(&node_matrix[i][j]);
            node.borrow_mut().corners = check_corners(j, i, &node_matrix);
            if j > 0 {
                let left_node = Rc::clone(&node_matrix[i][j - 1]);
                node.borrow_mut().neighbors.push(left_node);
            } else {
                node.borrow_mut().borders += 1;
            }
            if j < cols - 1 {
                let right_node = Rc::clone(&node_matrix[i][j + 1]);
                node.borrow_mut().neighbors.push(right_node);
            } else {
                node.borrow_mut().borders += 1;
            }

            if i > 0 {
                let up_node = Rc::clone(&node_matrix[i - 1][j]);
                node.borrow_mut().neighbors.push(up_node);
            } else {
                node.borrow_mut().borders += 1;
            }

            if i < rows - 1 {
                let down_node = Rc::clone(&node_matrix[i + 1][j]);
                node.borrow_mut().neighbors.push(down_node);
            } else {
                node.borrow_mut().borders += 1;
            }
            graph.nodes.push(node);
        }
    }

    graph
}

fn bfs(head: Rc<RefCell<Node>>, seen: &mut HashSet<usize>) -> Option<(usize, usize, usize)> {
    let mut queue = VecDeque::new();
    let cur_kind = head.borrow().kind;

    if seen.contains(&head.borrow().id) {
        return None;
    }

    queue.push_back(head);

    let mut cur_area = HashSet::new();
    let mut sides = 0;
    let mut corners = 0;
    while queue.len() > 0 {
        let n = queue.pop_front().unwrap();
        let n = n.borrow();
        if seen.contains(&n.id) {
            continue;
        }

        if n.kind == cur_kind {
            sides += n.borders;
            cur_area.insert(n.id);
            seen.insert(n.id);
            corners += n.corners;

            for ne in &n.neighbors {
                if ne.borrow().kind != cur_kind {
                    sides += 1;
                }
                queue.push_back(Rc::clone(ne));
            }
        }
    }

    if cur_area.len() > 0 {
        Some((cur_area.len(), sides, corners))
    } else {
        None
    }
}
