use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
}

fn part1(input: &String) -> usize {
    let mut res: usize = 0;
    let runes = get_runes(input);

    for rune in runes {
        res += input.lines().last().unwrap().matches(&rune).count();
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
}
