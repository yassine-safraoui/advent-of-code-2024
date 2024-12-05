use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
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

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut result: usize = 0;
        let mut parsing_rules = true;
        let mut rules: HashSet<(usize, usize)> = HashSet::new();
        'lines: for line in reader.lines() {
            let line = line.expect("reading line");
            if parsing_rules {
                if line == "" {
                    parsing_rules = false;
                    continue;
                }
                let mut numbers = line.split("|").into_iter();
                let n1 = numbers
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("parsing constraint");
                let n2 = numbers
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("parsing constraint");
                rules.insert((n1, n2));
            } else {
                let numbers: Vec<usize> = line
                    .split(",")
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect();
                let mut past_numbers: HashSet<usize> = HashSet::new();
                past_numbers.insert(numbers[0]);
                for (_, &number) in numbers
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != 0)
                {
                    for past_number in past_numbers.clone() {
                        if !rules.contains(&(past_number, number)) {
                            continue 'lines;
                        }
                    }
                    past_numbers.insert(number);
                }
                result += numbers[numbers.len() / 2]
            }
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut parsing_rules = true;
        let mut rules: HashSet<(usize, usize)> = HashSet::new();
        for line in reader.lines() {
            let line = line.expect("reading line");
            if parsing_rules {
                if line == "" {
                    parsing_rules = false;
                    continue;
                }
                let mut numbers = line.split("|").into_iter();
                let n1 = numbers
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("parsing constraint");
                let n2 = numbers
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .expect("parsing constraint");
                rules.insert((n1, n2));
            } else {
                let mut numbers: Vec<usize> = line
                    .split(",")
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect();
                let mut past_numbers: HashSet<(usize, usize)> = HashSet::new();
                let mut index = 1;
                past_numbers.insert((0, numbers[0]));
                let mut line_broken = false;
                'while_loop: while index < numbers.len() {
                    let number = numbers[index];
                    for (past_number_idx, past_number) in past_numbers.clone() {
                        if !rules.contains(&(past_number, number)) {
                            println!(
                                "Found unmatch: {}|{} for {}",
                                past_number,
                                number,
                                join_numbers(numbers.clone(), ",")
                            );
                            line_broken = true;
                            numbers.remove(index);
                            numbers.insert(past_number_idx, number);
                            index = 1;
                            past_numbers.clear();
                            past_numbers.insert((0, numbers[0]));
                            continue 'while_loop;
                        }
                    }
                    past_numbers.insert((index, number));
                    index += 1;
                }
                if line_broken {
                    result += numbers[numbers.len() / 2];
                    println!(
                        "fix: previous: {}, new: {}",
                        line,
                        join_numbers(numbers, ",")
                    );
                }
            }
        }
        Ok(result)
    }
    fn join_numbers(numbers: Vec<usize>, sep: &str) -> String {
        numbers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
