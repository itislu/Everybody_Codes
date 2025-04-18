use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &String) -> usize {
    let nails: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let min = nails.iter().min().unwrap_or(&0);
    nails.iter().map(|nail| nail - min).sum()
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
            assert_eq!(res, 84);
        }
    }
}
