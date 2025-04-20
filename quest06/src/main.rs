use std::collections::HashMap;
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2and3(&input));
    let input = input::read_file("inputs/part3.txt");
    println!("exercise 3: {}", part2and3(&input));
}

fn part1(input: &String) -> String {
    let tree = Tree::new(input);
    path_to_string(&find_unique_path(&tree))
}

fn part2and3(input: &String) -> String {
    let tree = Tree::new(input);
    path_to_string_short(&find_unique_path(&tree))
}

fn find_unique_path(tree: &Tree) -> Vec<&Node> {
    let mut paths: HashMap</*length*/ usize, /*path*/ Option<Vec<&Node>>> = HashMap::new();

    for fruit in tree.fruits() {
        let path: Vec<&Node> = tree.path(fruit);
        paths
            .entry(path.len())
            .and_modify(|e| *e = None)
            .or_insert(Some(path));
    }
    paths
        .drain()
        .filter_map(|(_, v)| v)
        .next()
        .expect("No unique path found")
}

fn path_to_string(path: &Vec<&Node>) -> String {
    path.iter().rev().map(|node| node.to_string()).collect()
}

fn path_to_string_short(path: &Vec<&Node>) -> String {
    path.iter()
        .rev()
        .map(|node| node.to_string().chars().next().unwrap())
        .collect()
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
}

impl Tree {
    fn new(input: &String) -> Self {
        let mut map = HashMap::new();
        let mut fruit_count = 0;

        for line in input.lines() {
            let (parent, children) = line.split_once(':').unwrap();

            if parent == "BUG" || parent == "ANT" {
                continue;
            }
            for child in children.split(',') {
                let node: Node = match child {
                    "BUG" | "ANT" => continue,
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
        Tree { map }
    }

    fn fruits(&self) -> impl Iterator<Item = &Node> {
        self.map
            .keys()
            .filter(|node| matches!(**node, Node::Fruit(_)))
    }

    fn path<'a>(&'a self, mut node: &'a Node) -> Vec<&'a Node> {
        let mut path: Vec<&'a Node> = vec![&node];

        while let Some(parent) = self.map.get(&node) {
            node = parent;
            path.push(parent);
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

    mod part2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part2and3(&input);
            assert_eq!(res, "RFBMNWSHLW@");
        }
    }

    mod part3 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part2and3(&input);
            assert_eq!(res, "RPPLHWXLKSTB@");
        }
    }
}
