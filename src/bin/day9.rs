fn main() {
    let input = include_str!("input9.txt");
    let input = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let weakness = day9a(&input);
    println!("a: {}", weakness);
    println!("b: {}", day9b(&input, weakness).unwrap());
}

const PREAMBLE_LENGTH: usize = 25;

fn day9a(input: &[u64]) -> u64 {
    *input
        .windows(PREAMBLE_LENGTH + 1)
        .find(|window| {
            let target = *window.last().unwrap();
            let previous = &window[0..PREAMBLE_LENGTH];

            for i in 0..previous.len() {
                for j in 0..previous.len() {
                    if i == j {
                        continue;
                    }

                    let a = previous[i];
                    let b = previous[j];

                    if a + b == target {
                        return false;
                    }
                }
            }

            true
        })
        .unwrap()
        .last()
        .unwrap()
}

fn day9b(input: &[u64], target: u64) -> Option<u64> {
    for i in 0..input.len() {
        let range = &input[i..input.len()];

        let mut acc = 0;

        for (range_offset, &elem) in range.iter().enumerate() {
            acc += elem;

            if acc == target {
                let final_range = &input[i..=i + range_offset];
                return Some(final_range.iter().min().unwrap() + final_range.iter().max().unwrap());
            }
        }
    }

    None
}
