use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn is_safe(nums: &Vec<i32>) -> bool {
        let mut prev: Option<i32> = None;
        let mut safe = true;
        let mut ascending = true;
        for pair in nums.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let diff = a - b;
            if diff == 0 || diff.abs() > 3 {
                safe = false;
                break;
            }

            if prev.is_none() {
                ascending = diff > 0;
            } else {
                if (ascending && diff < 0) || (!ascending && diff > 0) {
                    safe = false;
                    break;
                }
            }

            prev = Some(diff);
        }

        return safe;
    }

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut answer = 0;
        for line in reader.lines().map(|res| res.unwrap()) {
            let nums: Vec<i32> = line.split_whitespace().map(|token| token.parse::<i32>().unwrap()).collect();
            if is_safe(&nums) {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut answer = 0;
        for line in reader.lines().map(|res| res.unwrap()) {
            let nums: Vec<i32> = line.split_whitespace().map(|token| token.parse::<i32>().unwrap()).collect();
            if is_safe(&nums) {
                answer += 1;
            } else {
                for i in 0..nums.len() {
                    let mut new_vec = nums.clone();
                    new_vec.remove(i);
                    let result = is_safe(&new_vec);
                    if result {
                        answer += 1;
                        break;
                    }
                }
            }
        }

        Ok(answer)
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
