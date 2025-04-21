use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let available: usize = input.parse().unwrap();
    let mut blocks = 1;
    let mut width = 1;

    while blocks < available {
        width += 2;
        blocks += width;
    }
    (blocks - available) * width
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
            assert_eq!(res, 21);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, 7822668);
        }
    }
}
