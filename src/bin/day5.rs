use num::Integer;
use std::convert::TryInto;

fn main() {
    let input = include_str!("input5.txt");
    println!("a {:?}", day5a(input));
    println!("b {:?}", day5b(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BspDir {
    Upper,
    Lower,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoardingPass {
    encoded_row: [BspDir; 7],
    encoded_column: [BspDir; 3],
}

fn parse_boarding_pass(input: &str) -> BoardingPass {
    let encoded_row: [BspDir; 7] = input
        .chars()
        .take(7)
        .map(|c| match c {
            'F' => BspDir::Lower,
            'B' => BspDir::Upper,
            _ => panic!("Unexpected char {}", c),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let encoded_column: [BspDir; 3] = input
        .chars()
        .skip(7)
        .map(|c| match c {
            'L' => BspDir::Lower,
            'R' => BspDir::Upper,
            _ => panic!("Unexpected char {}", c),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    BoardingPass {
        encoded_row,
        encoded_column,
    }
}

pub(crate) fn apply_bsp(
    range: std::ops::RangeInclusive<u32>,
    dir: &BspDir,
) -> std::ops::RangeInclusive<u32> {
    let length: u32 = range.end() - range.start();
    match dir {
        BspDir::Upper => (range.start() + length.div_ceil(&2))..=(*range.end()),
        BspDir::Lower => (*range.start())..=range.start() + length.div_floor(&2),
    }
}

pub(crate) fn reduce_bsp(full_range: std::ops::RangeInclusive<u32>, ops: &[BspDir]) -> u32 {
    let range = ops.iter().fold(full_range, |range, d| {
        let applied = apply_bsp(range.clone(), d);
        applied
    });
    assert_eq!(range.start(), range.end());
    *range.start()
}

fn day5a(input: &str) -> Option<u32> {
    let boarding_passes: Vec<_> = input.lines().map(parse_boarding_pass).collect();

    boarding_passes
        .iter()
        .map(|pass| {
            let row = reduce_bsp(0..=127, &pass.encoded_row);
            let column = reduce_bsp(0..=7, &pass.encoded_column);
            row * 8 + column
        })
        .max()
}

fn day5b(input: &str) -> Option<u32> {
    let boarding_passes: Vec<_> = input.lines().map(parse_boarding_pass).collect();

    let mut ids: Vec<u32> = boarding_passes
        .iter()
        .map(|pass| {
            let row = reduce_bsp(0..=127, &pass.encoded_row);
            let column = reduce_bsp(0..=7, &pass.encoded_column);
            row * 8 + column
        })
        .collect();

    ids.sort();
    ids.windows(2)
        .find(|ids| ids[0] < ids[1] - 1)
        .map(|ids| ids[0] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_bsp() {
        assert_eq!(apply_bsp(0..=127, &BspDir::Lower), 0..=63);
        assert_eq!(apply_bsp(0..=63, &BspDir::Upper), 32..=63);
        assert_eq!(apply_bsp(32..=63, &BspDir::Lower), 32..=47);
        assert_eq!(apply_bsp(32..=47, &BspDir::Upper), 40..=47);
    }
}
