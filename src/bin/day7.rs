use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use nom;

fn main() {
    let input = include_str!("input7.txt");
    println!("day7: {:?}", day7(input));
}

fn day7(input: &str) -> (usize, usize) {
    let output: Result<Vec<Rule>, _> = input
        .lines()
        .map(parse_rule)
        .map(|inner| inner.map(|(_, rule)| rule))
        .collect();

    let rules = output.unwrap();

    let rules: HashMap<Color, Vec<(Color, u32)>> = rules
        .into_iter()
        .map(|rule| (rule.bag_color, rule.contains))
        .collect();

    let mut resolved = HashMap::new();

    for (bag_color, contains) in &rules {
        match resolved.get(bag_color) {
            Some(_) => {}
            None => {
                let mut queue = contains.iter().map(|(c, _)| c).collect::<Vec<_>>();
                let mut reachable = HashSet::new();

                while !queue.is_empty() {
                    let color = queue.pop().unwrap();

                    if reachable.contains(color) {
                        continue;
                    }
                    reachable.insert(color);

                    match resolved.get(color) {
                        Some(nodes) => reachable.extend(nodes),
                        None => {
                            let inner = rules.get(color).unwrap();
                            queue.extend(inner.iter().map(|(c, _)| c));
                        }
                    }
                }
                resolved.insert(bag_color.clone(), reachable);
            }
        }
    }

    let gold = Color {
        prefix: "shiny".to_string(),
        primary: "gold".to_string(),
    };

    let a = resolved
        .iter()
        .filter(|(_, contents)| contents.contains(&gold))
        .count();

    fn count_children(
        count_cache: &mut HashMap<Color, usize>,
        rules: &HashMap<Color, Vec<(Color, u32)>>,
        node_color: &Color,
    ) -> usize {
        let children = match count_cache.get(node_color) {
            Some(color_count) => *color_count,
            None => {
                let children = rules.get(&node_color).unwrap();
                let mut sum = 0;

                for (child_color, child_count) in children {
                    sum += *child_count as usize
                        + count_children(count_cache, rules, child_color) * *child_count as usize;
                }

                count_cache.insert(node_color.clone(), sum);

                sum
            }
        };

        children
    }

    let mut count_cache = HashMap::new();

    let b = count_children(&mut count_cache, &rules, &gold);

    (a, b)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Color {
    prefix: String,
    primary: String,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.prefix, self.primary)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    bag_color: Color,
    contains: Vec<(Color, u32)>,
}

fn parse_color(input: &str) -> nom::IResult<&str, Color> {
    use nom::bytes::complete::tag;
    use nom::character::complete::alpha1;

    let (input, prefix) = alpha1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, primary) = alpha1(input)?;

    Ok((
        input,
        Color {
            prefix: prefix.to_string(),
            primary: primary.to_string(),
        },
    ))
}

fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;

    let (input, bag_color) = parse_color(input)?;

    let (input, _) = tag(" bags contain ")(input)?;

    let parse_inner_bag = nom::combinator::map(
        nom::sequence::tuple((
            digit1,
            tag(" "),
            parse_color,
            nom::branch::alt((tag(" bags"), tag(" bag"))),
        )),
        |(count, _, color, _)| (color, count.parse::<u32>().unwrap()),
    );

    let (input, contains) = nom::sequence::terminated(
        nom::branch::alt((
            nom::combinator::map(tag("no other bags"), |_| Vec::new()),
            nom::multi::separated_list1(tag(", "), parse_inner_bag),
        )),
        tag("."),
    )(input)?;

    Ok((
        input,
        Rule {
            bag_color,
            contains,
        },
    ))
}
