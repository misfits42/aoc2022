use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use aoc2022::utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Hill Climbing Algorithm";
const PROBLEM_INPUT_FILE: &str = "./input/day12.txt";
const PROBLEM_DAY: u64 = 12;

/// Processes the AOC 2022 Day 12 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2022 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2022 Day 12 input file in the format required by the solver functions.
/// Returned value is tuple containing the heightmap, start point and end point.
fn process_input_file(filename: &str) -> (HashMap<Point2D, i64>, Point2D, Point2D) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut start: Option<Point2D> = None;
    let mut end: Option<Point2D> = None;
    let mut heightmap: HashMap<Point2D, i64> = HashMap::new();
    let mut y = 0;
    for line in raw_input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Process the line of the heightmap
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = Some(Point2D::new(x as i64, y));
                heightmap.insert(start.unwrap(), 0);
            } else if chr == 'E' {
                end = Some(Point2D::new(x as i64, y));
                heightmap.insert(end.unwrap(), 25);
            } else if chr.is_ascii_lowercase() {
                let height = (chr as i64) - ('a' as i64);
                heightmap.insert(Point2D::new(x as i64, y), height);
            } else {
                panic!("Day 12 - bad char in input file!");
            }
        }
        y += 1;
    }
    (heightmap, start.unwrap(), end.unwrap())
}

/// Solves AOC 2022 Day 12 Part 1 // Determines the minimum number of steps needed to reach the end
/// point from the start point.
fn solve_part1(problem_input: &(HashMap<Point2D, i64>, Point2D, Point2D)) -> u64 {
    let (heightmap, start, end) = problem_input;
    get_min_steps_to_end(heightmap, start, end)
}

/// Solves AOC 2022 Day 12 Part 2 // Determines the minimum number of steps needed to reach the end
/// point from a starting point with elevation 0.
fn solve_part2(problem_input: &(HashMap<Point2D, i64>, Point2D, Point2D)) -> u64 {
    let (heightmap, _, end) = problem_input;
    get_min_steps_from_elevation0_to_end(heightmap, end)
}

/// Determines the minimum number of steps needed to reach the end point from the start point.
fn get_min_steps_to_end(heightmap: &HashMap<Point2D, i64>, start: &Point2D, end: &Point2D) -> u64 {
    let mut visit_queue: VecDeque<(u64, Point2D)> = VecDeque::new();
    visit_queue.push_back((0, *start));
    let mut visited: HashSet<Point2D> = HashSet::new();
    visited.insert(*start);
    while !visit_queue.is_empty() {
        // Get the current point to visit
        let (steps, current_loc) = visit_queue.pop_front().unwrap();
        if current_loc == *end {
            return steps;
        }
        // Add the next points to visit
        for valid_point in get_next_valid_points(heightmap, &current_loc, false) {
            if !visited.contains(&valid_point) {
                visit_queue.push_back((steps + 1, valid_point));
                visited.insert(valid_point);
            }
        }
    }
    // Should have reached the end point already, so getting here indicates an error somewhere
    panic!("Day 12 Part 1 - did not reach the end point!");
}

/// Determines the minimum number of steps needed to reach a point with elevation 0 from the given
/// starting point.
fn get_min_steps_from_elevation0_to_end(heightmap: &HashMap<Point2D, i64>, start: &Point2D) -> u64 {
    let mut visit_queue: VecDeque<(u64, Point2D)> = VecDeque::new();
    visit_queue.push_back((0, *start));
    let mut visited: HashSet<Point2D> = HashSet::new();
    visited.insert(*start);
    while !visit_queue.is_empty() {
        // Get the current point to visit
        let (steps, current_loc) = visit_queue.pop_front().unwrap();
        if *heightmap.get(&current_loc).unwrap() == 0 {
            return steps;
        }
        // Add the next points to visit
        for valid_point in get_next_valid_points(heightmap, &current_loc, true) {
            if !visited.contains(&valid_point) {
                visit_queue.push_back((steps + 1, valid_point));
                visited.insert(valid_point);
            }
        }
    }
    // Should have reached the end point already, so getting here indicates an error somewhere
    panic!("Day 12 Part 2 - did not reach the end point!");
}

/// Gets the next valid points to visit from the current point.
fn get_next_valid_points(
    heightmap: &HashMap<Point2D, i64>,
    loc: &Point2D,
    reverse_course: bool,
) -> Vec<Point2D> {
    let mut valid_points: Vec<Point2D> = vec![];
    // Check the points to the left, up, right and down directions
    for (delta_x, delta_y) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let check_loc = loc.peek_move_point(*delta_x, *delta_y);
        // Determine the left and right points so elevation check is carried out correctly
        let left = {
            if reverse_course {
                &check_loc
            } else {
                loc
            }
        };
        let right = {
            if reverse_course {
                loc
            } else {
                &check_loc
            }
        };
        // Check if the checked location is a valid move from the current location
        if heightmap.contains_key(&check_loc)
            && (heightmap.get(right).unwrap() - heightmap.get(left).unwrap()) <= 1
        {
            valid_points.push(check_loc);
        }
    }
    valid_points
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 12 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day12_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(352, solution);
    }

    /// Tests the Day 12 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day12_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(345, solution);
    }
}
