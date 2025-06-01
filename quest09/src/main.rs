use utils::input;
use utils::parse;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input));
}

// Greedy
fn part1(input: &str) -> usize {
    let targets: Vec<usize> = parse::numbers(input).collect();
    let mut stamps = vec![1, 3, 5, 10];
    stamps.sort_by(|a, b| b.cmp(a));
    let stamps = stamps; // make immutable
    let mut total = 0;

    for mut remaining in targets {
        for stamp in &stamps {
            total += remaining / stamp;
            remaining %= stamp;
        }
    }
    total
}

/**
Dynamic programming

- Subproblems: DP(t) = min amount of stamps to sum to target t for t = 0,1,...,T
- Relation: DP(t) = min{1 + DP(t - s) | 1 <= s <= t}
- Topological Order: increasing t
- Base Case: DP(0) = 0
- Original Problem: DP(T)
- Time: O(T) subproblems * O(S) times = O(T * S)
*/
fn part2(input: &str) -> usize {
    let targets: Vec<usize> = parse::numbers(input).collect();
    let stamps = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    let mut total = 0;
    let mut solutions: Vec<Option<usize>> = vec![None; *targets.iter().max().unwrap() + 1];

    for target in targets {
        total += min_stamps(&stamps, target, &mut solutions)
            .expect(format!("no solution for {target}").as_str());
    }
    total
}

fn min_stamps(stamps: &[usize], target: usize, solutions: &mut [Option<usize>]) -> Option<usize> {
    if solutions[target].is_some() {
        return solutions[target];
    }
    solutions[target] = if target == 0 {
        Some(0)
    } else {
        stamps
            .iter()
            .filter(|&&stamp| stamp <= target)
            .filter_map(|stamp| min_stamps(stamps, target - stamp, solutions).map(|sol| 1 + sol))
            .min()
    };
    solutions[target]
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
            assert_eq!(res, 10);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, 12218);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part2_example.txt");
            let res = part2(&input);
            assert_eq!(res, 10);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part2(&input);
            assert_eq!(res, 5057);
        }
    }
}
