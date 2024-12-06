use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines().map(|res| res.unwrap()) {
            lines.push(line);
        }

        fn get_char(str: &Vec<String>, i: usize, j: usize) -> char {
            str[i].chars().nth(j).unwrap()
        }

        let mut answer = 0;
        let n = lines.len();
        let m = lines[0].len();
        for i in 0..n {
            for j in 0..m {
                if get_char(&lines, i, j) == 'X' {
                    if j + 3 < m
                        && get_char(&lines, i, j + 1) == 'M'
                        && get_char(&lines, i, j + 2) == 'A'
                        && get_char(&lines, i, j + 3) == 'S'
                    {
                        answer += 1
                    }
                    if j >= 3
                        && get_char(&lines, i, j - 1) == 'M'
                        && get_char(&lines, i, j - 2) == 'A'
                        && get_char(&lines, i, j - 3) == 'S'
                    {
                        answer += 1
                    }

                    if i + 3 < n
                        && get_char(&lines, i + 1, j) == 'M'
                        && get_char(&lines, i + 2, j) == 'A'
                        && get_char(&lines, i + 3, j) == 'S'
                    {
                        answer += 1
                    }
                    if i >= 3
                        && get_char(&lines, i - 1, j) == 'M'
                        && get_char(&lines, i - 2, j) == 'A'
                        && get_char(&lines, i - 3, j) == 'S'
                    {
                        answer += 1
                    }

                    if i + 3 < n
                        && j + 3 < m
                        && get_char(&lines, i + 1, j + 1) == 'M'
                        && get_char(&lines, i + 2, j + 2) == 'A'
                        && get_char(&lines, i + 3, j + 3) == 'S'
                    {
                        answer += 1
                    }
                    if i >= 3
                        && j >= 3
                        && get_char(&lines, i - 1, j - 1) == 'M'
                        && get_char(&lines, i - 2, j - 2) == 'A'
                        && get_char(&lines, i - 3, j - 3) == 'S'
                    {
                        answer += 1
                    }

                    if i + 3 < n
                        && j >= 3
                        && get_char(&lines, i + 1, j - 1) == 'M'
                        && get_char(&lines, i + 2, j - 2) == 'A'
                        && get_char(&lines, i + 3, j - 3) == 'S'
                    {
                        answer += 1
                    }
                    if i >= 3
                        && j + 3 < m
                        && get_char(&lines, i - 1, j + 1) == 'M'
                        && get_char(&lines, i - 2, j + 2) == 'A'
                        && get_char(&lines, i - 3, j + 3) == 'S'
                    {
                        answer += 1
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines().map(|res| res.unwrap()) {
            lines.push(line);
        }

        fn get_char(str: &Vec<String>, i: usize, j: usize) -> char {
            str[i].chars().nth(j).unwrap()
        }

        let mut answer = 0;
        let n = lines.len();
        let m = lines[0].len();
        for i in 0..n {
            for j in 0..m {
                if get_char(&lines, i, j) == 'A' {
                    if i >= 1 && j+1 < m && get_char(&lines, i-1, j+1) == 'S' && // Top right
                    i >= 1 && j >= 1 && get_char(&lines, i-1, j-1) == 'M' &&     // Top left
                    i+1 < n && j >= 1 && get_char(&lines, i+1, j-1) == 'M' &&    // Bottom left
                    i+1 < n && j+1 < m && get_char(&lines, i+1, j+1) == 'S'
                    // Bottom right
                    {
                        answer += 1;
                    }
                    if i >= 1 && j+1 < m && get_char(&lines, i-1, j+1) == 'S' && // Top right
                    i >= 1 && j >= 1 && get_char(&lines, i-1, j-1) == 'S' &&     // Top left
                    i+1 < n && j >= 1 && get_char(&lines, i+1, j-1) == 'M' &&    // Bottom left
                    i+1 < n && j+1 < m && get_char(&lines, i+1, j+1) == 'M'
                    // Bottom right
                    {
                        answer += 1;
                    }
                    if i >= 1 && j+1 < m && get_char(&lines, i-1, j+1) == 'M' && // Top right
                    i >= 1 && j >= 1 && get_char(&lines, i-1, j-1) == 'S' &&     // Top left
                    i+1 < n && j >= 1 && get_char(&lines, i+1, j-1) == 'S' &&    // Bottom left
                    i+1 < n && j+1 < m && get_char(&lines, i+1, j+1) == 'M'
                    // Bottom right
                    {
                        answer += 1;
                    }
                    if i >= 1 && j+1 < m && get_char(&lines, i-1, j+1) == 'M' && // Top right
                    i >= 1 && j >= 1 && get_char(&lines, i-1, j-1) == 'M' &&     // Top left
                    i+1 < n && j >= 1 && get_char(&lines, i+1, j-1) == 'S' &&    // Bottom left
                    i+1 < n && j+1 < m && get_char(&lines, i+1, j+1) == 'S'
                    // Bottom right
                    {
                        answer += 1;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
