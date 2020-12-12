fn main() {
    let input = include_str!("input12.txt");
    let commands: Vec<Command> = input.lines().map(parse_command).collect();
    println!("day12a {}", day12a(&commands));
    println!("day12b {}", day12b(&commands));
}

fn day12a(commands: &[Command]) -> i32 {
    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: AbsDirection::East,
    };

    for command in commands {
        ship.eval_command(command);
    }

    ship.x.abs() + ship.y.abs()
}

fn day12b(commands: &[Command]) -> i32 {
    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: AbsDirection::East,
    };

    for command in commands {
        match *command {
            Command::MoveDir(dir, units) => {
                let (x, y) = move_dir(waypoint_x, waypoint_y, dir, units);
                waypoint_x = x;
                waypoint_y = y;
            }
            Command::Turn(dir, units) => {
                let (new_waypoint_x, new_waypoint_y) = match (dir, units) {
                    (_, 0) => (waypoint_x, waypoint_y),
                    (TurnDir::Right, 180) | (TurnDir::Left, 180) => (-waypoint_x, -waypoint_y),
                    (TurnDir::Right, 90) | (TurnDir::Left, 270) => (-waypoint_y, waypoint_x),
                    (TurnDir::Left, 90) | (TurnDir::Right, 270) => (waypoint_y, -waypoint_x),
                    _ => panic!(),
                };
                waypoint_x = new_waypoint_x;
                waypoint_y = new_waypoint_y;
            }
            Command::MoveForward(times) => {
                ship.x += waypoint_x * times;
                ship.y += waypoint_y * times;
            }
        };
    }

    ship.x.abs() + ship.y.abs()
}

fn parse_command(input: &str) -> Command {
    use AbsDirection::*;
    use Command::*;
    use TurnDir::*;

    let units: i32 = input[1..].parse().unwrap();

    match input.chars().next().unwrap() {
        'N' => MoveDir(North, units),
        'S' => MoveDir(South, units),
        'E' => MoveDir(East, units),
        'W' => MoveDir(West, units),
        'L' => Turn(Left, units),
        'R' => Turn(Right, units),
        'F' => MoveForward(units),
        _ => panic!(),
    }
}

#[derive(Debug)]
enum Command {
    MoveDir(AbsDirection, i32),
    MoveForward(i32),
    Turn(TurnDir, i32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum AbsDirection {
    East,
    South,
    West,
    North,
}

impl AbsDirection {
    fn to_degrees(self) -> i32 {
        match self {
            AbsDirection::East => 0,
            AbsDirection::South => 90,
            AbsDirection::West => 180,
            AbsDirection::North => 270,
        }
    }

    fn from_degrees(degrees: i32) -> AbsDirection {
        match degrees {
            0 => AbsDirection::East,
            90 => AbsDirection::South,
            180 => AbsDirection::West,
            270 => AbsDirection::North,
            _ => panic!("Non-cardinal direction ({})", degrees),
        }
    }

    fn rotate(self, direction: TurnDir, degrees: i32) -> AbsDirection {
        match direction {
            TurnDir::Right => {
                AbsDirection::from_degrees((self.to_degrees() + degrees).rem_euclid(360))
            }
            TurnDir::Left => {
                AbsDirection::from_degrees((self.to_degrees() - degrees).rem_euclid(360))
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum TurnDir {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
struct Ship {
    x: i32,
    y: i32,
    dir: AbsDirection,
}

fn move_dir(x: i32, y: i32, dir: AbsDirection, units: i32) -> (i32, i32) {
    match dir {
        AbsDirection::North => (x, y - units),
        AbsDirection::South => (x, y + units),
        AbsDirection::East => (x + units, y),
        AbsDirection::West => (x - units, y),
    }
}

impl Ship {
    fn move_dir(&mut self, dir: AbsDirection, units: i32) {
        let (x, y) = move_dir(self.x, self.y, dir, units);
        self.x = x;
        self.y = y;
    }

    fn eval_command(&mut self, command: &Command) {
        match *command {
            Command::MoveDir(dir, units) => self.move_dir(dir, units),
            Command::Turn(dir, degrees) => self.dir = self.dir.rotate(dir, degrees),
            Command::MoveForward(units) => self.move_dir(self.dir, units),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_tests() {
        let mut ship = Ship {
            x: 0,
            y: 0,
            dir: AbsDirection::East,
        };

        ship.eval_command(&Command::Turn(TurnDir::Left, 90));

        let expected_ship = Ship {
            x: 0,
            y: 0,
            dir: AbsDirection::North,
        };

        assert_eq!(expected_ship, ship);

        ship.eval_command(&Command::Turn(TurnDir::Left, 90));

        let expected_ship = Ship {
            x: 0,
            y: 0,
            dir: AbsDirection::West,
        };

        assert_eq!(expected_ship, ship);
    }
}
