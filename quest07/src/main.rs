use itertools::Itertools;
use std::ops::{Add, AddAssign};
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    let track = input::read_file("inputs/part2_track.txt");
    println!("exercise 2: {}", part2(&input, &track));
    let input = input::read_file("inputs/part3.txt");
    let track = input::read_file("inputs/part3_track.txt");
    println!("exercise 3: {}", part3(&input, &track));
}

fn part1(input: &str) -> String {
    let mut plans: Vec<Plan> = input.lines().map(Plan::new).collect();

    plans.sort_by_cached_key(|plan| plan.value(10, 10));
    plans.iter().rev().map(|plan| plan.id).collect()
}

fn part2(input: &str, track: &str) -> String {
    let mut plans: Vec<Plan> = input.lines().map(Plan::new).collect();
    let track_actions: Vec<Action> = track::parse_track_actions(track);

    plans.sort_by_cached_key(|plan| plan.value_on_track(10, &track_actions, 10));
    plans.iter().rev().map(|plan| plan.id).collect()
}

fn part3(input: &str, track: &str) -> usize {
    let track_actions: Vec<Action> = track::parse_track_actions(track);
    let plan_to_beat = Plan::new(input);
    let score_to_beat = plan_to_beat.value_on_track(10, &track_actions, 2024);

    Plan::permutations(5, 3, 3)
        .filter(|plan| plan.value_on_track(10, &track_actions, 2024) > score_to_beat)
        .count()
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

    fn permutations(pluses: usize, minuses: usize, equals: usize) -> impl Iterator<Item = Self> {
        vec![Action::Plus; pluses]
            .into_iter()
            .chain(vec![Action::Minus; minuses].into_iter())
            .chain(vec![Action::Equal; equals].into_iter())
            .permutations(pluses + minuses + equals)
            .sorted_unstable()
            .dedup()
            .enumerate()
            .map(|(n, actions)| Self {
                actions,
                id: (b'A' + (n % 26) as u8) as char,
            })
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Plus,
    Minus,
    Equal,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        if value.len() != 1 {
            panic!("Invalid string length for Action")
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
            _ => panic!("Invalid symbol for Action"),
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

mod track {
    use crate::Action;
    use utils::{grid::Grid, grid::Position};

    pub fn parse_track_actions(track: &str) -> Vec<Action> {
        let mut track_actions = Vec::new();
        let track2d = Grid::from(track);
        let mut prev_pos = track2d
            .iter()
            .find_map(|(p, &c)| (c == 'S').then_some(p))
            .expect("No starting point 'S' found");
        let mut cur_pos = get_next_pos(&prev_pos, &prev_pos, &track2d);

        loop {
            track_actions.push(Action::from(*track2d.get(&cur_pos).unwrap()));
            (prev_pos, cur_pos) = (cur_pos, get_next_pos(&cur_pos, &prev_pos, &track2d));

            if *track2d.get(&prev_pos).unwrap() == 'S' {
                break;
            }
        }

        track_actions
    }

    fn get_next_pos(cur_pos: &Position, prev_pos: &Position, track2d: &Grid<char>) -> Position {
        let max_pos = Position::new(track2d.height - 1, track2d.width - 1);
        cur_pos
            .neighbors_contained(max_pos)
            .filter(|p| p != prev_pos)
            .filter(|p| track2d.get(p).map_or(false, |c| !c.is_whitespace()))
            .next()
            .unwrap()
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

    mod part3 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let track = input::read_file("inputs/part3_track.txt");
            let res = part3(&input, &track);
            assert_eq!(res, 5839);
        }
    }
}
