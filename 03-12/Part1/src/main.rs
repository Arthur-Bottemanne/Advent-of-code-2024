use std::fs;
use regex::Regex;

fn main() {
    let file_path = "src/input";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{input}");

    let mut result = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for captures in re.captures_iter(&input) {
        let number1_string = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let number2_string = captures.get(2).map(|m| m.as_str()).unwrap_or("");

        let number1: i32 = number1_string.parse().unwrap();
        let number2: i32 = number2_string.parse().unwrap();

        result += number1 * number2;
    }

    println!("Result: {}", result);
}
