use anyhow::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        for line_res in reader.lines() {
            let line = line_res.unwrap();
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<i32>().unwrap());
            right.push(parts.next().unwrap().parse::<i32>().unwrap());
        }

        left.sort();
        right.sort();

        let mut answer = 0;
        for (i, item) in left.iter().enumerate() {
            answer += (item - right[i]).abs();
        }

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut left: Vec<i32> = Vec::new();
        let mut right_counter: HashMap<i32, i32> = HashMap::new();
        for line_res in reader.lines() {
            let line = line_res.unwrap();
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse::<i32>().unwrap());
            right_counter.entry(parts.next().unwrap().parse::<i32>().unwrap()).and_modify(|counter| *counter += 1).or_insert(1);
        }

        let mut answer = 0;
        for n in left {
            if right_counter.contains_key(&n) {
                answer += n * right_counter.get(&n).unwrap();
            }
        }

        Ok(answer)
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
