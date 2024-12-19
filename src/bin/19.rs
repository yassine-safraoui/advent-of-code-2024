use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        let mut towels = Vec::new();
        let mut designs = Vec::new();

        for (line_index, line) in reader.lines().enumerate() {
            if line_index == 1 {
                continue;
            }
            let line = line.expect("reading line");
            if line_index == 0 {
                for towel in line.split(", ") {
                    towels.push(towel.to_string());
                }
                continue;
            }
            designs.push(line);
        }
        let towels = towels;
        let designs = designs;
        let mut queue: LinkedList<usize> = LinkedList::new();
        let mut used_indices: HashSet<usize> = HashSet::new();
        for design in designs {
            for towel in &towels {
                if design.starts_with(towel) {
                    queue.push_back(towel.len());
                    used_indices.insert(towel.len());
                }
            }
            while !queue.is_empty() {
                let index = queue.pop_front().unwrap();
                if index == design.len() {
                    result += 1;
                    break;
                }
                for towel in &towels {
                    if design[index..].starts_with(towel) {
                        let new_index = index + towel.len();
                        if !used_indices.contains(&new_index) {
                            queue.push_back(new_index);
                            used_indices.insert(new_index);
                        }
                    }
                }
            }
            queue.clear();
            used_indices.clear()
        }
        Ok(result)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        let mut towels = Vec::new();
        let mut designs = Vec::new();

        for (line_index, line) in reader.lines().enumerate() {
            if line_index == 1 {
                continue;
            }
            let line = line.expect("reading line");
            if line_index == 0 {
                for towel in line.split(", ") {
                    towels.push(towel.to_string());
                }
                continue;
            }
            designs.push(line);
        }
        let towels = towels;
        let designs = designs;
        let mut queue = PriorityQueue::new();
        let mut used_indices: HashSet<usize> = HashSet::new();
        for design in designs {
            let mut origins_count: Vec<usize> = std::iter::repeat_n(0, design.len() + 1).collect();

            for towel in &towels {
                if design.starts_with(towel) {
                    let index = towel.len();
                    queue.push(index, Reverse(index));
                    used_indices.insert(index);
                    origins_count[index] += 1;
                }
            }
            while !queue.is_empty() {
                let (index, _) = queue.pop().unwrap();
                for towel in &towels {
                    if design[index..].starts_with(towel) {
                        let new_index = index + towel.len();
                        if new_index == design.len() {
                            result += origins_count[index];
                            continue;
                        }
                        if !used_indices.contains(&new_index) {
                            queue.push(new_index, Reverse(new_index));
                            used_indices.insert(new_index);
                        }
                        origins_count[new_index] += origins_count[index];
                    }
                }
            }
            queue.clear();
            used_indices.clear();
        }
        Ok(result)
    }

    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
