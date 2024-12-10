use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    type Cell = (usize, usize);
    trait CellMethods {
        fn get_adjacent_cells(&self, height: usize, width: usize) -> Vec<Cell>;
        fn to_string(&self) -> String;
    }
    impl CellMethods for Cell {
        fn get_adjacent_cells(&self, height: usize, width: usize) -> Vec<Cell> {
            let &(x, y) = self;
            let mut adjacent_cells: Vec<Cell> = Vec::new();

            // Up
            if x > 0 {
                adjacent_cells.push((x - 1, y));
            }
            // Down
            if x + 1 < height {
                adjacent_cells.push((x + 1, y));
            }
            // Left
            if y > 0 {
                adjacent_cells.push((x, y - 1));
            }
            // Right
            if y + 1 < width {
                adjacent_cells.push((x, y + 1));
            }

            adjacent_cells
        }

        fn to_string(&self) -> String {
            format!("({},{})", self.0, self.1)
        }
    }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        let mut map: Vec<Vec<usize>> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut trailheads: HashSet<Cell> = HashSet::new();
        for (i, line) in reader.lines().enumerate() {
            height = i + 1;
            let line = line.expect("reading line");
            if i == 0 {
                width = line.len()
            }
            let mut numbers: Vec<usize> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let n: usize = c.to_string().parse().unwrap_or(100);
                if n == 0 {
                    trailheads.insert((i, j));
                }
                numbers.push(n);
            }
            map.push(numbers)
        }
        let mut queue: LinkedList<(Cell, Cell)> = LinkedList::new();
        let mut scores: HashMap<Cell, HashSet<Cell>> = HashMap::new();
        for trailhead in trailheads {
            scores.insert(trailhead, HashSet::new());
            for neighbour in trailhead.get_adjacent_cells(height, width) {
                if map[neighbour.0][neighbour.1] == 1 {
                    queue.push_back((trailhead, neighbour));
                }
            }
        }
        while !queue.is_empty() {
            let (origin, cell) = queue.pop_front().unwrap();
            for neighbour in cell.get_adjacent_cells(height, width) {
                if (map[neighbour.0][neighbour.1] as isize) - (map[cell.0][cell.1] as isize) == 1 {
                    if map[neighbour.0][neighbour.1] == 9 {
                        scores.get_mut(&origin).unwrap().insert(neighbour);
                        continue;
                    }
                    queue.push_back((origin, neighbour));
                }
            }
        }
        for sets in scores.values() {
            result += sets.len()
        }
        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result = 0;
        let mut map: Vec<Vec<usize>> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut trailheads: HashSet<Cell> = HashSet::new();
        for (i, line) in reader.lines().enumerate() {
            height = i + 1;
            let line = line.expect("reading line");
            if i == 0 {
                width = line.len()
            }
            let mut numbers: Vec<usize> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let n: usize = c.to_string().parse().unwrap_or(100);
                if n == 0 {
                    trailheads.insert((i, j));
                }
                numbers.push(n);
            }
            map.push(numbers)
        }
        let mut queue: LinkedList<(Cell, Cell)> = LinkedList::new();
        for trailhead in trailheads {
            for neighbour in trailhead.get_adjacent_cells(height, width) {
                if map[neighbour.0][neighbour.1] == 1 {
                    queue.push_back((trailhead, neighbour));
                }
            }
        }
        while !queue.is_empty() {
            let (origin, cell) = queue.pop_front().unwrap();
            for neighbour in cell.get_adjacent_cells(height, width) {
                if (map[neighbour.0][neighbour.1] as isize) - (map[cell.0][cell.1] as isize) == 1 {
                    if map[neighbour.0][neighbour.1] == 9 {
                        result += 1;
                        continue;
                    }
                    queue.push_back((origin, neighbour));
                }
            }
        }
        Ok(result)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
