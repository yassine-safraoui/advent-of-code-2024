use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use utf8_chars::BufReadCharsExt;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut result = 0;
        let mut blocks: Vec<Option<usize>> = Vec::new();
        let mut reading_block = true;
        let mut id: usize = 0;
        let mut first_empty_block: usize = 0;
        for char in reader.chars() {
            let char: String = char.expect("reading next character").to_string();
            if char == "\n" {
                continue;
            }
            let n: usize = char.parse().expect("parsing character to string");
            if reading_block {
                for _ in 0..n {
                    blocks.push(Some(id));
                }
                id += 1;
            } else {
                if id == 1 {
                    first_empty_block = blocks.len()
                }
                for _ in 0..n {
                    blocks.push(None);
                }
            }
            reading_block = !reading_block;
        }
        'ordering: while first_empty_block < blocks.len() {
            let last = blocks.pop().unwrap();
            match last {
                None => {
                    continue;
                }
                Some(id) => {
                    blocks[first_empty_block] = Some(id);
                    first_empty_block += 1;
                    while first_empty_block < blocks.len() && !blocks[first_empty_block].is_none() {
                        first_empty_block += 1
                    }
                    if first_empty_block >= blocks.len() {
                        break 'ordering;
                    }
                }
            }
        }
        for (index, block) in blocks.iter().enumerate() {
            let block = block.unwrap();
            result += index * block;
        }
        Ok(result)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    struct FullBlock {
        start: usize,
        length: usize,
        id: usize,
    }
    struct EmptyBlock {
        start: usize,
        length: usize,
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut result = 0;
        let mut blocks_array: Vec<Option<usize>> = Vec::new();
        let mut reading_block = true;
        let mut id: usize = 0;
        let mut full_blocks: Vec<FullBlock> = Vec::new();
        let mut empty_blocks: Vec<EmptyBlock> = Vec::new();
        for char in reader.chars() {
            let char: String = char.expect("reading next character").to_string();
            if char == "\n" {
                continue;
            }
            let n: usize = char.parse().expect("parsing character to string");
            if n == 0 {
                reading_block = !reading_block;
                continue;
            }
            if reading_block {
                full_blocks.push(FullBlock {
                    start: blocks_array.len(),
                    length: n,
                    id,
                });
                for _ in 0..n {
                    blocks_array.push(Some(id));
                }
                id += 1;
            } else {
                empty_blocks.push(EmptyBlock {
                    start: blocks_array.len(),
                    length: n,
                });
                for _ in 0..n {
                    blocks_array.push(None);
                }
            }
            reading_block = !reading_block;
        }
        for full_block in full_blocks.iter().rev() {
            'empty_block_search: for empty_block in empty_blocks.iter_mut() {
                if empty_block.start >= full_block.start {
                    break;
                }
                if empty_block.length >= full_block.length {
                    if empty_block.start + empty_block.length >= full_block.start {
                        for i in 0..full_block.length {
                            blocks_array[full_block.start + i] = None;
                        }
                        for i in 0..full_block.length {
                            blocks_array[empty_block.start + i] = Some(full_block.id);
                        }
                        break 'empty_block_search;
                    }
                    for i in 0..full_block.length {
                        blocks_array[empty_block.start + i] = Some(full_block.id);
                        blocks_array[full_block.start + i] = None;
                    }
                    empty_block.start += full_block.length;
                    empty_block.length -= full_block.length;
                    break 'empty_block_search;
                }
            }
        }
        println!();
        for (index, block) in blocks_array.iter().enumerate() {
            if !block.is_none() {
                let block = block.unwrap();
                result += index * block;
            }
        }
        Ok(result)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
