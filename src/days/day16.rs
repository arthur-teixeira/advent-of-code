use std::{
    cell::RefCell,
    cmp::Reverse,
    hash::{Hash, Hasher},
    rc::Rc,
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug, PartialEq, Eq)]
enum NodeType {
    Wall,
    Start,
    End,
    Free,
}

#[derive(Clone, Debug, Eq)]
struct NodeT(Rc<RefCell<Node>>);
impl NodeT {
    fn new(n: Node) -> Self {
        Self(Rc::new(RefCell::new(n)))
    }
}

impl Hash for NodeT {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.borrow().id);
        state.finish();
    }
}

impl PartialEq for NodeT {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().id == other.0.borrow().id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    kind: NodeType,
    id: usize,
    neighbors: Vec<(NodeT, Direction)>,
    prev: Option<(NodeT, Direction)>,
}

struct Graph {
    head: NodeT,
    end: NodeT,
    vertices: Vec<NodeT>,
    rows: usize,
    cols: usize,
}

impl Graph {
    fn draw_path(&self, path_ids: &[(usize, Direction)]) {
        let mut cur_id = 0;
        let ids = path_ids.iter().map(|(i, _)| *i).collect_vec();
        let dirs = path_ids.iter().map(|(_, i)| *i).collect_vec();

        for _ in 0..self.rows {
            for _ in 0..self.cols {
                if ids.contains(&cur_id) {
                    let p = ids.iter().position(|i| *i == cur_id).unwrap();
                    let c = match dirs[p] {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };

                    print!("{c}");
                } else {
                    print!(".");
                }

                cur_id += 1;
            }
            println!();
        }
    }
}

fn parse(input: &str) -> Graph {
    let mut id = 0;
    let node_matrix: Vec<Vec<NodeT>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let a = NodeT::new(Node {
                        kind: match c {
                            '.' => NodeType::Free,
                            'S' => NodeType::Start,
                            'E' => NodeType::End,
                            '#' => NodeType::Wall,
                            _ => unreachable!(),
                        },
                        id,
                        neighbors: vec![],
                        prev: None,
                    });
                    id += 1;
                    a
                })
                .collect()
        })
        .collect();

    let cols = node_matrix[0].len();
    let rows = node_matrix.len();
    let mut head: Option<NodeT> = None;
    let mut end: Option<NodeT> = None;

    for i in 0..rows {
        for j in 0..cols {
            let node = &node_matrix[i][j];
            if j > 0 {
                let left_node = node_matrix[i][j - 1].clone();
                if left_node.0.borrow().kind != NodeType::Wall {
                    node.0
                        .borrow_mut()
                        .neighbors
                        .push((left_node, Direction::Left));
                }
            }
            if j < cols - 1 {
                let right_node = node_matrix[i][j + 1].clone();
                if right_node.0.borrow().kind != NodeType::Wall {
                    node.0
                        .borrow_mut()
                        .neighbors
                        .push((right_node, Direction::Right));
                }
            }

            if i > 0 {
                let up_node = node_matrix[i - 1][j].clone();
                if up_node.0.borrow().kind != NodeType::Wall {
                    node.0.borrow_mut().neighbors.push((up_node, Direction::Up));
                }
            }

            if i < rows - 1 {
                let down_node = node_matrix[i + 1][j].clone();
                if down_node.0.borrow().kind != NodeType::Wall {
                    node.0
                        .borrow_mut()
                        .neighbors
                        .push((down_node, Direction::Down));
                }
            }
            if node.0.borrow().kind == NodeType::Start {
                head = Some(node.clone());
            } else if node.0.borrow().kind == NodeType::End {
                end = Some(node.clone());
            }
        }
    }

    Graph {
        head: head.expect("Expected start to be in the input"),
        end: end.expect("Expected end to be in the input"),
        vertices: node_matrix.into_iter().flatten().collect_vec(),
        rows,
        cols,
    }
}

fn shortest_distance(g: &Graph) -> usize {
    let mut distances = vec![0; 10000000];
    let mut queue: PriorityQueue<NodeT, Reverse<usize>> = PriorityQueue::new();
    let hid = g.head.0.borrow().id;
    distances[hid] = 0;
    queue.push(g.head.clone(), Reverse(0));

    for v in &g.vertices {
        let id = v.0.borrow().id;
        if v.0.borrow().id != hid {
            distances[id] = usize::MAX - 1001;
            queue.push(v.clone(), Reverse(usize::MAX - 1001));
        }
    }

    while !queue.is_empty() {
        let (u, _) = queue.pop().unwrap();

        for (v, dir) in &u.0.borrow().neighbors {
            let dir_to_reach_u = match u.0.borrow().prev {
                None => {
                    if u.0.borrow().kind == NodeType::Start {
                        Direction::Right
                    } else {
                        *dir
                    }
                }
                Some(ref p) => p.1,
            };
            let cost = if *dir != dir_to_reach_u { 1001 } else { 1 };

            let alt = distances[u.0.borrow().id].saturating_add(cost);
            if alt < distances[v.0.borrow().id] {
                v.0.borrow_mut().prev = Some((NodeT(Rc::clone(&u.0)), *dir));
                distances[v.0.borrow().id] = alt;
                queue.change_priority(&v, Reverse(alt));
            }
        }
    }

    distances[g.end.0.borrow().id]
}

fn build_path(g: &NodeT, acc: &mut Vec<(usize, Direction)>) {
    if let Some((p, dir)) = &g.0.borrow().prev {
        build_path(&p, acc);
        acc.push((p.0.borrow().id, *dir));
    }
}

pub fn day16(input: String) {
    let graph = parse(&input);
    println!("{:?}", shortest_distance(&graph));
    let mut result = Vec::new();
    build_path(&graph.end, &mut result);
    graph.draw_path(&result);
}
