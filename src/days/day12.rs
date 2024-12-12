use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    kind: char,
    id: usize,
    borders: usize,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

pub fn day12(input: String) {
    let graph = parse(input);
    println!("Part 1: {}", part1(&graph));
}
fn part1(graph: &Graph) -> usize {
    let mut seen = HashSet::new();
    graph
        .nodes
        .iter()
        .map(|node| bfs(Rc::clone(&node), &mut seen))
        .flatten()
        .fold(0, |acc, (area, perim)| acc + area * perim)
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

    for j in 0..cols {
        for i in 0..rows {
            let node = Rc::clone(&node_matrix[i][j]);
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

fn bfs(head: Rc<RefCell<Node>>, seen: &mut HashSet<usize>) -> Option<(usize, usize)> {
    let mut queue = VecDeque::new();
    let cur_kind = head.borrow().kind;

    if seen.contains(&head.borrow().id) {
        return None;
    }

    queue.push_back(head);

    let mut cur_area = HashSet::new();
    let mut sides = 0;
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

            for ne in &n.neighbors {
                if ne.borrow().kind != cur_kind {
                    sides += 1;
                }
                queue.push_back(Rc::clone(ne));
            }
        }
    }

    if cur_area.len() > 0 {
        Some((cur_area.len(), sides))
    } else {
        None
    }
}
