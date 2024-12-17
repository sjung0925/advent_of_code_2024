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

    let safe_at_once: usize = data.iter().filter(|report| is_safe(report)).count();

    println!("part 1 result : {:?}", safe_at_once);

    let modified_for_safe: usize = data.iter().filter(|report| can_be_safe(report)).count();

    println!("part 2 result : {:?}", modified_for_safe);
}

fn can_be_safe(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    (0..report.len()).any(|i| {
        let mut mod_report = report.to_vec();
        mod_report.remove(i);
        is_safe(&mod_report)
    })
}
fn is_safe(report: &Vec<i32>) -> bool {
    // 증가하거나 감소
    let is_increasing = is_increasing(report);
    let is_decreasing = is_decreasing(report);

    // 인접한 값의 차가 1에서 3 사이 값
    let diff = is_diff_in_range(report);

    (is_increasing || is_decreasing) && diff
}

fn is_increasing(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

fn is_diff_in_range(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| {
        let diff = if w[0] - w[1] < 0 {
            w[1] - w[0]
        } else {
            w[0] - w[1]
        };
        if diff >= 1 && diff <= 3 {
            true
        } else {
            false
        }
    })
}
