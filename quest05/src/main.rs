use std::collections::{HashMap, HashSet, VecDeque};
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input));
    let input = input::read_file("inputs/part3.txt");
    println!("exercise 3: {}", part3(&input));
}

fn part1(input: &String) -> String {
    let mut columns = parse_columns(input);

    for round in 1..=10 {
        do_round(&mut columns, round);
    }
    columns.iter().map(|col| col[0].to_string()).collect()
}

fn part2(input: &String) -> usize {
    let mut columns = parse_columns(input);
    let mut round_results: HashMap<usize, usize> = HashMap::new();
    let mut round = 1;

    loop {
        let round_result = do_round(&mut columns, round);
        let new_count = *round_results
            .entry(round_result)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if new_count == 2024 {
            return round_result * round;
        }
        round += 1;
    }
}

fn part3(input: &String) -> usize {
    let mut columns = parse_columns(input);
    let elem_count = columns.iter().flatten().count();
    let mut round_results: HashSet<usize> = HashSet::new();

    for round in 1..=elem_count {
        let round_result = do_round(&mut columns, round);
        round_results.insert(round_result);
    }
    *round_results.iter().max().unwrap()
}

fn do_round(columns: &mut Vec<VecDeque<usize>>, round: usize) -> usize {
    let col_count = columns.len();
    let old_col = (round - 1) % col_count;
    let clapper = columns[old_col].pop_front().unwrap();
    let new_col = (old_col + 1) % col_count;

    let pos = (clapper - 1) % (columns[new_col].len() * 2);
    let index = if pos < columns[new_col].len() {
        pos
    } else {
        columns[new_col].len() - (pos - columns[new_col].len())
    };
    columns[new_col].insert(index, clapper);

    columns
        .iter()
        .map(|col| col[0].to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn parse_columns(input: &String) -> Vec<VecDeque<usize>> {
    let col_count = input.lines().nth(0).unwrap().split_whitespace().count();
    let mut columns: Vec<VecDeque<usize>> = vec![VecDeque::new(); col_count];

    for line in input.lines() {
        for (col, nbr) in line.split_whitespace().enumerate() {
            columns[col].push_back(nbr.parse().unwrap());
        }
    }
    columns
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
            assert_eq!(res, "2323");
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, "2252");
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part2and3_example.txt");
            let res = part2(&input);
            assert_eq!(res, 50877075);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part2(&input);
            assert_eq!(res, 21202068741084);
        }
    }

    mod part3 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part2and3_example.txt");
            let res = part3(&input);
            assert_eq!(res, 6584);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part3(&input);
            assert_eq!(res, 4747374010031000);
        }
    }
}
