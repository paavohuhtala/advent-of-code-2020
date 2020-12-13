use itertools::Itertools;

fn main() {
    let input = include_str!("./input13.txt");

    println!("a: {:?}", day13a(input));
    day13b(input);
}

fn day13a(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let target: u64 = lines[0].parse().unwrap();
    let bus_ids: Vec<u64> = lines[1]
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for t in target.. {
        for &bus_id in &bus_ids {
            if t % bus_id == 0 {
                return (t - target) * bus_id;
            }
        }
    }

    0
}

fn day13b(input: &str) {
    let equation = input
        .lines()
        .last()
        .map(|line| {
            line.split(',')
                .enumerate()
                .filter(|(_, s)| *s != "x")
                .map(|(offset, id)| format!("(t + {}) mod {} = 0", offset, id))
                .join(", ")
        })
        .unwrap();

    println!("copy-paste the following to wolframalpha");
    println!("solve {}", equation);
}
