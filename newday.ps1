$date = (Get-Date).Day

$template = @"
fn main() {
    let input = include_str!("./input$date.txt");
    println!("a: {:?}", day${date}a(input));
    println!("b: {:?}", day${date}b(input));
}

fn day${date}a(input: &str) {}

fn day${date}b(input: &str) {}
"@

Set-Content -Path ./src/bin/day$date.rs -Value ($template)
New-Item -Force -ItemType "file" -Path ./src/bin/input$date.txt
