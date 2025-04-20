use std::ops::{Add, AddAssign};
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &str) -> String {
    let mut plans: Vec<Plan> = input.lines().map(Plan::new).collect();

    plans.sort_by_cached_key(|plan| plan.value(10, 10));
    plans.iter().rev().map(|plan| plan.id).collect()
}

#[derive(Clone, Copy)]
enum Action {
    Plus,
    Minus,
    Equal,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "+" => Action::Plus,
            "-" => Action::Minus,
            "=" => Action::Equal,
            _ => panic!("invalid symbol for Action"),
        }
    }
}

impl Add<Action> for usize {
    type Output = usize;

    fn add(self, rhs: Action) -> Self::Output {
        match rhs {
            Action::Plus => self + 1,
            Action::Minus => self.saturating_sub(1),
            Action::Equal => self,
        }
    }
}

impl AddAssign<Action> for usize {
    fn add_assign(&mut self, rhs: Action) {
        *self = *self + rhs;
    }
}

struct Plan {
    actions: Vec<Action>,
    id: char,
}

impl Plan {
    fn new(line: &str) -> Self {
        let (id, actions) = line.split_once(':').unwrap();
        let mut plan = Plan {
            actions: Vec::new(),
            id: id.chars().next().unwrap(),
        };

        for action in actions.split(',') {
            plan.actions.push(Action::from(action));
        }
        plan
    }

    fn value(&self, mut power: usize, length: usize) -> usize {
        let mut value = 0;

        for mut i in 0..length {
            i %= self.actions.len();
            power += self.actions[i];
            value += power;
        }
        value
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
            assert_eq!(res, "BDCA");
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, "BCGDKIHAE");
        }
    }
}
