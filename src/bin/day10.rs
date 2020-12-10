use itertools::Itertools;

fn main() {
    let input = include_str!("input10.txt");
    let input = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", day10a(&input));
    println!("{}", day10b(&input));
}

fn day10a(input: &[u64]) -> usize {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    sorted_input.insert(0, 0);
    sorted_input.push(sorted_input.iter().max().unwrap() + 3);

    let diffs: Vec<u64> = sorted_input
        .into_iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .collect();

    let ones = diffs.iter().filter(|&&i| i == 1).count();
    let threes = diffs.iter().filter(|&&i| i == 3).count();

    ones * threes
}

fn day10b(input: &[u64]) -> usize {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    sorted_input.insert(0, 0);

    let target = *sorted_input.last().unwrap();

    let hmmmm = sorted_input
        .iter()
        .combinations(3)
        .take(50)
        .collect::<Vec<_>>();

    println!("{:?}", hmmmm);

    0
}
