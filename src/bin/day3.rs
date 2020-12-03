const MAP_HEIGHT: usize = 323;
const MAP_WIDTH: usize = 31;

fn main() {
    let input = include_str!("input3.txt");
    let input = parse_input(input);
    println!("3a {:?}", day3a(&input));
    println!("3b {:?}", day3b(&input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GridCell {
    Open,
    Tree,
}

fn parse_input(input: &str) -> Vec<GridCell> {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| {
                if ch == '#' {
                    GridCell::Tree
                } else {
                    GridCell::Open
                }
            })
        })
        .collect()
}

fn count_trees(map: &[GridCell], move_x: usize, move_y: usize) -> u64 {
    let mut y = 0;
    let mut x = 0;

    let mut trees = 0;

    while y < MAP_HEIGHT - (move_y) {
        y += move_y;
        x = (x + move_x) % MAP_WIDTH;

        if *map.get(y * MAP_WIDTH + x).unwrap() == GridCell::Tree {
            trees += 1;
        }
    }

    trees
}

fn day3a(input: &[GridCell]) -> u64 {
    count_trees(input, 3, 1)
}

fn day3b(input: &[GridCell]) -> u64 {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
}
