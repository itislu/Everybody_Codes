use utils::input;

fn main() {
    let input = input::read_file("inputs/part1.txt");
    println!("exercise 1: {}", part1(&input));
    let input = input::read_file("inputs/part2.txt");
    println!("exercise 2: {}", part2(&input, 1111, 20240000));
    let input = input::read_file("inputs/part3.txt");
    println!("exercise 3: {}", part3(&input, 10, 202400000));
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

fn part3(input: &str, acolytes: usize, available: usize) -> usize {
    let priests: usize = input.parse().unwrap();
    let mut shrine = Shrine::new();

    while shrine.blocks < available {
        shrine.add_layer(priests, acolytes);
    }

    let mut empty_blocks;
    loop {
        empty_blocks = shrine.empty_blocks(priests, acolytes);
        if shrine.blocks - empty_blocks >= available {
            break;
        }
        shrine.add_layer(priests, acolytes);
    }
    shrine.blocks - empty_blocks - available
}

struct Shrine {
    blocks: usize,
    width: usize,
    col_heights: Vec<usize>,
    top_layer_height: usize,
}

impl Shrine {
    fn new() -> Self {
        let top_layer_height = 1;
        Self {
            blocks: top_layer_height,
            width: 1,
            col_heights: vec![top_layer_height],
            top_layer_height,
        }
    }

    fn add_layer(&mut self, priests: usize, acolytes: usize) {
        self.top_layer_height = (self.top_layer_height * priests) % acolytes + acolytes;
        for col_height in &mut self.col_heights {
            *col_height += self.top_layer_height;
        }
        self.col_heights.push(self.top_layer_height);
        self.width += 2;
        self.blocks += self.width * self.top_layer_height;
    }

    fn empty_blocks(&self, priests: usize, acolytes: usize) -> usize {
        let base_multiplier = priests * self.width;
        let mut empty_blocks = (base_multiplier * self.col_heights[0]) % acolytes;
        for i in 1..self.col_heights.len() - 1 {
            empty_blocks += ((base_multiplier * self.col_heights[i]) % acolytes) * 2;
        }
        empty_blocks
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

    mod part3 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("inputs/part3_example.txt");
            let res = part3(&input, 5, 160);
            assert_eq!(res, 2);
        }

        #[test]
        fn answer() {
            let input = input::read_file("inputs/part3.txt");
            let res = part3(&input, 10, 202400000);
            assert_eq!(res, 41067);
        }
    }
}
