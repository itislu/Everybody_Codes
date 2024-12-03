use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input));
}

fn part1(input: &String) -> usize {
    let mut res: usize = 0;
    let runes = get_runes(input);
    let words = input.lines().last().unwrap();

    for rune in runes {
        res += words.matches(&rune).count();
    }
    res
}

fn part2(input: &String) -> usize {
    let mut res: usize = 0;
    let runes = get_runes(input);
    let words = input.lines().skip(2).collect::<String>();
    let words_rev = words.chars().rev().collect::<String>();

    for rune in runes {
        res += words.matches(&rune).count() * rune.len();
        res += words_rev.matches(&rune).count() * rune.len();
    }
    res
}

fn get_runes(input: &String) -> Vec<&str> {
    input
        .lines()
        .find(|line| line.starts_with("WORDS:"))
        .unwrap()
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input::read_file("inputs/part1_example.txt");
        let res = part1(&input);
        assert_eq!(res, 12);
    }

    #[test]
    fn test_part2() {
        let input = input::read_file("inputs/part2_example.txt");
        let res = part2(&input);
        assert_eq!(res, 42);
    }
}
