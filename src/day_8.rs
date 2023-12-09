use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use rayon::prelude::*;

use crate::common::Lexer;

pub fn puzzle_1(source: &str) -> usize {
    let mut lexer = Lexer::new(source);

    let steps = parse_steps(&mut lexer);

    let graph = parse_nodes(&mut lexer);

    let mut current = graph.id_by_name(['A', 'A', 'A']).unwrap();

    let mut steps = steps.into_iter().cycle();

    let mut count = 0;

    loop {
        let node = &graph[current];

        let step = steps.next().unwrap();

        let next = match step {
            Step::Left => node.left,
            Step::Right => node.right,
        };

        count += 1;

        if *graph.name_by_id(next) == ['Z', 'Z', 'Z'] {
            break;
        }

        current = next;
    }

    count
}

pub fn puzzle_2(source: &str) -> usize {
    let mut lexer = Lexer::new(source);

    let steps = parse_steps(&mut lexer);

    let graph = parse_nodes(&mut lexer);

    let starting_nodes = graph
        .node_names
        .iter()
        .filter_map(|(k, v)| (k[2] == 'A').then_some(*v))
        .collect::<Vec<_>>();

    starting_nodes
        .par_iter()
        .map(|start| {
            let mut steps = steps.iter().copied().cycle();

            let mut count = 0;

            let mut current = *start;

            loop {
                let node = &graph[current];

                let step = steps.next().unwrap();

                let next = match step {
                    Step::Left => node.left,
                    Step::Right => node.right,
                };

                count += 1;

                if {
                    let name = graph.name_by_id(next);
                    name[2] == 'Z'
                } {
                    break;
                }

                current = next;
            }

            count
        })
        .reduce(|| 1, lcd)
}

fn lcd(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        return 0;
    }

    a * (b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b)
    }

    a
}

fn parse_steps(lexer: &mut Lexer) -> Vec<Step> {
    let mut steps = Vec::new();

    while let Some(c) = lexer.peek() {
        match c {
            'L' => {
                steps.push(Step::Left);
                lexer.advance();
            }
            'R' => {
                steps.push(Step::Right);
                lexer.advance();
            }
            _ => break,
        }
    }
    lexer.skip_whitespace();

    steps
}

fn parse_nodes(lexer: &mut Lexer) -> Graph {
    let mut result = Graph::new();

    let mut parsed_nodes = Vec::new();

    while let Some((name, left, right)) = parse_node(lexer) {
        let id = result.add_node(name);
        parsed_nodes.push((id, left, right));
        lexer.skip_whitespace();
    }

    for (id, left, right) in parsed_nodes {
        let left = result.id_by_name(left).unwrap();
        let right = result.id_by_name(right).unwrap();
        let node = &mut result[id];

        node.left = left;
        node.right = right;
    }

    result
}

fn parse_node(lexer: &mut Lexer) -> Option<([char; 3], [char; 3], [char; 3])> {
    let name = parse_node_name(lexer)?;
    lexer.skip_whitespace();

    lexer.advance_matches("=")?;
    lexer.skip_whitespace();

    lexer.advance_matches("(")?;
    lexer.skip_whitespace();

    let left = parse_node_name(lexer)?;
    lexer.skip_whitespace();

    lexer.advance_matches(",")?;
    lexer.skip_whitespace();

    let right = parse_node_name(lexer)?;
    lexer.skip_whitespace();

    lexer.advance_matches(")")?;
    lexer.skip_whitespace();

    Some((name, left, right))
}

fn parse_node_name(lexer: &mut Lexer) -> Option<[char; 3]> {
    let mut result = ['\u{0}'; 3];

    for c in result.iter_mut() {
        let p = lexer.peek()?;
        if p.is_alphanumeric() {
            lexer.advance();
            *c = p;
        } else {
            return None;
        }
    }

    Some(result)
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Left,
    Right,
}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeId(usize);

#[derive(Clone, Copy, Default, Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

struct Graph {
    node_names: HashMap<[char; 3], NodeId>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            node_names: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    pub fn id_by_name(&self, name: [char; 3]) -> Option<NodeId> {
        self.node_names.get(&name).copied()
    }

    pub fn name_by_id(&self, id: NodeId) -> &[char; 3] {
        for (k, v) in self.node_names.iter() {
            if *v == id {
                return k;
            }
        }

        unreachable!()
    }

    pub fn add_node(&mut self, name: [char; 3]) -> NodeId {
        let id = NodeId(self.nodes.len());

        self.nodes.push(Node::default());

        if let Some(v) = self.node_names.insert(name, id) {
            panic!("Node with name {name:?} already exists; has id {v:?}")
        }

        id
    }
}

impl Index<NodeId> for Graph {
    type Output = Node;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl IndexMut<NodeId> for Graph {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index.0]
    }
}

impl Index<[char; 3]> for Graph {
    type Output = Node;

    fn index(&self, index: [char; 3]) -> &Self::Output {
        let index = self.id_by_name(index).unwrap();
        &self[index]
    }
}

impl IndexMut<[char; 3]> for Graph {
    fn index_mut(&mut self, index: [char; 3]) -> &mut Self::Output {
        let index = self.id_by_name(index).unwrap();
        &mut self[index]
    }
}
