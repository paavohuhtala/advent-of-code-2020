use nom;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input14.txt");
    println!("a: {:?}", day14a(input));
    println!("b: {:?}", day14b(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
    Floating,
}

type BitMask = HashMap<u64, Bit>;

fn parse_mask(input: &str) -> nom::IResult<&str, Statement> {
    use nom::bytes::complete::{tag, take_while};

    let (input, _) = tag("mask = ")(input)?;

    let (input, mask_chars) = take_while(|c| c == 'X' || c == '1' || c == '0')(input)?;

    let mask = mask_chars
        .chars()
        .enumerate()
        .map(|(offset, c)| match c {
            'X' => (35 - offset as u64, Bit::Floating),
            '1' => (35 - offset as u64, Bit::One),
            '0' => (35 - offset as u64, Bit::Zero),
            _ => panic!(),
        })
        .collect();

    Ok((input, Statement::SetMask(mask)))
}

#[derive(Debug)]
enum Statement {
    SetMask(BitMask),
    Assign { address: u64, value: u64 },
}

fn parse_assignment(input: &str) -> nom::IResult<&str, Statement> {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;

    let (input, _) = tag("mem[")(input)?;
    let (input, address) = digit1(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, value) = digit1(input)?;

    Ok((
        input,
        Statement::Assign {
            address: address.parse().unwrap(),
            value: value.parse().unwrap(),
        },
    ))
}

fn parse_statement(input: &str) -> nom::IResult<&str, Statement> {
    nom::branch::alt((parse_mask, parse_assignment))(input)
}

fn set_bit(value: u64, n: u64, bit_value: bool) -> u64 {
    (value & !(1 << n)) | ((bit_value as u64) << n)
}

fn apply_mask(mut value: u64, mask: &BitMask) -> u64 {
    for (&n, &bit_value) in mask.iter() {
        match bit_value {
            Bit::Floating => {}
            Bit::One => value = set_bit(value, n, true),
            Bit::Zero => value = set_bit(value, n, true),
        };
    }

    value
}

fn apply_mask_with_floating_bits(value: u64, mask: &BitMask) -> Vec<u64> {
    fn apply_mask_rec(value: u64, entries: &[(u64, Bit)]) -> Vec<u64> {
        match entries.iter().next() {
            None => return vec![value],
            Some((n, Bit::One)) => apply_mask_rec(set_bit(value, *n, true), &entries[1..]),
            Some((_, Bit::Zero)) => apply_mask_rec(value, &entries[1..]),
            Some((n, Bit::Floating)) => {
                let mut results = apply_mask_rec(set_bit(value, *n, true), &entries[1..]);
                results.append(&mut apply_mask_rec(
                    set_bit(value, *n, false),
                    &entries[1..],
                ));
                results
            }
        }
    }

    let mask_entries: Vec<(u64, Bit)> = mask.iter().map(|(offset, bit)| (*offset, *bit)).collect();
    apply_mask_rec(value, &mask_entries)
}

fn day14a(input: &str) -> u64 {
    let statements = input
        .lines()
        .map(|line| parse_statement(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut memory = HashMap::<u64, u64>::new();
    let mut current_mask = HashMap::new();

    for statement in statements {
        match statement {
            Statement::SetMask(mask) => {
                current_mask = mask;
            }
            Statement::Assign { address, value } => {
                memory.insert(address, apply_mask(value, &current_mask));
            }
        }
    }

    memory.values().sum()
}

fn day14b(input: &str) -> u64 {
    let statements = input
        .lines()
        .map(|line| parse_statement(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut memory = HashMap::<u64, u64>::new();
    let mut current_mask = HashMap::new();

    for statement in statements {
        match statement {
            Statement::SetMask(mask) => {
                current_mask = mask;
            }
            Statement::Assign { address, value } => {
                let addresses = apply_mask_with_floating_bits(address, &current_mask);

                for address in addresses {
                    memory.insert(address, value);
                }
            }
        }
    }

    memory.values().sum()
}
