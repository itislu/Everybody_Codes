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
    let col_count = columns.len();

    for round in 0..10 {
        let mut cur_col = round % col_count;
        let clapper = columns[cur_col].pop_front().expect("empty column");
        let mut claps_remaining = clapper;

        loop {
            cur_col = (cur_col + 1) % col_count;
            if claps_remaining <= columns[cur_col].len() {
                columns[cur_col].insert(claps_remaining - 1, clapper);
                break;
            }
            if claps_remaining <= columns[cur_col].len() * 2 {
                let idx = columns[cur_col].len() - (claps_remaining - columns[cur_col].len()) + 1;
                columns[cur_col].insert(idx, clapper);
                break;
            }
            claps_remaining -= columns[cur_col].len() * 2;
        }
    }

    columns.iter().map(|col| col[0].to_string()).collect()
}

fn part2(input: &String) -> usize {
    let mut columns = parse_columns(input);
    let col_count = columns.len();
    let mut round_results: HashMap<usize, usize> = HashMap::new();
    let mut round = 1;

    loop {
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

        let round_result: usize = columns
            .iter()
            .map(|col| col[0].to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        let new_count = round_results
            .get(&round_result)
            .and_then(|count| Some(count + 1))
            .unwrap_or(1);

        if new_count == 2024 {
            return round_result * round;
        }

        round_results.insert(round_result, new_count);
        round += 1;
    }
}

fn part3(input: &String) -> usize {
    let mut columns = parse_columns(input);
    let col_count = columns.len();
    let elem_count = columns.iter().flatten().count();
    let mut round_results: HashSet<usize> = HashSet::new();

    for round in 0..elem_count {
        let old_col = round % col_count;
        let clapper = columns[old_col].pop_front().unwrap();
        let new_col = (old_col + 1) % col_count;

        let pos = (clapper - 1) % (columns[new_col].len() * 2);
        let index = if pos < columns[new_col].len() {
            pos
        } else {
            columns[new_col].len() - (pos - columns[new_col].len())
        };
        columns[new_col].insert(index, clapper);

        let round_result: usize = columns
            .iter()
            .map(|col| col[0].to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        round_results.insert(round_result);
    }

    *round_results.iter().max().unwrap()
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
