fn main() {
    let input = include_str!("input1.txt");
    println!("1a {:?}", day1a(input));
    println!("1b {:?}", day1b(input));
}

fn day1a(input: &str) -> Option<i32> {
    let input: Result<Vec<i32>, _> = input.lines().map(str::parse).collect();
    let input = input.unwrap();

    for &a in &input {
        for &b in &input {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

fn day1b(input: &str) -> Option<i32> {
    let input: Result<Vec<i32>, _> = input.lines().map(str::parse).collect();
    let input = input.unwrap();

    for &a in &input {
        for &b in &input {
            for &c in &input {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}
