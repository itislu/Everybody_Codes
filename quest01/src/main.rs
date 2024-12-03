use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input));
    let input = input::read_file("inputs/part3.txt");
    println!("exercise 3: {}", part3(&input));
}

fn part1(input: &String) -> usize {
    count_occurrences(input, 'B') + count_occurrences(input, 'C') * 3
}

fn part2(input: &String) -> usize {
    let mut res: usize = part1(input) + count_occurrences(input, 'D') * 5;
    for pair in input.as_bytes().chunks(2) {
        if pair[0] != b'x' && pair[1] != b'x' {
            res += 2;
        }
    }
    res
}

fn part3(input: &String) -> usize {
    let mut res: usize = part1(input) + count_occurrences(input, 'D') * 5;
    for triple in input.as_bytes().chunks(3) {
        match triple.iter().filter(|&&c| c == b'x').count() {
            0 => res += 6,
            1 => res += 2,
            _ => {}
        }
    }
    res
}

fn count_occurrences(input: &String, c: char) -> usize {
    input.matches(c).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input::read_file("inputs/part1_example.txt");
        let res = part1(&input);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_part2() {
        let input = input::read_file("inputs/part2_example.txt");
        let res = part2(&input);
        assert_eq!(res, 28);
    }

    #[test]
    fn test_part3() {
        let input = input::read_file("inputs/part3_example.txt");
        let res = part3(&input);
        assert_eq!(res, 30);
    }
}
