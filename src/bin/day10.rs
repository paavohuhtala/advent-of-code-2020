use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input10.txt");
    let input = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("a {}", day10a(&input));
    println!("b {}", day10b(&input));
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
    let target = *sorted_input.last().unwrap();

    fn count_ways(
        target: u64,
        current: u64,
        remaining: &[u64],
        count_cache: &mut HashMap<u64, usize>,
    ) -> usize {
        if current == target {
            return 1;
        }

        match count_cache.get(&current) {
            Some(value) => {
                return *value;
            }
            None => {}
        };

        let mut total = 0;

        for (i, &next) in remaining.iter().enumerate() {
            if next > current + 3 {
                break;
            }

            total += count_ways(target, next, &remaining[i + 1..], count_cache);
        }

        count_cache.insert(current, total);

        return total;
    }

    let mut count_cache: HashMap<u64, usize> = HashMap::new();
    count_ways(target, 0, &sorted_input, &mut count_cache)
}
