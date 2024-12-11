use std::{fs, vec};

fn split_int_in_half(num: u64) -> Vec<u64> {
    let num_str = num.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let first_half = &num_str[..mid];
    let second_half = &num_str[mid..];

    return vec![first_half.parse::<u64>().unwrap(), second_half.parse::<u64>().unwrap()];
}

fn blink(mut stone: u64, iteration: u32) -> Vec<u64> {
    let mut result = vec![0];

    if stone == 0 {
        stone = 1;
        result = vec![stone];
    }
    else if stone.to_string().len() % 2 == 0 {
        result = split_int_in_half(stone);
    }
    else {
        stone *= 2024;
        result = vec![stone];
    }

    return result;
}

fn main() {
    let file_path = "src/input";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut stones: Vec<u64> = input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    
    for i in 0..75 {
        let mut new_stones = vec![];

        for stone in stones {
            new_stones.extend(blink(stone));
        }

        stones = new_stones;

        println!("iteration : {}", i + 1);
        println!("count : {}", stones.len());
    }
    
    println!("result");
    println!("count : {}", stones.len());
}
