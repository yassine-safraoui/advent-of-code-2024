use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06"; // TODO: Fill the day
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

    //region Part 1
    println!("=== Part 1 ===");

    fn is_pos_in_board(position: (isize, isize), board_height: usize, board_width: usize) -> bool {
        position.0 >= 0
            && position.0 < board_height as isize
            && position.1 >= 0
            && position.1 < board_width as isize
    }

    #[derive(Copy, Clone, Hash, Eq, PartialEq)]
    enum DIRECTION {
        RIGHT,
        UP,
        LEFT,
        DOWN,
    }
    fn rotate_direction(direction: DIRECTION) -> DIRECTION {
        match direction {
            DIRECTION::RIGHT => DIRECTION::DOWN,
            DIRECTION::UP => DIRECTION::RIGHT,
            DIRECTION::LEFT => DIRECTION::UP,
            DIRECTION::DOWN => DIRECTION::LEFT,
        }
    }

    fn translate_position(position: (isize, isize), direction: DIRECTION) -> (isize, isize) {
        match direction {
            DIRECTION::RIGHT => (position.0, position.1 + 1),
            DIRECTION::UP => (position.0 - 1, position.1),
            DIRECTION::LEFT => (position.0, position.1 - 1),
            DIRECTION::DOWN => (position.0 + 1, position.1),
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
        let mut board_height = 0;
        let mut board_width = 0;
        let mut direction: DIRECTION = DIRECTION::UP;
        let mut position: (isize, isize) = (0, 0);
        for (i, line) in reader.lines().enumerate() {
            board_height += 1;
            let line = line.expect("reading line");
            if i == 0 {
                board_width = line.len()
            }
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert((i as isize, j as isize));
                }
                if "<>v^".contains(c) {
                    direction = match c {
                        'v' => DIRECTION::DOWN,
                        '<' => DIRECTION::LEFT,
                        '>' => DIRECTION::RIGHT,
                        '^' => DIRECTION::UP,
                        _ => DIRECTION::UP,
                    };
                    position = (i as isize, j as isize)
                }
            }
        }
        let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();

        while is_pos_in_board(position, board_height, board_width) {
            visited_positions.insert(position);
            let next_position = translate_position(position, direction);
            if obstacles.contains(&next_position) {
                direction = rotate_direction(direction);
                continue;
            }
            position = next_position;
        }
        Ok(visited_positions.len())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
        let mut board_height = 0;
        let mut board_width = 0;
        let mut initial_direction: DIRECTION = DIRECTION::UP;
        let mut initial_position: (isize, isize) = (0, 0);
        for (i, line) in reader.lines().enumerate() {
            let i = i as isize;
            board_height += 1;
            let line = line.expect("reading line");
            if i == 0 {
                board_width = line.len()
            }
            for (j, c) in line.chars().enumerate() {
                let j = j as isize;
                if c == '#' {
                    obstacles.insert((i, j));
                }
                if "<>v^".contains(c) {
                    initial_direction = match c {
                        'v' => DIRECTION::DOWN,
                        '<' => DIRECTION::LEFT,
                        '>' => DIRECTION::RIGHT,
                        '^' => DIRECTION::UP,
                        _ => DIRECTION::UP,
                    };
                    initial_position = (i, j)
                }
            }
        }
        let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
        let mut position = initial_position;
        let mut direction = initial_direction;
        while is_pos_in_board(position, board_height, board_width) {
            visited_positions.insert(position);
            let next_position = translate_position(position, direction);
            if obstacles.contains(&next_position) {
                direction = rotate_direction(direction);
                continue;
            }
            position = next_position;
        }
        let mut found_loops: usize = 0;
        'obstacles_loop: for obstacle in visited_positions {
            if obstacle == initial_position {
                continue;
            }
            obstacles.insert(obstacle);
            position = initial_position;
            direction = initial_direction;
            let mut current_visited_positions: HashSet<((isize, isize), DIRECTION)> =
                HashSet::new();
            current_visited_positions.insert((initial_position, initial_direction));
            while is_pos_in_board(position, board_height, board_width) {
                let next_position = translate_position(position, direction);
                if obstacles.contains(&next_position) {
                    direction = rotate_direction(direction);
                    continue;
                }
                position = next_position;

                if current_visited_positions.contains(&(position, direction)) {
                    found_loops += 1;
                    obstacles.remove(&obstacle);
                    continue 'obstacles_loop;
                }
                current_visited_positions.insert((position, direction));
            }
            obstacles.remove(&obstacle);
        }
        Ok(found_loops)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
