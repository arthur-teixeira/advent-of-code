use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashSet};

use itertools::{Itertools, Position};

type NodeT = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: String,
    non_leaf_included: bool,
    children: HashMap<char, NodeT>,
}

impl Node {
    fn new(value: &str, non_leaf_included: bool) -> NodeT {
        Rc::new(RefCell::new(Node {
            value: value.to_string(),
            non_leaf_included,
            children: HashMap::new(),
        }))
    }
}

#[derive(Debug)]
struct Trie {
    head: NodeT,
}

impl Trie {
    fn new() -> Self {
        Self {
            head: Node::new("", false),
        }
    }

    fn insert(&mut self, pattern: &str) {
        let mut cur_node = Rc::clone(&self.head);
        for (i, c) in pattern.chars().enumerate() {
            let mut cn = cur_node.borrow_mut();
            let new_node = match cn.children.get(&c) {
                Some(child) => {
                    if i == pattern.len() - 1 {
                        child.borrow_mut().non_leaf_included = true;
                    }
                    Rc::clone(child)
                }
                None => {
                    let new_node = Node::new(&pattern[..=i], i == pattern.len() - 1);
                    cn.children.insert(c, Rc::clone(&new_node));
                    new_node
                }
            };
            drop(cn);
            cur_node = new_node;
        }
    }

    fn contains(&self, pattern: &str) -> (bool, bool) {
        let mut cur_node = Rc::clone(&self.head);
        for c in pattern.chars() {
            let cn = cur_node.borrow();
            let new_node = match cn.children.get(&c) {
                Some(child) => Rc::clone(child),
                None => return (false, false),
            };
            drop(cn);
            cur_node = new_node;
        }

        let cn = cur_node.borrow();
        (
            cn.children.len() == 0 || cn.non_leaf_included,
            cn.children.len() > 0,
        )
    }

    fn contains_subpatterns<'a>(&self, p: &'a str) -> bool {
        let mut stack = Vec::new();
        stack.push(p);

        let mut ways = 0;
        while let Some(p) = stack.pop() {
            for i in 1..=p.len() {
                let pp = &p[..i];
                let rem = &p[i..];
                let (contained, is_prefix) = self.contains(pp);
                if !contained && !is_prefix {
                    break;
                }
                if contained {
                    if rem.is_empty() {
                        return true;
                    }
                    stack.push(rem);
                }
                if !is_prefix {
                    break;
                }
            }
        }
        false
    }
}

fn test<'a>(patterns: &[&str], p: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    if p.is_empty() {
        return 1;
    }

    if let Some(&c) = cache.get(p) {
        return c;
    }

    let mut count = 0;
    for pattern in patterns {
        if p.starts_with(pattern) {
            count += test(patterns, &p[pattern.len()..], cache);
        }
    }

    cache.insert(p, count);
    return count;
}

pub fn day19(input: String) {
    let (trie, patterns, possibilities) = parse(&input);
    println!("Part 1: {}", part1(&trie, &possibilities));
    println!("Part 2: {}", part2(&patterns, &possibilities));
}

fn parse(input: &str) -> (Trie, Vec<&str>, Vec<&str>) {
    let mut trie = Trie::new();
    let mut parts = input.split("\n\n");

    let f = parts.next().unwrap();
    let patterns = f.split(", ").collect_vec();
    for pattern in &patterns {
        trie.insert(pattern);
    }

    let possibilities = parts.next().unwrap().lines().collect_vec();

    (trie, patterns, possibilities)
}

fn part1(trie: &Trie, patterns: &[&str]) -> usize {
    patterns
        .iter()
        .filter(|p| trie.contains_subpatterns(p))
        .count()
}

fn part2(prefixes: &[&str], patterns: &[&str]) -> usize {
    let mut cache = Default::default();
    patterns.iter().fold(0, |acc, p| {
        acc + test(prefixes, p, &mut cache)
    })
}
