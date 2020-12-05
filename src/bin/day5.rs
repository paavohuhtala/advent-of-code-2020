use itertools::Itertools;

fn main() {
    let input = include_str!("input5.txt");
    println!("a {:?}", day5a(input));
    println!("b {:?}", day5b(input));
}

fn parse_boarding_pass(input: &str) -> u32 {
    let binary_input = input
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!("Unexpected char {}", c),
        })
        .collect::<String>();

    let row = u32::from_str_radix(&binary_input[0..7], 2).unwrap();
    let column = u32::from_str_radix(&binary_input[7..], 2).unwrap();

    row * 8 + column
}

fn day5a(input: &str) -> Option<u32> {
    input.lines().map(parse_boarding_pass).max()
}

fn day5b(input: &str) -> Option<u32> {
    input
        .lines()
        .map(parse_boarding_pass)
        .sorted()
        .tuple_windows()
        .find(|(a, b)| *a < *b - 1)
        .map(|(a, _)| a + 1)
}
