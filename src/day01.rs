use std::{fs, i32};

pub fn solve() {
    println!("Day 1 solution start *****");
    let input = fs::read_to_string("input/day01.txt").expect("failed to read input file");
    // println!("Input : {}", input);
    // 데이터를 벡터로 추출
    let (col1, col2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    // 정렬
    let mut sorted_col1 = col1.clone();
    let mut sorted_col2 = col2.clone();

    sorted_col1.sort();
    sorted_col2.sort();

    // 두 벡터 간 차이 계산
    let diff: Vec<i32> = sorted_col1
        .iter()
        .zip(&sorted_col2)
        .map(|(x, y)| if x - y < 0 { (x - y).abs() } else { x - y })
        .collect();

    // 합계
    let res: i32 = diff.iter().sum();
    println!("result: {:?}", res);
}
