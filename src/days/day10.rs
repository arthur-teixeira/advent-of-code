use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
    let graph = parse(input);
    println!("Part 1: {:?}", part1(&graph));
    println!("Part 2: {:?}", part2(&graph));
}

fn part1(graph: &Graph) -> usize {
    graph.heads.iter().fold(0, |acc, head| {
        let mut seen = HashSet::new();
        let head_points = count_1(&head.borrow(), &mut seen);
        acc + head_points
    })
}

fn part2(graph: &Graph) -> usize {
    graph.heads.iter().fold(0, |acc, head| {
        let head_points = count_2(&head.borrow());
        acc + head_points
    })
}

fn count_1(head: &Node, seen: &mut HashSet<usize>) -> usize {
    if head.level == 9 && !seen.contains(&head.id) {
        seen.insert(head.id);
        return 1;
    } else if head.neighbors.len() == 0 {
        return 0;
    }

    let mut cur = 0;
    for n in &head.neighbors {
        let n = n.borrow();
        cur += count_1(&n, seen);
    }

    cur
}

fn count_2(head: &Node) -> usize {
    if head.level == 9 {
        return 1;
    } else if head.neighbors.len() == 0 {
        return 0;
    }

    let mut cur = 0;
    for n in &head.neighbors {
        let n = n.borrow();
        cur += count_2(&n);
    }
    cur
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
