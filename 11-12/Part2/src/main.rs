use std::fs;
use std::collections::HashMap;

fn split_int_in_half(num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let len = num_str.len();
    let mid = len / 2;

    let first_half = &num_str[..mid];
    let second_half = &num_str[mid..];

    return (first_half.parse::<u64>().unwrap(), second_half.parse::<u64>().unwrap());
}

fn blink(stone: u64, memo: &mut HashMap<(u64, u32), u64>, iteration: u32) -> u64 {
    if iteration == 0 {
        //println!("Iteration {}: stone {}, count 1", iteration, stone);

        return 1;
    }

    if let Some(result) = memo.get(&(stone, iteration)) {
        //println!("Memoized result for stone {}: iteration {}", stone, iteration);

        return *result;
    }

    let count = if stone == 0 {
        blink(1, memo, iteration - 1)
    }
    else if stone.to_string().len() % 2 == 0 {
        let stones = split_int_in_half(stone);
        let mut stones_count = blink(stones.0, memo, iteration - 1);
        stones_count += blink(stones.1, memo, iteration - 1);
        stones_count
    }
    else {
        blink(stone * 2024, memo, iteration - 1)
    };

    //println!("Iteration {}: stone {}, count {}", iteration, stone, count);
    
    memo.insert((stone, iteration), count);

    count
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let file_path = "src/input";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let stones: Vec<u64> = input
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut memo: HashMap<(u64, u32), u64> = HashMap::new();

    let mut result = 0;
    
    for stone in stones {
        result += blink(stone, &mut memo, 75);
    }
    
    println!("result");
    println!("count : {}", result);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
