use std::fs;
use regex::Regex;

fn get_minimum_cost(button_a_x: i128, button_a_y: i128, button_b_x: i128, button_b_y: i128, prize_x: i128, prize_y: i128) -> i128 {
    let determinant = button_a_x * button_b_y - button_b_x * button_a_y;
    let n = (prize_x * button_b_y - button_b_x * prize_y) / determinant;
    let m = (prize_y * button_a_x - button_a_y * prize_x) / determinant;

    println!("n: {}, m: {}, result: {}, result_wehave: {}", n, m, n * button_a_x + m * button_b_x, prize_x);

    if determinant == 0 {
        return 0
    }

    if n * button_a_x + m * button_b_x == prize_x {
        println!("prize 1 correct");

        if n * button_a_y + m * button_b_y == prize_y {
            println!("prize 2 correct");

            return n * 3 + m
        }
    }

    0
}

fn main() {
    let file_path = "src/input";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut button_a_x = Vec::new();
    let mut button_a_y = Vec::new();
    let mut button_b_x = Vec::new();
    let mut button_b_y = Vec::new();
    let mut prize_x = Vec::new();
    let mut prize_y = Vec::new();

    for line in input.lines() {
        if let Some(caps) = re_button_a.captures(line) {
            button_a_x.push(caps[1].parse::<i128>().unwrap());
            button_a_y.push(caps[2].parse::<i128>().unwrap());
        } else if let Some(caps) = re_button_b.captures(line) {
            button_b_x.push(caps[1].parse::<i128>().unwrap());
            button_b_y.push(caps[2].parse::<i128>().unwrap());
        } else if let Some(caps) = re_prize.captures(line) {
            prize_x.push(caps[1].parse::<i128>().unwrap() + 10000000000000);
            prize_y.push(caps[2].parse::<i128>().unwrap() + 10000000000000);
        }
    }

    let mut cost = 0;

    for i in 0..prize_x.len() {
        println!("==================================================");
        println!("Starting new prize count: {}", i + 1);
        println!("==================================================");
        cost += get_minimum_cost(button_a_x[i], button_a_y[i], button_b_x[i], button_b_y[i], prize_x[i], prize_y[i]);
        println!("current tokens: {}", cost);
    }
    
    println!("total cost: {}", cost);

}
