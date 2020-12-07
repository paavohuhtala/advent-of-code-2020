use std::collections::HashSet;

use nom;

fn main() {
    let input = include_str!("input7.txt");
    day7a();
}

fn day7a() {}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Color {
    prefix: String,
    primary: String,
}

struct Rule {
    bag_color: Color,
    contains: HashSet<(Color, u32)>,
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

fn parse_bag_contents(input: &str) -> nom::IResult<&str, Vec<(Color, u32)>> {
    Ok((input, Vec::new()))
}

fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;

    let (input, bag_color) = parse_color(input)?;

    let (input, _) = tag(" bags contain")(input)?;

    let parse_sub_bag = nom::sequence::tuple((
        
    ))

    let hmmm = nom::branch::alt(
        (
            nom::combinator::map(tag(" no other bags."), |_| Vec::new()),
            nom::multi::separated_list1(tag(", "), )
        )
    )
}
