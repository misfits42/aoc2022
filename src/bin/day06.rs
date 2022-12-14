use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Tuning Trouble";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

/// Processes the AOC 2022 Day 6 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 6 input file in the format required by the solver functions.
/// Returned value is vector of characters given in the input file.
fn process_input_file(filename: &str) -> Vec<char> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    return raw_input.trim().chars().collect::<Vec<char>>();
}

/// Solves AOC 2022 Day 6 Part 1 // Returns the number of characters that need to be processed
/// before the first start-of-packet marker (four consecutive characters that are different) is
/// observed.
fn solve_part1(input: &[char]) -> usize {
    if let Some(index) = find_marker_index(input, 4) {
        return index;
    }
    panic!("Day 6 Part 1 - did not find the start-of-packet marker!");
}

/// Solves AOC 2022 Day 6 Part 2 // Returns the number of characters that need to be processed
/// before the first start-of-message marker (14 consecutive characters that are different) is
/// observed.
fn solve_part2(input: &[char]) -> usize {
    if let Some(index) = find_marker_index(input, 14) {
        return index;
    }
    panic!("Day 6 Part 2 - did not find the start-of-message marker!");
}

/// Finds the index of the marker (sequence of characters that are different) in the given vector
/// of characters with the given length. Index is the number of characters from the start of the
/// given chars to the end of the marker (inclusive).
fn find_marker_index(chars: &[char], marker_len: usize) -> Option<usize> {
    for cursor in 0..(chars.len() - marker_len + 1) {
        let mut window_set: HashSet<char> = HashSet::new();
        for i in 0..marker_len {
            // Break early if duplicate character is observed
            if window_set.contains(&chars[cursor + i]) {
                break;
            }
            window_set.insert(chars[cursor + i]);
        }
        // Check if the marker has been found
        if window_set.len() == marker_len {
            return Some(cursor + marker_len);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 6 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1109, solution);
    }

    /// Tests the Day 6 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(3965, solution);
    }
}
