use std::collections::HashMap;
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &String) -> String {
    let tree = Tree::new(input);
    let mut paths: HashMap</*length*/ usize, /*path*/ Option<Vec<Node>>> = HashMap::new();

    for fruit in tree.fruits() {
        let path: Vec<Node> = tree.path(fruit);
        paths
            .entry(path.len())
            .and_modify(|e| *e = None)
            .or_insert(Some(path));
    }

    path_to_string(paths.values().filter_map(|v| v.as_ref()).next().unwrap())
}

fn path_to_string(path: &Vec<Node>) -> String {
    path.iter().rev().map(|node| node.to_string()).collect()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node {
    Branch(String),
    Fruit(usize),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Branch(s) => write!(f, "{}", s),
            Node::Fruit(_) => write!(f, "@"),
        }
    }
}

struct Tree {
    map: HashMap<Node, /*parent*/ Node>,
    fruit_count: usize,
}

impl Tree {
    fn new(input: &String) -> Self {
        let mut map = HashMap::new();
        let mut fruit_count = 0;

        for line in input.lines() {
            let (parent, children) = line.split_once(':').unwrap();
            for child in children.split(',') {
                let node: Node = match child {
                    "@" => {
                        let fruit_id = fruit_count;
                        fruit_count += 1;
                        Node::Fruit(fruit_id)
                    }
                    _ => Node::Branch(child.to_owned()),
                };
                map.insert(node, Node::Branch(parent.to_owned()));
            }
        }
        Tree { map, fruit_count }
    }

    fn fruits(&self) -> impl Iterator<Item = Node> {
        (0..self.fruit_count).map(|fruit_id| Node::Fruit(fruit_id))
    }

    fn path(&self, mut node: Node) -> Vec<Node> {
        let mut path: Vec<Node> = vec![node.clone()];

        while let Some(parent) = self.map.get(&node) {
            path.push(parent.clone());
            node = parent.clone();
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part1_example.txt");
            let res = part1(&input);
            assert_eq!(res, "RRB@");
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, "RRBSDGVPJHKG@");
        }
    }
}
