use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    enum Dirs {
        Up,
        Down,
        Left,
        Right,
    }

    const OFFSETS: [(Dirs, (i32, i32)); 4] = [
        (Dirs::Up, (-1, 0)),
        (Dirs::Down, (1, 0)),
        (Dirs::Left, (0, -1)),
        (Dirs::Right, (0, 1)),
    ];

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut grid = Vec::new();
        let mut guard_pos_res = None;
        let mut m = 0;
        let mut n = 0;
        for (i, line) in enumerate(reader.lines().map(|res| res.unwrap())) {
            if m == 0 {
                m = line.len() as i32;
            }

            for c in line.chars() {
                grid.push(c);
            }
            if let Some(j) = line.find('^') {
                guard_pos_res = Some((i, j));
            }

            n = (i + 1) as i32;
        }

        let pos_usize = guard_pos_res.unwrap();
        let mut pos = (pos_usize.0 as i32, pos_usize.1 as i32);
        let mut dir = Dirs::Up;
        loop {
            grid[(pos.0 * m + pos.1) as usize] = 'X';

            if (dir == Dirs::Up && pos.0 == 0)
                || (dir == Dirs::Right && pos.1 == m - 1)
                || (dir == Dirs::Down && pos.0 == n - 1)
                || (dir == Dirs::Left && pos.1 == 0)
            {
                break;
            }

            if let Some(value) = OFFSETS.iter().find(|&(k, _)| k == &dir).map(|&(_, v)| v) {
                let new_pos = (pos.0 + value.0, pos.1 + value.1);
                if grid[(new_pos.0 * m + new_pos.1) as usize] == '#' {
                    if dir == Dirs::Up {
                        dir = Dirs::Right
                    } else if dir == Dirs::Right {
                        dir = Dirs::Down
                    } else if dir == Dirs::Down {
                        dir = Dirs::Left
                    } else if dir == Dirs::Left {
                        dir = Dirs::Up
                    }
                } else {
                    pos = new_pos;
                }
            }
        }

        let mut answer = 0;
        for c in grid {
            if c == 'X' {
                answer += 1;
            }
        }

        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Vec::new();
        let mut guard_pos_res = None;
        let mut m = 0;
        let mut n = 0;
        for (i, line) in enumerate(reader.lines().map(|res| res.unwrap())) {
            if m == 0 {
                m = line.len() as i32;
            }

            for c in line.chars() {
                grid.push(c);
            }
            if let Some(j) = line.find('^') {
                guard_pos_res = Some((i, j));
            }

            n = (i + 1) as i32;
        }

        let pos_usize = guard_pos_res.unwrap();
        let mut answer = 0;
        for obst_i in 0..n {
            for obst_j in 0..m {
                if grid[(obst_i * m + obst_j) as usize] == '.' {
                    grid[(obst_i * m + obst_j) as usize] = 'O';
                } else {
                    continue;
                }

                let mut seen_obsts = HashSet::new();
                let mut pos = (pos_usize.0 as i32, pos_usize.1 as i32);
                let mut dir = Dirs::Up;
                loop {
                    if (dir == Dirs::Up && pos.0 == 0)
                        || (dir == Dirs::Right && pos.1 == m - 1)
                        || (dir == Dirs::Down && pos.0 == n - 1)
                        || (dir == Dirs::Left && pos.1 == 0)
                    {
                        break;
                    }

                    if let Some(value) = OFFSETS.iter().find(|&(k, _)| k == &dir).map(|&(_, v)| v) {
                        let new_pos = (pos.0 + value.0, pos.1 + value.1);
                        let cur_char = grid[(new_pos.0 * m + new_pos.1) as usize];
                        if cur_char == '#' || cur_char == 'O' {
                            if seen_obsts.contains(&(new_pos.0, new_pos.1, dir)) {
                                answer += 1;
                                break;
                            } else {
                                seen_obsts.insert((new_pos.0, new_pos.1, dir));
                            }
                            if dir == Dirs::Up {
                                dir = Dirs::Right
                            } else if dir == Dirs::Right {
                                dir = Dirs::Down
                            } else if dir == Dirs::Down {
                                dir = Dirs::Left
                            } else if dir == Dirs::Left {
                                dir = Dirs::Up
                            }
                        } else {
                            pos = new_pos;
                        }
                    }
                }

                grid[(obst_i * m + obst_j) as usize] = '.';
            }
        }

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
