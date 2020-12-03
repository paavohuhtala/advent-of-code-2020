use regex;

fn main() {
    let input = include_str!("input2.txt");
    println!("a: {}", day2a(input));
    println!("b: {}", day2b(input));
}

#[derive(Debug)]
struct InputLine {
    range_start: u32,
    range_end: u32,
    letter: char,
    password: String,
}

fn parse_line(line_regex: &regex::Regex, input: &str) -> InputLine {
    let captures = line_regex.captures(input).unwrap();
    let range_start: u32 = str::parse(&captures[1]).unwrap();
    let range_end: u32 = str::parse(&captures[2]).unwrap();
    let letter: char = captures[3].chars().next().unwrap();
    let password = captures[4].to_string();

    InputLine {
        range_start,
        range_end,
        letter,
        password,
    }
}

fn day2a(input: &str) -> usize {
    let line_regex = regex::Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    input
        .lines()
        .map(|line| parse_line(&line_regex, line))
        .filter(|line| {
            let times_in_password = line.password.chars().filter(|&c| c == line.letter).count();
            (line.range_start..=line.range_end).contains(&(times_in_password as u32))
        })
        .count()
}

fn day2b(input: &str) -> usize {
    let line_regex = regex::Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    input
        .lines()
        .map(|line| parse_line(&line_regex, line))
        .filter(|line| {
            let first_matches = line
                .password
                .chars()
                .nth(line.range_start as usize - 1)
                .unwrap()
                == line.letter;
            let second_matches = line
                .password
                .chars()
                .nth(line.range_end as usize - 1)
                .unwrap()
                == line.letter;

            first_matches != second_matches
        })
        .count()
}
