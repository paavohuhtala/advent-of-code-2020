fn main() {
    let input = include_str!("input11.txt");
    let world = parse_world(input);
    println!("a: {:?}", day11a(world.clone()));
    println!("b: {:?}", day11b(world));
}

fn day11a(world: World) -> usize {
    run_simulation(
        SimulationConfig {
            count_occupied,
            max_occupied: 4,
        },
        world,
    )
}

fn day11b(world: World) -> usize {
    run_simulation(
        SimulationConfig {
            count_occupied: trace_occupied,
            max_occupied: 5,
        },
        world,
    )
}

struct SimulationConfig {
    count_occupied: fn(world: &World, x: isize, y: isize) -> usize,
    max_occupied: usize,
}

fn run_simulation(config: SimulationConfig, mut world: World) -> usize {
    loop {
        let mut next_world = world.clone();

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let current_cell = get_cell(&world, x as isize, y as isize).unwrap();
                let occupants = (config.count_occupied)(&world, x as isize, y as isize);

                let next_cell = match (current_cell, occupants) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, n) if n >= config.max_occupied => Cell::Empty,
                    (cell, _) => cell,
                };

                next_world[y * WORLD_WIDTH + x] = next_cell;
            }
        }

        if world == next_world {
            return world.iter().filter(|&&cell| cell == Cell::Occupied).count();
        } else {
            world = next_world;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Cell {
    fn parse(ch: char) -> Cell {
        match ch {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            '.' => Cell::Floor,
            _ => panic!(),
        }
    }
}

const WORLD_WIDTH: usize = 99;
const WORLD_HEIGHT: usize = 98;

type World = Vec<Cell>;

fn parse_world(input: &str) -> World {
    input
        .lines()
        .flat_map(|line| line.chars().map(Cell::parse))
        .collect()
}

fn is_in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && (x as usize) < WORLD_WIDTH && y >= 0 && (y as usize) < WORLD_HEIGHT
}

fn get_cell(world: &World, x: isize, y: isize) -> Option<Cell> {
    if is_in_bounds(x, y) {
        return Some(world[(y as usize * WORLD_WIDTH + x as usize) as usize]);
    }

    None
}

fn count_occupied(world: &World, x: isize, y: isize) -> usize {
    [
        get_cell(world, x - 1, y - 1),
        get_cell(world, x, y - 1),
        get_cell(world, x + 1, y - 1),
        get_cell(world, x + 1, y),
        get_cell(world, x + 1, y + 1),
        get_cell(world, x, y + 1),
        get_cell(world, x - 1, y + 1),
        get_cell(world, x - 1, y),
    ]
    .iter()
    .filter_map(|c| c.as_ref())
    .filter(|&&c| c == Cell::Occupied)
    .count()
}

fn trace_direction(
    world: &World,
    x: isize,
    y: isize,
    (dir_x, dir_y): (isize, isize),
) -> Option<Cell> {
    let mut x = x;
    let mut y = y;

    loop {
        x += dir_x;
        y += dir_y;

        let cell = get_cell(world, x, y);

        match cell {
            None => return None,
            Some(Cell::Floor) => {}
            Some(empty_or_occupied) => return Some(empty_or_occupied),
        }
    }
}

fn trace_occupied(world: &World, x: isize, y: isize) -> usize {
    [
        trace_direction(world, x, y, (-1, -1)),
        trace_direction(world, x, y, (0, -1)),
        trace_direction(world, x, y, (1, -1)),
        trace_direction(world, x, y, (1, 0)),
        trace_direction(world, x, y, (1, 1)),
        trace_direction(world, x, y, (0, 1)),
        trace_direction(world, x, y, (-1, 1)),
        trace_direction(world, x, y, (-1, 0)),
    ]
    .iter()
    .filter_map(|c| c.as_ref())
    .filter(|&&c| c == Cell::Occupied)
    .count()
}
