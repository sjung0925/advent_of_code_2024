use std::fs;

pub fn solve() {
    println!("day2 ***** ");
    let input = fs::read_to_string("input/day02.txt").expect("failed to read input file");
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("parse error"))
                .collect()
        })
        .collect();

    let safe: Vec<bool> = data.iter().map(|report| is_safe(report)).collect();

    let count = safe.iter().filter(|&&is_safe| is_safe).count();
    println!("result : {:?}", count);
}
fn is_safe(report: &Vec<i32>) -> bool {
    // 증가하거나 감소
    let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);

    // 인접한 값의 차가 1에서 3 사이 값
    let diff = report.windows(2).all(|w| {
        let diff = if w[0] - w[1] < 0 {
            w[1] - w[0]
        } else {
            w[0] - w[1]
        };
        if diff > 0 && diff < 4 {
            true
        } else {
            false
        }
    });

    (is_increasing || is_decreasing) && diff
}
