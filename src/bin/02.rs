use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    //region Part 1
    println!("=== Part 1 ===");

    fn verify_report<I>(levels: I) -> bool
    where
        I: IntoIterator<Item = usize>,
        I::IntoIter: Clone,
    {
        let levels: Vec<usize> = levels.into_iter().collect();

        if levels.is_empty() {
            return false;
        }
        // If there's only one level, it's considered unsafe
        if levels.len() == 1 {
            return false;
        }
        // Determine initial trend (increasing or decreasing)
        let increasing = levels[1] > levels[0];

        // Check each subsequent level
        for window in levels.windows(2) {
            let previous = window[0];
            let current = window[1];

            let difference: isize = (current as isize) - (previous as isize);

            // Check trend consistency and difference constraints
            if (increasing && difference <= 0)
                || (!increasing && difference >= 0)
                || difference.abs() < 1
                || difference.abs() > 3
            {
                return false;
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_reports: usize = 0;
        for line in reader.lines() {
            let report = line.expect("reading report");
            let levels: Vec<usize> = report
                .split_whitespace()
                .map(|s| s.parse().expect("parsing level as a number"))
                .collect();
            if verify_report(levels) {
                safe_reports += 1;
            }
        }
        Ok(safe_reports)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        #[derive(PartialEq, Eq)]
        enum NuclearErrorType {
            GapError,
            DirectionError,
        }

        fn are_adjacent_levels_correct(
            first_level: usize,
            second_level: usize,
            is_increasing: bool,
        ) -> Option<NuclearErrorType> {
            let difference: isize = (second_level as isize) - (first_level as isize);
            if difference.abs() < 1 || difference.abs() > 3 {
                Some(NuclearErrorType::GapError)
            } else if (is_increasing && difference <= 0) || (!is_increasing && difference >= 0) {
                Some(NuclearErrorType::DirectionError)
            } else {
                None
            }
        }

        fn vec_without_index<T: Clone>(vec: &Vec<T>, index: usize) -> Vec<T> {
            vec.iter()
                .enumerate()
                .filter_map(|(i, val)| if i != index { Some(val.clone()) } else { None })
                .collect()
        }

        let mut safe_reports: usize = 0;
        // let mut retries_queue: VecDeque<Vec<usize>> = VecDeque::new();

        for line in reader.lines() {
            let report = line.expect("reading report");
            let mut is_report_safe = true;
            let levels: Vec<usize> = report
                .split_whitespace()
                .map(|s| s.parse().expect("parsing level as a number"))
                .collect();

            if levels[0] == levels[1] {
                if verify_report(levels[1..].to_vec())
                    || verify_report(vec_without_index(&levels, 1))
                {
                    safe_reports += 1;
                }
                continue;
            }
            let is_increasing = levels[0] < levels[1];
            let mut previous_level: usize = levels[1];
            for index in 2..levels.len() {
                let level = levels[index];
                let error = are_adjacent_levels_correct(previous_level, level, is_increasing);
                if !error.is_none() {
                    if index == 2 {
                        if error.unwrap() == NuclearErrorType::DirectionError {
                            if verify_report(vec_without_index(&levels, 0)) {
                                break;
                            }
                        }
                        if verify_report(vec_without_index(&levels, 1)) {
                            break;
                        }
                    } else {
                        if verify_report(vec_without_index(&levels, index)) {
                            break;
                        }
                    }
                    is_report_safe = false;
                    break;
                }
                previous_level = level;
            }
            if is_report_safe {
                println!("Correct report: {}", report);
                safe_reports += 1
            }
        }
        Ok(safe_reports)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
