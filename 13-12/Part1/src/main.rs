use std::fs;
use regex::Regex;

fn get_minimum_cost(button_a_x: i32, button_a_y: i32, button_b_x: i32, button_b_y: i32, prize_x: i32, prize_y: i32) -> i32 {
    let mut dbx = prize_x / button_b_x;
    let mut dby = prize_y / button_b_y;

    println!("dbx: {}, dby: {}", dbx, dby);

    if dbx < dby {
        let mut n = dbx + 1;
        let mut m = 0;
        while n > 0 {
            let mut result = 0;
            m = 0;

            while result < prize_x {
                m += 1;

                result = n * button_b_x + m * button_a_x;
            }

            if result == prize_x {
                if n * button_b_y + m * button_a_y == prize_y {
                    println!("==================================================");
                    println!("n: {}, m: {}, result: {}, intended result: {}", n, m, n * button_b_x + m * button_a_x, prize_x);
                    println!("==================================================");
                    println!("tokens: {}", n + m * 3);
                    return n + m * 3
                }

                result = 0;
                n -= 1;
            }
            else {
                n -= 1;

                println!("n: {}, m: {}, result: {}, intended result: {}", n, m, n * button_b_x + m * button_a_x, prize_x);
            }
        }
    }
    else {
        let mut n = dby + 1;
        let mut m = 0;
        while n > 0 {
            let mut result = 0;
            m = 0;

            while result < prize_y {
                m += 1;

                result = n * button_b_y + m * button_a_y;
            }

            if result == prize_y {
                if n * button_b_x + m * button_a_x == prize_x {
                    println!("==================================================");
                    println!("n: {}, m: {}, result: {}, intended result: {}", n, m, n * button_b_y + m * button_a_y, prize_y);
                    println!("==================================================");
                    println!("tokens: {}", n + m*3);
                    return n + m * 3
                }

                result = 0;
                n -= 1;
            }
            else {
                n -= 1;

                println!("n: {}, m: {}, result: {}, intended result: {}", n, m, n * button_b_y + m * button_a_y, prize_y);
            }
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
            button_a_x.push(caps[1].parse::<i32>().unwrap());
            button_a_y.push(caps[2].parse::<i32>().unwrap());
        } else if let Some(caps) = re_button_b.captures(line) {
            button_b_x.push(caps[1].parse::<i32>().unwrap());
            button_b_y.push(caps[2].parse::<i32>().unwrap());
        } else if let Some(caps) = re_prize.captures(line) {
            prize_x.push(caps[1].parse::<i32>().unwrap());
            prize_y.push(caps[2].parse::<i32>().unwrap());
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
