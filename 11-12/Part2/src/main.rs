use std::{fs, vec};
use std::collections::HashMap;

fn split_int_in_half(num: u64) -> Vec<u64> {
    let num_str = num.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let first_half = &num_str[..mid];
    let second_half = &num_str[mid..];

    return vec![first_half.parse::<u64>().unwrap(), second_half.parse::<u64>().unwrap()];
}

fn blink(mut stone: u64, memo: &mut HashMap<(u64, u32), Vec<u64>>, iteration: u32) -> Vec<u64> {
    if iteration == 0 {
        return vec![stone];
    }

    if let Some(result) = memo.get(&(stone, iteration)) {
        println!("Memoized result for stone {}: {:?}", stone, result);

        return result.clone();
    }

    let next_stones = if stone == 0 {
        vec![1]
    }
    else if stone.to_string().len() % 2 == 0 {
        split_int_in_half(stone)
    }
    else {
        vec![stone * 2024]
    };

    let recursive_result: Vec<u64> = next_stones
        .iter()
        .flat_map(|&s| blink(s, memo, iteration - 1))
        .collect();
    
    //println!("Iteration {}: stone {}, recursive_result {:?}", iteration, stone, recursive_result);

    memo.insert((stone, iteration), recursive_result.clone());

    recursive_result
}

fn main() {
    let file_path = "src/input";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut stones: Vec<u64> = input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut memo: HashMap<(u64, u32), Vec<u64>> = HashMap::new();

    let mut result = 0;
    
    for stone in stones {
        result += blink(stone, &mut memo, 45).len();
        println!("finished with a stone");
    }
    
    println!("result");
    println!("count : {}", result);
}
