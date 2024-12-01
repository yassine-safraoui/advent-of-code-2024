use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("read input");
        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();
        for line in input.lines() {
            let mut numbers = line.split_whitespace();
            let first_number = numbers.next().expect("Parsing first number from line");
            let second_number = numbers.next().expect("Parsing second number from line");
            left_list.push(first_number.parse::<usize>()?);
            right_list.push(second_number.parse::<usize>()?);
        }
        left_list.sort();
        right_list.sort();
        let mut result = 0;
        if left_list.len() == 0 || right_list.len() == 0 {
            return Err(anyhow!("Empty input, not possible"));
        }
        for (l, r) in left_list.iter().zip(right_list.iter()) {
            result += l.abs_diff(*r);
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("read input");

        let mut left_list: Vec<usize> = Vec::new();
        let mut right_list: Vec<usize> = Vec::new();
        for line in input.lines() {
            let mut numbers = line.split_whitespace();
            let first_number = numbers.next().expect("Parsing first number from line");
            let second_number = numbers.next().expect("Parsing second number from line");
            left_list.push(first_number.parse::<usize>()?);
            right_list.push(second_number.parse::<usize>()?);
        }
        if left_list.len() == 0 || right_list.len() == 0 {
            return Err(anyhow!("Empty input, not possible"));
        }
        let mut numbers_usage: HashMap<usize, usize> = HashMap::new();
        for number in right_list {
            let count = numbers_usage.entry(number).or_insert(0);
            *count += 1
        }
        let mut similarity = 0;
        for number in left_list {
            similarity += numbers_usage.get(&number).copied().unwrap_or(0) * number
        }
        Ok(similarity)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
