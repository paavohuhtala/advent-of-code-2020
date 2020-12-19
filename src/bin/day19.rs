use std::collections::HashMap;

fn main() {
    let input = include_str!("./input19.rules.txt");
    let mut rules = parse_rules(input);

    rules.insert(8, Rule::Many1(Box::new(MaybeRefRule::Ref(42))));

    rules.insert(
        11,
        Rule::Seq(vec![
            MaybeRefRule::Owned(Rule::Many1(Box::new(MaybeRefRule::Ref(42)))),
            MaybeRefRule::Ref(31),
        ]),
    );

    /*rules.insert(
        0,
        Rule::Seq(vec![
            MaybeRefRule::Owned(Rule::Many1(Box::new(MaybeRefRule::Owned(Rule::Terminal(
                'a',
            ))))),
            MaybeRefRule::Owned(Rule::Many1(Box::new(MaybeRefRule::Owned(Rule::Terminal(
                'b',
            ))))),
        ]),
    );*/

    let root_rule = rules.get(&0).unwrap();

    let messages: Vec<Vec<char>> = include_str!("./input19.examples.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut matches = 0;

    for message in messages {
        match test_rule(&message, root_rule, &rules) {
            Some(&[]) => {
                matches += 1;
            }
            None | Some(_) => {}
        }
    }

    println!("{}", matches);

    println!("a: {:?}", day19a(input));
    println!("b: {:?}", day19b(input));
}

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    Seq(Vec<MaybeRefRule>),
    Any(Vec<MaybeRefRule>),
    Many1(Box<MaybeRefRule>),
}

#[derive(Debug, Clone)]
enum MaybeRefRule {
    Owned(Rule),
    Ref(u32),
}

fn parse_rules(input: &str) -> HashMap<u32, Rule> {
    fn parse_line(line: &str) -> (u32, Rule) {
        let id = line.split(":").next().unwrap().parse::<u32>().unwrap();
        let line = line.split(": ").skip(1).next().unwrap();

        if line.contains("|") {
            let parts: Vec<MaybeRefRule> = line
                .split(" | ")
                .map(|branch| {
                    MaybeRefRule::Owned(Rule::Seq(
                        branch
                            .split(" ")
                            .map(|num| num.parse::<u32>().unwrap())
                            .map(MaybeRefRule::Ref)
                            .collect::<Vec<_>>(),
                    ))
                })
                .collect();
            (id, Rule::Any(parts))
        } else if line.contains('"') {
            let terminal = line
                .split('"')
                .skip(1)
                .next()
                .unwrap()
                .chars()
                .next()
                .unwrap();
            (id, Rule::Terminal(terminal))
        } else {
            let rule = Rule::Seq(
                line.split(" ")
                    .map(|num| MaybeRefRule::Ref(num.parse().unwrap()))
                    .collect(),
            );

            (id, rule)
        }
    }

    input.lines().map(parse_line).collect()
}

fn resolve_rule<'a>(rule: &'a MaybeRefRule, rules: &'a HashMap<u32, Rule>) -> &'a Rule {
    match rule {
        MaybeRefRule::Owned(rule) => rule,
        MaybeRefRule::Ref(id) => rules.get(id).unwrap(),
    }
}

fn test_rule<'a>(
    input: &'a [char],
    rule: &Rule,
    all_rules: &HashMap<u32, Rule>,
) -> Option<&'a [char]> {
    if input.len() == 0 {
        return None;
    }

    let head = input[0];
    let tail = &input[1..];

    match rule {
        Rule::Terminal(ch) if head == *ch => Some(tail),
        Rule::Terminal(_) => None,
        Rule::Seq(rules) => {
            let mut input = input;
            for rule in rules {
                let rule = resolve_rule(rule, all_rules);
                match test_rule(input, rule, all_rules) {
                    Some(remaining_input) => input = remaining_input,
                    None => {
                        return None;
                    }
                }
            }

            Some(input)
        }
        Rule::Any(rules) => {
            for rule in rules {
                let rule = resolve_rule(rule, all_rules);
                match test_rule(input, rule, all_rules) {
                    Some(remaining_input) => return Some(remaining_input),
                    None => {}
                }
            }

            None
        }
        Rule::Many1(rule) => {
            let rule = resolve_rule(rule, all_rules);

            match test_rule(input, rule, all_rules) {
                None => return None,
                Some(mut remaining_input) => loop {
                    match test_rule(remaining_input, rule, all_rules) {
                        None => {
                            return Some(remaining_input);
                        }
                        Some(&[]) => {
                            return Some(&[]);
                        }
                        Some(tail) => {
                            remaining_input = tail;
                        }
                    }
                },
            }
        }
    }
}

fn day19a(input: &str) {}

fn day19b(input: &str) {}
