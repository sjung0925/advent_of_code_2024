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

    let p1_ans = part1(&col1, &col2);
    println!("### part 1 answer is {:?}", p1_ans);

    let p2_ans = part2(&col1, &col2);
    println!("### part 2 answer is {:?}", p2_ans);
}

fn part1(c1: &Vec<i32>, c2: &Vec<i32>) -> i32 {
    // 정렬
    let mut sorted_col1 = c1.clone();
    let mut sorted_col2 = c2.clone();

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

    res
}

fn part2(c1: &Vec<i32>, c2: &Vec<i32>) -> i32 {
    let mut total_sum = 0;
    // 컬럼 1을 순회
    for n in c1.iter() {
        // 현재 컬럼 1의 값과 동일 한 값이 컬럼 2에 몇개 있는지 확인
        let mut count = 0;
        for m in c2.iter() {
            if n == m {
                count += 1
            }
        }
        // 현재 컬럼 1값 * 컬럼 2 속 개수
        // 토탈 합계
        total_sum += n * count;
    }
    total_sum
}
