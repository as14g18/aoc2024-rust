use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Vec::new();
        for line in reader.lines().map(|res| res.unwrap()) {
            grid.push(line);
        }

        let n = grid.len();
        let m = grid[0].len();
        let mut answer = 0;
        

        for (i, line) in enumerate(&grid) {
            for (j, c) in enumerate(line.chars()) {
                if c == '0' {
                    let mut stack = Vec::new();
                    stack.push((i, j));
                    let mut seen_9 = HashSet::new();
                    loop {
                        if stack.is_empty() {
                            break;
                        }

                        let top = stack.pop().unwrap();
                        if grid[top.0].bytes().nth(top.1).unwrap() == '9' as u8 {
                            seen_9.insert((top.0, top.1));
                        }
                        for offset in [(-1,0), (0, -1), (0, 1), (1, 0)] {
                            let offset_i = top.0 as i32 + offset.0;
                            let offset_j = top.1 as i32 + offset.1;
                            if offset_i < 0 || offset_j < 0 || offset_i >= n as i32 || offset_j >= m as i32 {
                                continue;
                            }
                            if grid[offset_i as usize].bytes().nth(offset_j as usize).unwrap() == grid[top.0].bytes().nth(top.1).unwrap() + 1 {
                                stack.push((offset_i as usize, offset_j as usize));
                            }
                        }
                    }

                    answer += seen_9.len();
                }
            }
        }

        Ok(answer)
    }



    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Vec::new();
        for line in reader.lines().map(|res| res.unwrap()) {
            grid.push(line);
        }

        let n = grid.len();
        let m = grid[0].len();
        let mut paths = vec![0; n * m];
        for k in (0..9).rev() {
            for (i, line) in enumerate(&grid) {
                for (j, c) in enumerate(line.chars()) {
                    if char::from_digit(k, 10).unwrap() == c {
                        for offset in [(-1,0), (0, -1), (0, 1), (1, 0)] {
                            let offset_i = i as i32 + offset.0;
                            let offset_j = j as i32 + offset.1;
                            if offset_i < 0 || offset_j < 0 || offset_i >= n as i32 || offset_j >= m as i32 {
                                continue;
                            }
                            let offset_c = grid[offset_i as usize].bytes().nth(offset_j as usize).unwrap();
                            if offset_c == char::from_digit(k+1, 10).unwrap() as u8 {
                                if offset_c == '9' as u8 {
                                    paths[offset_i as usize * m + offset_j as usize] = 1;
                                }
                                paths[i * m + j] += paths[offset_i as usize * m + offset_j as usize];
                            }
                        }
                    }
                }
            }
        }
        
        let mut answer = 0;
        for (i, line) in enumerate(&grid) {
            for (j, c) in enumerate(line.chars()) {
                if c == '0' {
                    answer += paths[i * m + j];
                }
            }
        }
        Ok(answer)
    }
    
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
