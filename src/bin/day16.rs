use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    println!("a: {:?}", day16a());
    println!("b: {:?}", day16b());
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    range1: std::ops::RangeInclusive<u64>,
    range2: std::ops::RangeInclusive<u64>,
}

fn get_input() -> (Vec<Field>, Vec<Vec<u64>>) {
    let input_fields = include_str!("./input16.fields.txt");
    let field_regex = Regex::new(r"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();

    let fields: Vec<_> = input_fields
        .lines()
        .map(|line| {
            let captures = field_regex.captures(line).unwrap();
            let name = captures[1].to_string();

            let range1_start = captures[2].parse().unwrap();
            let range1_end = captures[3].parse().unwrap();

            let range2_start = captures[4].parse().unwrap();
            let range2_end = captures[5].parse().unwrap();

            Field {
                name,
                range1: range1_start..=range1_end,
                range2: range2_start..=range2_end,
            }
        })
        .collect();

    let input_tickets = include_str!("./input16.nearby_tickets.txt");
    let tickets: Vec<Vec<u64>> = input_tickets
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (fields, tickets)
}

fn day16a() -> u64 {
    let (fields, tickets) = get_input();

    let mut error_rate = 0;

    for ticket in &tickets {
        'check_number: for number in ticket {
            for field in &fields {
                if field.range1.contains(number) || field.range2.contains(number) {
                    continue 'check_number;
                }
            }
            error_rate += number;
        }
    }
    error_rate
}

fn day16b() -> u64 {
    let (fields, tickets) = get_input();

    let valid_tickets: Vec<Vec<u64>> = tickets
        .into_iter()
        .filter(|ticket| {
            'check_number: for number in ticket {
                for field in &fields {
                    if field.range1.contains(number) || field.range2.contains(number) {
                        continue 'check_number;
                    }
                }

                return false;
            }

            true
        })
        .collect();

    let number_of_fields = valid_tickets[0].len();

    let mut possible_fields_for_index: HashMap<usize, HashSet<&str>> = HashMap::new();

    let mut resolved_fields: HashMap<usize, &str> = HashMap::new();

    for field_i in 0..number_of_fields {
        let values: Vec<u64> = valid_tickets.iter().map(|ticket| ticket[field_i]).collect();

        let mut possible_fields: Vec<&Field> = fields.iter().collect();
        possible_fields.retain(|field| {
            for value in &values {
                if !(field.range1.contains(&value) || field.range2.contains(&value)) {
                    return false;
                }
            }

            true
        });

        possible_fields_for_index.insert(
            field_i,
            possible_fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<HashSet<_>>(),
        );
    }

    while possible_fields_for_index.len() > 0 {
        let (&i, _) = possible_fields_for_index
            .iter()
            .find(|(_, fields)| fields.len() == 1)
            .unwrap();

        let singleton_set = possible_fields_for_index.remove(&i).unwrap();

        let field = singleton_set.iter().next().unwrap();
        possible_fields_for_index.remove(&i);

        for field_set in possible_fields_for_index.values_mut() {
            field_set.remove(field);
        }

        resolved_fields.insert(i, field);
    }

    let my_ticket: Vec<u64> = include_str!("./input16.my_ticket.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    resolved_fields
        .iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|(&i, _)| my_ticket[i])
        .product()
}
