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
    let mut map = Map::new(input);
    let mut changed = true;

    while changed {
        changed = false;
        for (pos, val) in map
            .clone()
            .iter_all()
            .filter_map(|(pos, opt_val)| opt_val.map(|val| (pos, val)))
        {
            if map
                .neighbors_cross(pos)
                .filter(|&neighbor_val| neighbor_val >= val)
                .count()
                == 4
            {
                map.increment(pos);
                changed = true;
            }
        }
    }
    println!("{map}");
    map.iter().sum()
}

fn part3(input: &str) -> usize {
    let mut map = Map::new(input);
    let mut changed = true;

    while changed {
        changed = false;
        for (pos, val) in map
            .clone()
            .iter_all()
            .filter_map(|(pos, opt_val)| opt_val.map(|val| (pos, val)))
        {
            if map
                .neighbors_all(pos)
                .filter(|&neighbor_val| neighbor_val >= val)
                .count()
                == 8
            {
                map.increment(pos);
                changed = true;
            }
        }
    }
    println!("{map}");
    map.iter().sum()
}

#[derive(Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Option<usize>>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let grid: Vec<Vec<Option<usize>>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => None,
                        '#' => Some(1),
                        _ => panic!("Invalid character in map found!"),
                    })
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Map {
            grid,
            height,
            width,
        }
    }

    fn neighbors_cross(&self, pos: Position) -> impl Iterator<Item = usize> {
        let mut neighbors: Vec<Option<usize>> = Vec::new();
        if pos.row > 0 {
            neighbors.push(self.grid[pos.row - 1][pos.col]);
        }
        if pos.col > 0 {
            neighbors.push(self.grid[pos.row][pos.col - 1]);
        }
        if pos.col + 1 < self.width {
            neighbors.push(self.grid[pos.row][pos.col + 1]);
        }
        if pos.row + 1 < self.height {
            neighbors.push(self.grid[pos.row + 1][pos.col]);
        }
        neighbors.into_iter().flatten()
    }

    fn neighbors_all(&self, pos: Position) -> impl Iterator<Item = usize> + '_ {
        [
            // Cross directions
            (-1, 0),
            (0, -1),
            (0, 1),
            (1, 0),
            // Diagonal directions
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ]
        .into_iter()
        .filter_map(move |(row_offset, col_offset)| {
            let new_row = pos.row as isize + row_offset;
            let new_col = pos.col as isize + col_offset;

            if new_row >= 0
                && new_row < self.height as isize
                && new_col >= 0
                && new_col < self.width as isize
            {
                self.grid[new_row as usize][new_col as usize]
            } else {
                None
            }
        })
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.grid.iter().flatten().filter_map(|opt| *opt)
    }

    fn iter_all(&self) -> impl Iterator<Item = (Position, Option<usize>)> + '_ {
        self.grid.iter().enumerate().flat_map(|(row, row_vals)| {
            row_vals
                .iter()
                .enumerate()
                .map(move |(col, opt)| (Position::new(row, col), *opt))
        })
    }

    fn at(&self, pos: Position) -> Option<usize> {
        if pos.row >= self.height || pos.col >= self.width {
            return None;
        }
        self.grid[pos.row][pos.col]
    }

    fn increment(&mut self, pos: Position) {
        if let Some(cur_val) = self.at(pos) {
            self.grid[pos.row][pos.col] = Some(cur_val + 1);
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            let line: String = row
                .iter()
                .map(|opt| match opt {
                    Some(val) => (*val as u8 + b'0') as char,
                    None => '.',
                })
                .collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
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
            let res = part1and2(&input);
            assert_eq!(res, 35);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part1.txt");
            let res = part1and2(&input);
            assert_eq!(res, 124);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part2.txt");
            let res = part1and2(&input);
            assert_eq!(res, 2668);
        }
    }

    mod part3 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part3_example.txt");
            let res = part3(&input);
            assert_eq!(res, 29);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part3(&input);
            assert_eq!(res, 10190);
        }
    }
}
