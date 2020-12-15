use std::collections::HashMap;

fn main() {
    let input = include_str!("./input15.txt");
    let input = input
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("a: {:?}", day15a(&input));
    println!("b: {:?}", day15b(&input));
}

fn day15a(input: &[u64]) -> u64 {
    let mut memory = Vec::new();
    let mut turn = 1u32;

    for number in input {
        memory.push(*number);
        turn += 1;
    }

    let mut previous = *input.last().unwrap();

    while turn <= 2020 {
        let maybe_previous = memory
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|(_, &x)| x == previous);

        let next_number = match maybe_previous {
            None => 0,
            Some((age, _)) => (turn as u64 - 1) - (age as u64 + 1),
        };

        memory.push(next_number);
        previous = next_number;

        turn += 1;
    }

    memory[2019]
}

fn day15b(input: &[u64]) -> u64 {
    let mut memory: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut turn = 1u64;

    for &number in input {
        memory.entry(number).or_default().push(turn);
        turn += 1;
    }

    let mut previous = *input.last().unwrap();

    while turn <= 30_000_000 {
        if turn % 1_000_000 == 0 {
            println!("{}", turn);
        }

        let previous_instances = memory.get(&previous).unwrap();

        let next_number = if previous_instances.len() == 1 {
            0
        } else {
            let previous_turn = previous_instances[previous_instances.len() - 2];
            let age = turn - previous_turn - 1;
            age
        };

        memory.entry(next_number).or_default().push(turn);
        previous = next_number;

        turn += 1;
    }

    previous
}
