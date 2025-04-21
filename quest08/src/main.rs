use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input, 1111, 20240000));
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

fn part2(input: &str, acolytes: usize, available: usize) -> usize {
    let priests: usize = input.parse().unwrap();
    let mut blocks = 1;
    let mut width = 1;
    let mut thickness = 1;

    while blocks < available {
        thickness = (thickness * priests) % acolytes;
        width += 2;
        blocks += width * thickness;
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

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part2_example.txt");
            let res = part2(&input, 5, 50);
            assert_eq!(res, 27);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part2(&input, 1111, 20240000);
            assert_eq!(res, 133388862);
        }
    }
}
