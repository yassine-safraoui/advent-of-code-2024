use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn verify_xmas(word: String) -> bool {
        word == "XMAS" || word == "SAMX"
    }
    fn verify_mas(word: String) -> bool {
        word == "MAS" || word == "SAM"
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut input: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line.expect("reading line");
            input.push(line.chars().collect::<Vec<char>>())
        }
        let height: usize = input.len();
        let width: usize = input[0].len();
        for h in 0..height {
            for w in 0..width {
                // println!("h:{}, w:{}", h, w);
                let mut c1: bool = false;
                let mut c2: bool = false;
                if w < width - 3 {
                    c1 = true;
                    let word: String = input.get(h).unwrap()[w..w + 4].iter().collect();
                    if verify_xmas(word.clone()) {
                        // println!("Horizontal h:{}, w:{} {}", h, w, word);
                        result += 1;
                    }
                }
                if h < height - 3 {
                    c2 = true;
                    let word: String = input[h..h + 4].iter().map(|s| s.get(w).unwrap()).collect();
                    if verify_xmas(word.clone()) {
                        // println!("Vertical h:{}, w:{} {}", h, w, word);
                        result += 1;
                    }
                }
                if c1 && c2 {
                    let word: String = (0..4)
                        .map(|i| input.get(h + i).unwrap().get(w + i).unwrap())
                        .collect();
                    if verify_xmas(word.clone()) {
                        // println!("Diagonal h:{}, w:{} {}", h, w, word);
                        result += 1;
                    }
                }
                if c2 && (w > 2) {
                    let word: String = (0..4)
                        .map(|i| input.get(h + i).unwrap().get(w - i).unwrap())
                        .collect();
                    if verify_xmas(word.clone()) {
                        // println!("Diagonal h:{}, w:{} {}", h, w, word);
                        result += 1;
                    }
                }
            }
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;
        let mut input: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line.expect("reading line");
            input.push(line.chars().collect::<Vec<char>>())
        }
        let height: usize = input.len();
        let width: usize = input[0].len();
        for h in 1..height - 1 {
            for w in 1..width - 1 {
                let mut w1 = String::new();
                let mut w2 = String::new();
                for i in 0..3 {
                    w1.push(
                        input
                            .get(h - 1 + i)
                            .unwrap()
                            .get(w - 1 + i)
                            .unwrap()
                            .to_ascii_uppercase(),
                    );
                    w2.push(
                        input
                            .get(h + 1 - i)
                            .unwrap()
                            .get(w - 1 + i)
                            .unwrap()
                            .to_ascii_uppercase(),
                    )
                }
                println!("w1: {}, w2: {}", w1, w2);
                if verify_mas(w1) && verify_mas(w2) {
                    result += 1
                }
            }
        }
        Ok(result)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
