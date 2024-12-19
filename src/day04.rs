use std::fs;

pub fn solve() {
    println!("Day 4 code here!");
    // Day 4의 코드

    // 파일 읽기
    let raw_data = fs::read_to_string("input/day04.txt").expect("failed to read input file");
    let grid: Vec<Vec<char>> = raw_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let direction: Vec<(isize, isize)> = vec![
        (0, 1),   // 오른쪽
        (0, -1),  // 왼쪽
        (-1, 0),  // 위쪽
        (1, 0),   // 아래쪽
        (-1, -1), // 왼쪽 위 대각선
        (1, -1),  // 오른쪽 위 대각선
        (-1, 1),  // 왼쪽 아래 대각선
        (1, 1),   // 오른쪽 아래 대각선
    ];
    let word = "XMAS";
    let word_char: Vec<char> = word.chars().collect();
    let mut count = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'X' {
                // 8 방향으로
                for (delta_row, delta_col) in direction.iter() {
                    let mut check = true;

                    for i in 0..word_char.len() {
                        let new_row = r as isize + delta_row * i as isize;
                        let new_col = c as isize + delta_col * i as isize;

                        // 벡터 범위를 벗어나지 않으면 진행
                        if !(new_row >= 0
                            && new_row < grid.len() as isize
                            && new_col >= 0
                            && new_col < grid[r].len() as isize)
                        {
                            check = false;
                            break;
                        }

                        if grid[new_row as usize][new_col as usize] != word_char[i] {
                            check = false;
                            break;
                        }
                    }
                    if check {
                        count += 1;
                        println!(
                            "Found '{}' at ({}, {}) in direction {:?}",
                            word,
                            r,
                            c,
                            (delta_row, delta_col)
                        );
                    }
                }
            }
        }
    }
    println!("part 1 answer is {:?}", count);
}
