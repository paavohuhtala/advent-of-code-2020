use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input6.txt");
    println!("a {:?}", day6a(input));
    println!("b {:?}", day6b(input));
}

fn day6a(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(char::is_ascii_alphabetic)
                .unique()
                .count()
        })
        .sum()
}

fn day6b(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.chars()
                        .filter(char::is_ascii_alphabetic)
                        .collect::<HashSet<char>>()
                })
                .fold1(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}
