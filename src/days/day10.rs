use std::{cell::RefCell, collections::HashSet, rc::Rc, time::Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    level: u8,
    id: usize,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

struct Graph {
    heads: Vec<Rc<RefCell<Node>>>,
}

pub fn day10(input: String) {
    let start = Instant::now();
    let graph = parse(input);
    let parse_time = start.elapsed();
    let solve_timer = Instant::now();
    let (p1, p2) = solve(&graph);
    let solve_time = solve_timer.elapsed();
    println!("Part 1: {:?}\nPart 2: {:?}", p1, p2);

    println!(
        "Performance:\nParsing: {:?}\nPart 1 and 2: {:?}\nTotal:{:?}",
        parse_time,
        solve_time,
        parse_time + solve_time,
    );
}

fn solve(graph: &Graph) -> (usize, usize) {
    graph.heads.iter().fold((0, 0), |(acc1, acc2), head| {
        let mut seen = HashSet::new();
        let (p1, p2) = dfs(&head.borrow(), &mut seen);
        (acc1 + p1, acc2 + p2)
    })
}

fn dfs(head: &Node, seen: &mut HashSet<usize>) -> (usize, usize) {
    if head.level == 9 {
        if !seen.contains(&head.id) {
            seen.insert(head.id);
            return (1, 1);
        } else {
            return (0, 1);
        }
    } else if head.neighbors.len() == 0 {
        return (0, 0);
    }

    head.neighbors.iter().fold((0, 0), |(acc1, acc2), cur| {
        let (p1, p2) = dfs(&cur.borrow(), seen);
        (acc1 + p1, acc2 + p2)
    })
}

fn parse(input: String) -> Graph {
    let mut id = 0;
    let node_matrix: Vec<Vec<Rc<RefCell<Node>>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let a = Rc::new(RefCell::new(Node {
                        level: c as u8 - b'0',
                        id,
                        neighbors: vec![],
                    }));
                    id += 1;
                    a
                })
                .collect()
        })
        .collect();

    let mut graph = Graph { heads: vec![] };
    let cols = node_matrix[0].len();
    let rows = node_matrix.len();

    for i in 0..rows {
        for j in 0..cols {
            let node = Rc::clone(&node_matrix[i][j]);
            if j > 0 {
                let left_node = Rc::clone(&node_matrix[i][j - 1]);
                if left_node.borrow().level == node.borrow().level + 1 {
                    node.borrow_mut().neighbors.push(left_node);
                }
            }
            if j < cols - 1 {
                let right_node = Rc::clone(&node_matrix[i][j + 1]);
                if right_node.borrow().level == node.borrow().level + 1 {
                    node.borrow_mut().neighbors.push(right_node);
                }
            }

            if i > 0 {
                let up_node = Rc::clone(&node_matrix[i - 1][j]);
                if up_node.borrow().level == node.borrow().level + 1 {
                    node.borrow_mut().neighbors.push(up_node);
                }
            }

            if i < rows - 1 {
                let down_node = Rc::clone(&node_matrix[i + 1][j]);
                if down_node.borrow().level == node.borrow().level + 1 {
                    node.borrow_mut().neighbors.push(down_node);
                }
            }
            if node.borrow().level == 0 {
                graph.heads.push(node);
            }
        }
    }

    graph
}
