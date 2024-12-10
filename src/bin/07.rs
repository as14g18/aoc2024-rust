use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn solve(i: usize, acc: i64, operands: &Vec<i64>, result: i64) -> bool {
        if i >= operands.len() {
            return acc == result;
        }
        
        if acc == 0 {
            return solve(i + 1, operands[i], operands, result);
        } else {
            return solve(i + 1, acc + operands[i], operands, result) || solve(i + 1, acc * operands[i], operands, result);
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let mut answer = 0;
        for line in reader.lines().map(|res| res.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            let result = tokens[0].parse::<i64>().unwrap();
            let operands: Vec<i64> = tokens[1].split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
            if solve(0, 0, &operands, result) {
                answer += result;
            }
        }

        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn solve2(i: usize, acc: u64, operands: &Vec<u64>, result: u64) -> bool {
        if i >= operands.len() {
            return acc == result;
        }
        
        if acc == 0 {
            return solve2(i + 1, operands[i], operands, result);
        } else {
            return solve2(i + 1, acc + operands[i], operands, result) ||
            solve2(i + 1, acc * operands[i], operands, result) ||
            solve2(i + 1, acc * 10u64.pow(operands[i].ilog10() + 1) + operands[i], operands, result);
        }
    }
    
    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut answer = 0;
        for line in reader.lines().map(|res| res.unwrap()) {
            let tokens: Vec<&str> = line.split(": ").collect();
            let result = tokens[0].parse::<u64>().unwrap();
            let operands: Vec<u64> = tokens[1].split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
            if solve2(0, 0, &operands, result) {
                answer += result;
            }
        }

        Ok(answer)
    }
    
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
