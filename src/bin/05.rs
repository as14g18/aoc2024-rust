use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut m: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines().map(|res| res.unwrap()) {
            if line.chars().nth(2) == Some('|') {
                let tokens: Vec<u32> = line
                    .split('|')
                    .map(|token| token.parse::<u32>().unwrap())
                    .collect();
                m.entry(tokens[0])
                    .and_modify(|e| e.push(tokens[1]))
                    .or_insert_with(|| vec![tokens[1]]);
            } else if line.chars().nth(2) == Some(',') {
                lines.push(line);
            }
        }

        let mut answer = 0;
        for line in lines {
            let nums: Vec<u32> = line
                .split(',')
                .map(|token| token.parse::<u32>().unwrap())
                .collect();
            let mut is_valid = true;
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if let Some(ret) = m.get(&nums[j]) {
                        if ret.contains(&nums[i]) {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }

            if is_valid {
                answer += nums[(nums.len() - 1) / 2];
            }
        }

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut m: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines().map(|res| res.unwrap()) {
            if line.chars().nth(2) == Some('|') {
                let tokens: Vec<u32> = line
                    .split('|')
                    .map(|token| token.parse::<u32>().unwrap())
                    .collect();
                let a = tokens[0];
                let b = tokens[1];
                m.entry(a)
                    .and_modify(|e| e.push(b))
                    .or_insert_with(|| vec![b]);
            } else if line.chars().nth(2) == Some(',') {
                lines.push(line);
            }
        }

        let mut answer = 0;
        for line in lines {
            let mut nums: Vec<u32> = line
                .split(',')
                .map(|token| token.parse::<u32>().unwrap())
                .collect();
            let mut is_valid = true;
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if let Some(ret) = m.get(&nums[j]) {
                        if ret.contains(&nums[i]) {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }

            let mut graph = DiGraph::<u32, ()>::new();
            let mut node_map = HashMap::new();

            for a in nums.as_slice() {
                let Some(vec_b) = m.get(a) else {
                    continue;
                };
                for b in vec_b.as_slice() {
                    let mut get_or_insert_node = |value| {
                        *node_map
                            .entry(value)
                            .or_insert_with(|| graph.add_node(value))
                    };
                    let node_a = get_or_insert_node(*a);
                    let node_b = get_or_insert_node(*b);
                    graph.add_edge(node_a, node_b, ());
                }
            }

            let correct_order: Vec<&u32> = toposort(&graph, None)
                .unwrap()
                .iter()
                .map(|idx| graph.node_weight(*idx).unwrap())
                .collect();
            let mut weighted_map = HashMap::new();
            for (i, n) in enumerate(correct_order) {
                weighted_map.insert(n, i);
            }

            if !is_valid {
                nums.sort_by(|a, b| weighted_map[a].cmp(&weighted_map[b]));

                answer += nums[(nums.len() - 1) / 2];
            }
        }

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
