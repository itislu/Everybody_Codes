use std::ops::{Add, AddAssign};
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    let track = input::read_file("inputs/part2_track.txt");
    println!("exercise 2: {}", part2(&input, &track));
}

fn part1(input: &str) -> String {
    let mut plans: Vec<Plan> = input.lines().map(Plan::new).collect();

    plans.sort_by_cached_key(|plan| plan.value(10, 10));
    plans.iter().rev().map(|plan| plan.id).collect()
}

fn part2(input: &str, track: &str) -> String {
    let mut plans: Vec<Plan> = input.lines().map(Plan::new).collect();
    let track_actions: Vec<Action> = parse_track_actions(track);

    plans.sort_by_cached_key(|plan| plan.value_on_track(10, &track_actions, 10));
    plans.iter().rev().map(|plan| plan.id).collect()
}

// Assumes 'S' is at (0, 0)
fn parse_track_actions(track: &str) -> Vec<Action> {
    let mut track_actions = Vec::new();
    let track_2d: Vec<Vec<char>> = track.lines().map(|line| line.chars().collect()).collect();
    let (width, height) = (track_2d[0].len(), track_2d.len());

    for col in 1..width {
        track_actions.push(Action::from(track_2d[0][col]));
    }
    for row in 1..height {
        track_actions.push(Action::from(track_2d[row][width - 1]));
    }
    for col in (0..width - 1).rev() {
        track_actions.push(Action::from(track_2d[height - 1][col]));
    }
    for row in (0..height - 1).rev() {
        track_actions.push(Action::from(track_2d[row][0]));
    }
    track_actions
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Plus,
    Minus,
    Equal,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        if value.len() != 1 {
            panic!("invalid string length for Action")
        }
        Action::from(value.chars().next().unwrap())
    }
}

impl From<char> for Action {
    fn from(value: char) -> Self {
        match value {
            '+' => Action::Plus,
            '-' => Action::Minus,
            '=' | 'S' => Action::Equal,
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

#[derive(Debug)]
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

    fn value_on_track(&self, mut power: usize, track_actions: &[Action], loops: usize) -> usize {
        let mut value = 0;

        for (i, track_action) in track_actions
            .iter()
            .cycle()
            .take(track_actions.len() * loops)
            .enumerate()
        {
            power += if matches!(track_action, Action::Equal) {
                self.actions[i % self.actions.len()]
            } else {
                *track_action
            };
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
            let input = input::read_file("inputs/part1and2_example.txt");
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

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part1and2_example.txt");
            let track = input::read_file("inputs/part2_track_example.txt");
            let res = part2(&input, &track);
            assert_eq!(res, "DCBA");
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let track = input::read_file("inputs/part2_track.txt");
            let res = part2(&input, &track);
            assert_eq!(res, "FAIKHBEJG");
        }
    }
}
