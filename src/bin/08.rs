use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Map;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    type FloatingUnit = f32;
    type Vector = (FloatingUnit, FloatingUnit);
    trait VectorOps {
        fn new(v: (usize, usize)) -> Self;
        fn add(&self, other: &Self) -> Self;
        fn subtract(&self, other: &Self) -> Self;
        fn multiply(&self, factor: FloatingUnit) -> Self;
        fn divise(&self, factor: FloatingUnit) -> Self;
        fn invert(&self) -> Self;
        fn get_norm(&self) -> FloatingUnit;
        fn inside_grid(&self, height: usize, width: usize) -> bool;
        fn is_position_valid(&self, height: usize, width: usize) -> bool;
        fn to_string(&self) -> String;
    }
    impl VectorOps for Vector {
        fn new(v: (usize, usize)) -> Self {
            (v.0 as FloatingUnit, v.1 as FloatingUnit)
        }

        fn add(&self, other: &Self) -> Self {
            (self.0 + other.0, self.1 + other.1)
        }

        fn subtract(&self, other: &Self) -> Self {
            (self.0 - other.0, self.1 - other.1)
        }

        fn multiply(&self, factor: FloatingUnit) -> Self {
            (self.0 * factor, self.1 * factor)
        }

        fn divise(&self, factor: FloatingUnit) -> Self {
            (self.0 / factor, self.1 / factor)
        }

        fn invert(&self) -> Self {
            (-self.0, -self.1)
        }

        fn get_norm(&self) -> FloatingUnit {
            (self.0.powi(2) + self.1.powi(2)).sqrt()
        }
        fn inside_grid(&self, height: usize, width: usize) -> bool {
            (self.0 as usize) < height
                && (self.1 as usize) < width
                && 0 <= (self.0 as isize)
                && 0 <= (self.1 as isize)
        }
        fn is_position_valid(&self, height: usize, width: usize) -> bool {
            self.inside_grid(height, width) && self.0.fract() == 0_f32 && self.1.fract() == 0_f32
        }

        fn to_string(&self) -> String {
            format!("({}, {})", self.0, self.1)
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        let mut height = 0;
        let mut width = 0;

        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line.expect("reading line");
            width = line.len();
            height = i + 1;
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    if !antennas.contains_key(&ch) {
                        antennas.insert(ch, Vec::new());
                    }
                    let positions = antennas.get_mut(&ch).unwrap();
                    positions.push((i, j));
                }
            }
        }
        for (_, positions) in antennas {
            let antennas_count = positions.len();
            for antenna1 in 0..antennas_count - 1 {
                for antenna2 in antenna1 + 1..antennas_count {
                    let antenna1: &Vector = &Vector::new(positions[antenna1]);
                    let antenna2: &Vector = &Vector::new(positions[antenna2]);
                    let v = antenna2.subtract(antenna1);
                    let matches: [Vector; 4] = [
                        antenna1.add(&v.multiply(2_f32)),
                        antenna1.add(&v.multiply(2_f32).divise(3_f32)),
                        antenna1.add(&v.divise(3_f32)),
                        antenna1.add(&v.invert()),
                    ];
                    for _match in matches {
                        if _match.is_position_valid(height, width) {
                            antinodes.insert((_match.0 as usize, _match.1 as usize));
                        }
                    }
                }
            }
        }
        Ok(antinodes.len())
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
        let mut height = 0;
        let mut width = 0;

        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line.expect("reading line");
            width = line.len();
            height = i + 1;
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    if !antennas.contains_key(&ch) {
                        antennas.insert(ch, Vec::new());
                    }
                    let positions = antennas.get_mut(&ch).unwrap();
                    positions.push((i, j));
                }
            }
        }
        for (_, positions) in antennas {
            let antennas_count = positions.len();
            for antenna1 in 0..antennas_count - 1 {
                antinodes.insert((positions[antenna1].0, positions[antenna1].1));
                for antenna2 in antenna1 + 1..antennas_count {
                    let antenna1: &Vector = &Vector::new(positions[antenna1]);
                    let antenna2: &Vector = &Vector::new(positions[antenna2]);
                    antinodes.insert((antenna2.0 as usize, antenna2.1 as usize));
                    let v = antenna2.subtract(antenna1);
                    let matches: [Vector; 2] = [
                        antenna1.add(&v.multiply(2_f32).divise(3_f32)),
                        antenna1.add(&v.divise(3_f32)),
                    ];
                    for _match in matches {
                        if _match.is_position_valid(height, width) {
                            antinodes.insert((_match.0 as usize, _match.1 as usize));
                        }
                    }
                    let mut i: usize = 1;
                    let v_inverted = &v.invert();
                    loop {
                        let mut is_in_grid = false;
                        let match1 = antenna2.add(&v.multiply(i as FloatingUnit));
                        let match2 = antenna1.add(&v_inverted.multiply(i as FloatingUnit));
                        if match1.inside_grid(height, width) {
                            is_in_grid = true;
                            if match1.is_position_valid(height, width) {
                                antinodes.insert((match1.0 as usize, match1.1 as usize));
                            }
                        }
                        if match2.inside_grid(height, width) {
                            is_in_grid = true;
                            if match2.is_position_valid(height, width) {
                                antinodes.insert((match2.0 as usize, match2.1 as usize));
                            }
                        }
                        i += 1;
                        if !is_in_grid {
                            break;
                        }
                    }
                }
            }
        }
        Ok(antinodes.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
