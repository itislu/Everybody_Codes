use utils::input;
use utils::parse;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let targets: Vec<usize> = parse::numbers(input).collect();
    let stamps: Vec<usize> = vec![10, 5, 3, 1];
    let mut total = 0;

    for mut remaining in targets {
        for stamp in &stamps {
            total += remaining / stamp;
            remaining %= stamp;
        }
    }
    total
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
}
