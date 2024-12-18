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
    println!("Part 1 answer is {}", total_sum);

    let do_or_dont_re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut seleted_sum = 0;
    let mut enable = true;

    for t in do_or_dont_re.captures_iter(&raw_data) {
        if let Some(_mul) = t.get(1) {
            if enable {
                let x2: i32 = t[2].parse().unwrap();
                let y2: i32 = t[3].parse().unwrap();

                seleted_sum += x2 * y2;
            }
        }

        if let Some(_do_mul) = t.get(4) {
            enable = true;
        }

        if let Some(_do_not_mul) = t.get(5) {
            enable = false;
        }
    }
    println!("Part 2 answer is {}", seleted_sum);
}
