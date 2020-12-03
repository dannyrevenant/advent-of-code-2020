use std::fs;

struct Map {
    width: usize,
    lines: Vec<String>,
}

impl Map {
    fn new(lines: Vec<String>) -> Map {
        let width = lines[0].chars().count();
        Map { lines, width }
    }

    fn get_hits_for_slope(&self, slope: (usize, usize)) -> usize {
        let mut current_position = (0, 0);
        let mut hits = 0;

        loop {
            current_position.0 += slope.0;
            current_position.1 += slope.1;

            if current_position.0 >= self.lines.len() {
                break;
            }

            current_position.1 %= self.width;

            let next_pos = self.lines[current_position.0]
                .chars()
                .nth(current_position.1)
                .unwrap();

            if next_pos == '#' {
                hits += 1;
            }
        }

        hits
    }
}

pub fn main() {
    let contents =
        fs::read_to_string("./input/03.txt").expect("Something went wrong reading the file");

    let map = Map::new(contents.lines().map(String::from).collect());

    // Part 1 - Single slope
    let slope = (1, 3);

    let total_hits = map.get_hits_for_slope(slope);
    println!("Total hits: {}", total_hits);

    // #2 more complex
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    // Using fold (single pass)
    let total_hits = slopes.iter().fold(1, |total, slope: &(usize, usize)| {
        total * map.get_hits_for_slope(*slope)
    });
    println!("Total hits w/ fold: {}", total_hits);

    // Alternative method, using map + product (maybe more readable, probably slower but you never know with Rust)
    let total_hits = slopes
        .iter()
        .map(|slope| map.get_hits_for_slope(*slope))
        .product::<usize>();
    println!("Total hits w/ map + product: {}", total_hits);
}
