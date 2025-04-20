use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1and2(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part1and2(&input));
    let input = input::read_file("inputs/part3.txt");
    println!("exercise 3: {}", part3(&input));
}

fn part1and2(input: &str) -> usize {
    let nails: Vec<usize> = get_nails(input);
    let min = *nails.iter().min().unwrap();

    count_hits_to_target(&nails, min)
}

fn part3(input: &str) -> usize {
    let nails: Vec<usize> = get_nails(input);
    let avg = nails.iter().sum::<usize>() / nails.len();

    usize::min(
        try_while_better(&nails, avg, 1),
        try_while_better(&nails, avg, -1),
    )
}

fn get_nails(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_hits_to_target(nails: &[usize], target: usize) -> usize {
    nails.iter().map(|nail| nail.abs_diff(target)).sum()
}

fn try_while_better(nails: &[usize], start: usize, dir: isize) -> usize {
    let mut best = count_hits_to_target(nails, start);
    let mut offset = dir;

    loop {
        let tmp = count_hits_to_target(nails, (start as isize + offset) as usize);
        if tmp > best {
            break;
        }
        best = tmp;
        offset += dir;
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part1_example.txt");
            let res = part1and2(&input);
            assert_eq!(res, 10);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1and2(&input);
            assert_eq!(res, 84);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part1and2(&input);
            assert_eq!(res, 919880);
        }
    }

    mod part3 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part3_example.txt");
            let res = part3(&input);
            assert_eq!(res, 8);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part3(&input);
            assert_eq!(res, 129441494);
        }
    }
}
