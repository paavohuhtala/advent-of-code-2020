use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./input17.txt");
    println!("a: {:?}", day17a(input));
    println!("b: {:?}", day17b(input));
}

fn day17a(input: &str) -> usize {
    solve::<Vec3>(input)
}

fn day17b(input: &str) -> usize {
    solve::<Vec4>(input)
}

fn solve<C: Coord>(input: &str) -> usize {
    let mut world = create_world_from_input::<C>(input);

    for _ in 0..=5 {
        world = simulate_tick(world);
    }

    let active_cubes = world.len();
    active_cubes
}

fn create_world_from_input<C: Coord>(input: &str) -> BTreeSet<C> {
    let mut world = BTreeSet::<C>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '#' {
                continue;
            }

            world.insert(C::from_x_y(x as i32, y as i32));
        }
    }

    world
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Vec3(i32, i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Vec4(i32, i32, i32, i32);

trait Coord: Eq + PartialEq + PartialOrd + Ord + Copy + Clone {
    fn from_x_y(x: i32, y: i32) -> Self;
    fn get_neighbors(self, filter: Filter) -> Box<dyn Iterator<Item = Self>>;
}

impl Coord for Vec3 {
    fn get_neighbors(self, filter: Filter) -> Box<dyn Iterator<Item = Self>> {
        Box::new((-1..=1).flat_map(move |z| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).filter_map(move |x| {
                    if filter == Filter::OnlyNeighbors && x == 0 && y == 0 && z == 0 {
                        return None;
                    }
                    Some(Vec3(self.0 + x, self.1 + y, self.2 + z))
                })
            })
        }))
    }

    fn from_x_y(x: i32, y: i32) -> Self {
        Vec3(x, y, 0)
    }
}

impl Coord for Vec4 {
    fn get_neighbors(self, filter: Filter) -> Box<dyn Iterator<Item = Self>> {
        Box::new((-1..=1).flat_map(move |w| {
            (-1..=1).flat_map(move |z| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).filter_map(move |x| {
                        if filter == Filter::OnlyNeighbors && x == 0 && y == 0 && z == 0 && w == 0 {
                            return None;
                        }
                        Some(Vec4(self.0 + x, self.1 + y, self.2 + z, self.3 + w))
                    })
                })
            })
        }))
    }

    fn from_x_y(x: i32, y: i32) -> Self {
        Vec4(x, y, 0, 0)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Filter {
    IncludeSelf,
    OnlyNeighbors,
}

fn simulate_tick<C: Coord>(world: BTreeSet<C>) -> BTreeSet<C> {
    let mut new_world = BTreeSet::new();

    let potentially_active_nodes = world
        .iter()
        .flat_map(|pos| pos.get_neighbors(Filter::IncludeSelf))
        .collect::<BTreeSet<_>>();

    for this_pos in &potentially_active_nodes {
        let is_active = world.contains(&this_pos);

        let neighbor_count = this_pos
            .get_neighbors(Filter::OnlyNeighbors)
            .filter(|pos| world.contains(&pos))
            .count();

        match (is_active, neighbor_count) {
            (true, 2) | (true, 3) | (false, 3) => {
                new_world.insert(*this_pos);
            }
            _ => {}
        }
    }

    new_world
}
