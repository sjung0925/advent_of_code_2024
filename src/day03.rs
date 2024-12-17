use regex::Regex;
use std::fs;

pub fn solve() {
    println!("day3 **********");

    let raw_data = fs::read_to_string("input/day03.txt").expect("failed to read input file");
    // mul(x,y) 형식
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;

    for t in re.captures_iter(&raw_data) {
        let x: i32 = t[1].parse().unwrap();
        let y: i32 = t[2].parse().unwrap();

        total_sum += x * y;
    }

    println!("my answer is {}", total_sum);
}
