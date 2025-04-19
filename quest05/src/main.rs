use std::collections::VecDeque;
use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
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
}
