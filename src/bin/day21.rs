use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use regex::Regex;

const PROBLEM_NAME: &str = "Monkey Math";
const PROBLEM_INPUT_FILE: &str = "./input/day21.txt";
const PROBLEM_DAY: u64 = 21;

enum Operation {
    Nop { value: i64 },
    Add { left: String, right: String },
    Subtract { left: String, right: String },
    Multiply { left: String, right: String },
    Divide { left: String, right: String },
}

/// Processes the AOC 2022 Day 21 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 21 input file in the format required by the solver functions.
/// Returned value is hashmap of monkey names mapped to their operation.
fn process_input_file(filename: &str) -> HashMap<String, Operation> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_nop = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
    let regex_add = Regex::new(r"^([a-z]+): ([a-z]+) \+ ([a-z]+)$").unwrap();
    let regex_subtract = Regex::new(r"^([a-z]+): ([a-z]+) \- ([a-z]+)$").unwrap();
    let regex_multiply = Regex::new(r"^([a-z]+): ([a-z]+) \* ([a-z]+)$").unwrap();
    let regex_divide = Regex::new(r"^([a-z]+): ([a-z]+) / ([a-z]+)$").unwrap();
    let mut output: HashMap<String, Operation> = HashMap::new();
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(caps) = regex_nop.captures(line) {
            let name = caps[1].to_string();
            let value = caps[2].parse::<i64>().unwrap();
            output.insert(name, Operation::Nop { value });
        } else if let Some(caps) = regex_add.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Add { left, right });
        } else if let Some(caps) = regex_subtract.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Subtract { left, right });
        } else if let Some(caps) = regex_multiply.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Multiply { left, right });
        } else if let Some(caps) = regex_divide.captures(line) {
            let name = caps[1].to_string();
            let left = caps[2].to_string();
            let right = caps[3].to_string();
            output.insert(name, Operation::Divide { left, right });
        } else {
            panic!("Day 21 - bad input line!");
        }
    }
    output
}

/// Solves AOC 2022 Day 21 Part 1 // Determines the number that the monkey named "root" will yell
/// out.
fn solve_part1(monkey_ops: &HashMap<String, Operation>) -> i64 {
    determine_monkey_yell_value("root", monkey_ops)
}

/// Solves AOC 2022 Day 21 Part 2 // ###
fn solve_part2(_input: &HashMap<String, Operation>) -> i64 {
    0
}

/// Determines the value that will be yelled by the named monkey.
fn determine_monkey_yell_value(name: &str, monkey_ops: &HashMap<String, Operation>) -> i64 {
    match monkey_ops.get(name).unwrap() {
        Operation::Nop { value } => {
            *value
        }
        Operation::Add { left, right } => {
            determine_monkey_yell_value(left, monkey_ops) + determine_monkey_yell_value(right, monkey_ops)
        }
        Operation::Subtract { left, right } => {
            determine_monkey_yell_value(left, monkey_ops) - determine_monkey_yell_value(right, monkey_ops)
        }
        Operation::Multiply { left, right } => {
            determine_monkey_yell_value(left, monkey_ops) * determine_monkey_yell_value(right, monkey_ops)
        }
        Operation::Divide { left, right } => {
            determine_monkey_yell_value(left, monkey_ops) / determine_monkey_yell_value(right, monkey_ops)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 21 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day21_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(268597611536314, solution);
    }

    /// Tests the Day 21 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day21_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
