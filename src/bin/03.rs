use anyhow::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let hay = reader.lines().flatten().next().unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut muls: Vec<(i32, i32)> = vec![];
        for (_, [lhs, rhs]) in re.captures_iter(&hay).map(|c| c.extract()) {
            muls.push((lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()));
        }

        let mut answer = 0;
        for (lhs, rhs) in muls {
            answer += lhs * rhs;
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let hay = reader.lines().flatten().next().unwrap();
        let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
        let mut muls: Vec<(i32, i32)> = vec![];
        let mut enabled = true;

        for caps in re.captures_iter(&hay) {
            if let Some(match_) = caps.get(1) {
                if match_.as_str() == "do()" {
                    enabled = true;
                } else if match_.as_str() == "don't()" {
                    enabled = false;
                } else if enabled {
                    muls.push((caps.get(2).unwrap().as_str().parse::<i32>().unwrap(), caps.get(3).unwrap().as_str().parse::<i32>().unwrap()));
                }
            }
        }

        let mut answer = 0;
        for (lhs, rhs) in muls {
            answer += lhs * rhs;
        }

        Ok(answer)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
