use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
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
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    enum OPERATION {
        MUL,
        ADD,
    }
    fn is_target_atteinable(target: usize, numbers: Vec<usize>) -> bool {
        let operations_len = numbers.len() - 1;
        let mut combinations: Vec<Vec<OPERATION>> = Vec::with_capacity(1 << operations_len);
        for i in 0..(1 << operations_len) {
            let mut operation: Vec<OPERATION> = Vec::with_capacity(operations_len);
            for j in 0..operations_len {
                operation.push(if i & (1 << j) != 0 {
                    OPERATION::ADD
                } else {
                    OPERATION::MUL
                })
            }
            combinations.push(operation);
        }
        for combination in combinations {
            let mut result = numbers[0];
            for (index, operation) in combination.iter().enumerate() {
                let number = numbers[index + 1];
                result = match operation {
                    OPERATION::MUL => result * number,
                    OPERATION::ADD => result + number,
                };
                if result > target {
                    break;
                }
            }
            if result == target {
                return true;
            }
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        for line in reader.lines() {
            let line = line.expect("reading line");
            let mut splits = line.split(": ");
            let first_split = splits.next().unwrap();
            let target: usize = first_split.parse()?;
            let numbers = splits
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            if is_target_atteinable(target, numbers) {
                result += target
            }
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    enum OPERATION2 {
        MUL,
        ADD,
        CONCAT,
    }
    fn is_target_atteinable2(target: usize, numbers: Vec<usize>) -> bool {
        let operations_len = numbers.len() - 1;
        let operations_count = 3_usize.pow(operations_len as u32);
        let mut combinations: Vec<Vec<OPERATION2>> = Vec::with_capacity(operations_count);
        for i in 0..operations_count {
            let mut operation: Vec<OPERATION2> = Vec::with_capacity(operations_len);
            let mut value = i;
            for _ in 0..operations_len {
                operation.push(match value % 3 {
                    0 => OPERATION2::ADD,
                    1 => OPERATION2::MUL,
                    2 => OPERATION2::CONCAT,
                    _ => OPERATION2::ADD,
                });
                value /= 3;
            }
            combinations.push(operation);
        }
        println!("{}, {}", operations_count, combinations.len());
        for combination in combinations {
            let mut result = numbers[0];
            for (index, oepration) in combination.iter().enumerate() {
                let number = numbers[index + 1];
                result = match oepration {
                    OPERATION2::MUL => result * number,
                    OPERATION2::ADD => result + number,
                    OPERATION2::CONCAT => {
                        let number_digits_count = (number as f64).log10() as u32;
                        result * 10usize.pow(number_digits_count + 1) + number
                    }
                };
                if result > target {
                    break;
                }
            }
            if result == target {
                return true;
            }
        }
        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        for line in reader.lines() {
            let line = line.expect("reading line");
            let mut splits = line.split(": ");
            let first_split = splits.next().unwrap();
            let target: usize = first_split.parse()?;
            let numbers = splits
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            if is_target_atteinable2(target, numbers) {
                result += target
            }
        }
        Ok(result)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
