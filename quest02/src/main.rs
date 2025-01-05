use strum::IntoEnumIterator;
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
    let mut res: usize = 0;
    let runes = get_runes(input);
    let words = input.lines().last().unwrap();

    for rune in runes {
        res += words.matches(&rune).count();
    }
    res
}

fn part2(input: &String) -> usize {
    let runes = get_runes(input);
    let words = input.lines().skip(2).collect::<Vec<_>>().join("\n");
    let mut used_indeces = vec![false; words.len()];

    for rune in runes {
        let rune_rev = rune.chars().rev().collect::<String>();
        for (win_pos, window) in words.as_bytes().windows(rune.len()).enumerate() {
            if window == rune.as_bytes() || window == rune_rev.as_bytes() {
                for i in 0..window.len() {
                    used_indeces[win_pos + i] = true;
                }
            }
        }
    }
    used_indeces.iter().filter(|&&x| x == true).count()
}

fn part3(input: &String) -> usize {
    let runes = get_runes(input);
    let grid = Grid::new(&input.split("\n\n").nth(1).unwrap_or("").to_string());
    let mut used_indeces: Vec<Vec<bool>> = vec![vec![false; grid.width]; grid.height];

    for rune in runes {
        let rune_rev = rune.chars().rev().collect::<String>();
        for direction in Direction::iter() {
            for (i, line) in grid.iter_direction(direction).enumerate() {
                line.as_bytes()
                    .iter()
                    .copied()
                    .cycle()
                    .take(match direction {
                        Direction::Horizontal => line.len() + rune.len() - 1,
                        Direction::Vertical => line.len(),
                    })
                    .collect::<Vec<_>>()
                    .windows(rune.len())
                    .enumerate()
                    .filter(|&(_, w)| w == rune.as_bytes() || w == rune_rev.as_bytes())
                    .for_each(|(j, w)| set_used(&mut used_indeces, i, j, w.len(), direction));
            }
        }
    }
    used_indeces
        .iter()
        .flatten()
        .filter(|&&x| x == true)
        .count()
}

fn set_used(
    used_indeces: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    len: usize,
    direction: Direction,
) {
    let max_row = used_indeces.len();
    let max_col = used_indeces[0].len();
    match direction {
        Direction::Horizontal => {
            for n in 0..len {
                used_indeces[i][(j + n) % max_col] = true;
            }
        }
        Direction::Vertical => {
            for n in 0..len {
                used_indeces[(j + n) % max_row][i] = true;
            }
        }
    }
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

// Direction of strings in Grid
#[derive(Clone, Copy, Debug, strum_macros::EnumIter)]
enum Direction {
    Horizontal,
    Vertical,
}

struct Grid {
    matrix: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

struct GridIterator<'a> {
    grid: &'a Grid,
    current_row: usize,
    current_col: usize,
    direction: Direction,
}

impl Grid {
    fn new(input: &String) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = matrix.len();
        let width = matrix[0].len();
        Grid {
            matrix,
            height,
            width,
        }
    }

    fn iter_direction(&self, direction: Direction) -> GridIterator {
        GridIterator::new(self, direction)
    }
}

impl<'a> GridIterator<'a> {
    fn new(grid: &'a Grid, direction: Direction) -> Self {
        let (current_row, current_col) = (0, 0);
        GridIterator {
            grid,
            current_row,
            current_col,
            direction,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let res: String;

        match self.direction {
            Direction::Horizontal => {
                if self.current_row == self.grid.height {
                    return None;
                }
                res = self.grid.matrix[self.current_row].iter().collect();
                self.current_row += 1;
            }
            Direction::Vertical => {
                if self.current_col == self.grid.width {
                    return None;
                }
                res = self
                    .grid
                    .matrix
                    .iter()
                    .map(|row| row[self.current_col])
                    .collect();
                self.current_col += 1;
            }
        }
        Some(res)
    }
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
            assert_eq!(res, 12);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1(&input);
            assert_eq!(res, 34);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part2_example.txt");
            let res = part2(&input);
            assert_eq!(res, 42);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part2(&input);
            assert_eq!(res, 5165);
        }
    }

    mod part3 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part3_example.txt");
            let res = part3(&input);
            assert_eq!(res, 10);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part3(&input);
            assert_eq!(res, 12076);
        }
    }
}
