use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nums: Vec<i32> = Vec::new();

    for arg in args {
        match arg.parse() {
            Ok(n) => nums.push(n),
            Err(_) => continue,
        }
    } 
    nums.sort();
    print!("Numbers sorted: ");
    for (i, n) in nums.iter().enumerate() {
        print!("{n}");
        if i != nums.len() - 1 {
            print!(" ");
        }
    }
    println!();
    println!("Median: {}", median(&nums));
    println!("Mode: {}", mode(&nums));
}

fn median(nums: &[i32]) -> i32 {
    nums[nums.len().div_ceil(2)]
}

fn mode(nums: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        let count = map.entry(*n).or_insert(0);
        *count += 1;
    }
    let max = match map.values().max() {
        Some(v) => *v,
        None => 0,
    };
    for k in map.keys() {
        if map[k] == max {
            return *k;
        }
    }
    0
}

