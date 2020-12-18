use std::collections::VecDeque;

use regex::Regex;

fn main() {
    let input = include_str!("./input18.txt");
    println!("a: {:?}", day18a(input));
    println!("b: {:?}", day18b(input));
}

#[derive(Debug)]
enum OpCode {
    Constant(i64),
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum OpStackEntry {
    Add,
    Mul,
    LParen,
}

impl OpStackEntry {
    fn is_op(self) -> bool {
        match self {
            OpStackEntry::Add | OpStackEntry::Mul => true,
            _ => false,
        }
    }
}

fn parse_tokens(tokens: &[&str], get_precedence: impl Fn(OpStackEntry) -> u8) -> VecDeque<OpCode> {
    let mut output_queue = VecDeque::<OpCode>::new();
    let mut op_stack = VecDeque::<OpStackEntry>::new();

    let mut token_iterator = tokens.iter();

    while let Some(&token) = token_iterator.next() {
        match token {
            number if number.starts_with(|ch: char| ch.is_ascii_digit()) => {
                output_queue.push_back(OpCode::Constant(number.parse().unwrap()));
            }
            "+" | "*" => {
                let op = match token {
                    "+" => OpStackEntry::Add,
                    "*" => OpStackEntry::Mul,
                    _ => unreachable!(),
                };

                let precedence = get_precedence(op);

                while let Some(stack_op) = op_stack.front() {
                    if stack_op.is_op() && get_precedence(*stack_op) >= precedence {
                        match op_stack.pop_front().unwrap() {
                            OpStackEntry::Add => output_queue.push_back(OpCode::Add),
                            OpStackEntry::Mul => output_queue.push_back(OpCode::Mul),
                            _ => unreachable!(),
                        }
                    } else {
                        break;
                    }
                }

                match token {
                    "+" => op_stack.push_front(OpStackEntry::Add),
                    "*" => op_stack.push_front(OpStackEntry::Mul),
                    _ => unreachable!(),
                };
            }
            "(" => {
                op_stack.push_front(OpStackEntry::LParen);
            }
            ")" => {
                while let Some(OpStackEntry::Add) | Some(OpStackEntry::Mul) = op_stack.front() {
                    match op_stack.pop_front().unwrap() {
                        OpStackEntry::Add => output_queue.push_back(OpCode::Add),
                        OpStackEntry::Mul => output_queue.push_back(OpCode::Mul),
                        _ => unreachable!(),
                    };
                }

                match op_stack.front() {
                    Some(OpStackEntry::LParen) => {
                        op_stack.pop_front();
                    }
                    _ => {}
                };
            }
            _ => panic!(),
        }
    }

    while let Some(op) = op_stack.pop_front() {
        match op {
            OpStackEntry::Add => output_queue.push_back(OpCode::Add),
            OpStackEntry::Mul => output_queue.push_back(OpCode::Mul),
            _ => unreachable!(),
        };
    }

    output_queue
}

fn tokenize<'a>(token_regex: &regex::Regex, input: &'a str) -> Vec<&'a str> {
    token_regex.find_iter(input).map(|s| s.as_str()).collect()
}

fn eval_program(program: VecDeque<OpCode>) -> i64 {
    let mut stack = VecDeque::new();

    for op in program {
        match op {
            OpCode::Constant(i) => {
                stack.push_front(i);
            }
            OpCode::Add => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                stack.push_front(a + b);
            }
            OpCode::Mul => {
                let a = stack.pop_front().unwrap();
                let b = stack.pop_front().unwrap();
                stack.push_front(a * b);
            }
        }
    }

    stack.pop_front().unwrap()
}

fn tokenize_lines<'a>(input: &'a str) -> impl Iterator<Item = Vec<&'a str>> {
    let token_regex = Regex::new(r"([0-9]+|\+|\*|\(|\))").unwrap();
    input.lines().map(move |line| tokenize(&token_regex, line))
}

fn day18a(input: &str) -> i64 {
    tokenize_lines(input)
        .map(|tokens| parse_tokens(&tokens, |_| 1))
        .map(eval_program)
        .sum()
}

fn day18b(input: &str) -> i64 {
    tokenize_lines(input)
        .map(|tokens| {
            parse_tokens(&tokens, |op| match op {
                OpStackEntry::Add => 2,
                OpStackEntry::Mul => 1,
                _ => panic!(),
            })
        })
        .map(eval_program)
        .sum()
}
