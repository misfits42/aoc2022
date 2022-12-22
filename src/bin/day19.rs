use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

lazy_static! {
    static ref ROBOT_COMBOS: Vec<Vec<RobotType>> = {
        let mut robot_combos: Vec<Vec<RobotType>> = vec![];
        robot_combos.extend(RobotType::iter().combinations(1));
        robot_combos.extend(RobotType::iter().combinations(2));
        robot_combos.extend(RobotType::iter().combinations(3));
        robot_combos.extend(RobotType::iter().combinations(4));
        robot_combos
    };
}

const PROBLEM_NAME: &str = "Not Enough Minerals";
const PROBLEM_INPUT_FILE: &str = "./input/day19.txt";
// const PROBLEM_INPUT_FILE: &str = "./input/test/day19_t001.txt";
const PROBLEM_DAY: u64 = 19;

const PART1_MINUTES_ALLOWED: u64 = 24;

/// Represents the different kinds of robot.
#[derive(Copy, Clone, PartialEq, Eq, EnumIter)]
enum RobotType {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}

// /// Represents the different kind of resource.
// enum ResourceType {
//     Ore,
//     Clay,
//     Obsidian,
//     Geode,
// }

#[derive(Clone, Copy, PartialEq, Eq)]
struct ResourceBag {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

impl ResourceBag {
    pub fn new(ore: u64, clay: u64, obsidian: u64, geode: u64) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    pub fn fits_within(&self, other: &ResourceBag) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    pub fn blank() -> ResourceBag {
        ResourceBag {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

/// Represents a blueprint with robots having different costs
struct Blueprint {
    id: u64,
    ore_robot: ResourceBag,
    clay_robot: ResourceBag,
    obsidian_robot: ResourceBag,
    geode_robot: ResourceBag,
}

impl Blueprint {
    pub fn new(
        id: u64,
        ore_robot: ResourceBag,
        clay_robot: ResourceBag,
        obsidian_robot: ResourceBag,
        geode_robot: ResourceBag,
    ) -> Self {
        Self {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
}

/// Processes the AOC 2022 Day 19 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2022 Day 19 input file in the format required by the solver functions.
/// Returned value is vector of blueprints specified in the input file.
fn process_input_file(filename: &str) -> Vec<Blueprint> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_blueprint = Regex::new(concat!(
        r#"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. "#,
        r#"Each obsidian robot costs (\d+) ore and (\d+) clay. "#,
        r#"Each geode robot costs (\d+) ore and (\d+) obsidian.$"#,
    ))
    .unwrap();
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in raw_input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let caps = regex_blueprint.captures(line).unwrap();
        // Extract parameters from input line
        let id = caps[1].parse::<u64>().unwrap();
        let ore_robot = ResourceBag {
            ore: caps[2].parse::<u64>().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let clay_robot = ResourceBag {
            ore: caps[3].parse::<u64>().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let obsidian_robot = ResourceBag {
            ore: caps[4].parse::<u64>().unwrap(),
            clay: caps[5].parse::<u64>().unwrap(),
            obsidian: 0,
            geode: 0,
        };
        let geode_robot = ResourceBag {
            ore: caps[6].parse::<u64>().unwrap(),
            clay: 0,
            obsidian: caps[7].parse::<u64>().unwrap(),
            geode: 0,
        };
        // Create and record the blueprint
        let bp = Blueprint::new(id, ore_robot, clay_robot, obsidian_robot, geode_robot);
        blueprints.push(bp);
    }
    blueprints
}

/// Solves AOC 2022 Day 19 Part 1 // Calculates the sum of the quality levels of the blueprints.
fn solve_part1(blueprints: &[Blueprint]) -> u64 {
    let mut total = 0;
    for bp in blueprints {
        println!("[+] Simulating blueprint {}...", bp.id);
        total += simulate_blueprint(bp, PART1_MINUTES_ALLOWED);
    }
    total
}

/// Solves AOC 2022 Day 19 Part 2 // ###
fn solve_part2(_input: &[Blueprint]) -> u64 {
    0
}

fn simulate_blueprint(blueprint: &Blueprint, time_allowed: u64) -> u64 {
    let mut geode_totals: HashSet<u64> = HashSet::new();
    geode_totals.insert(0);
    let resource_blank = ResourceBag::blank();
    let robot_start = ResourceBag::new(1, 0, 0, 0);
    let mut earliest_geode_robot_time = 0;
    simulate_blueprint_recursive(
        blueprint,
        &mut geode_totals,
        resource_blank,
        robot_start,
        resource_blank,
        time_allowed,
        &mut earliest_geode_robot_time,
        false,
    );
    geode_totals.iter().max().unwrap() * blueprint.id
}

fn simulate_blueprint_recursive(
    blueprint: &Blueprint,
    geode_totals: &mut HashSet<u64>,
    resource_total: ResourceBag,
    robot_total: ResourceBag,
    robot_construction: ResourceBag,
    time_remaining: u64,
    earliest_geode_robot_time: &mut u64,
    skip_build: bool,
) {
    // std::thread::sleep(std::time::Duration::from_millis(100));
    // println!("time remaining: {}", time_remaining);
    if time_remaining <= 0 {
        if geode_totals.insert(resource_total.geode) {
            println!("[{}] new geode total: {}", blueprint.id, resource_total.geode);
        }
        return;
    }
    // Try to build robots
    if !skip_build && time_remaining > 1 {
        let mut build_options: Vec<Vec<RobotType>> = vec![vec![]];
        for combo in ROBOT_COMBOS.iter() {
            let mut resources_needed = ResourceBag::blank();
            for robot in combo {
                match robot {
                    RobotType::OreRobot => {
                        resources_needed.ore += blueprint.ore_robot.ore;
                        resources_needed.clay += blueprint.ore_robot.clay;
                        resources_needed.obsidian += blueprint.ore_robot.obsidian;
                    }
                    RobotType::ClayRobot => {
                        resources_needed.ore += blueprint.clay_robot.ore;
                        resources_needed.clay += blueprint.clay_robot.clay;
                        resources_needed.obsidian += blueprint.clay_robot.obsidian;
                    }
                    RobotType::ObsidianRobot => {
                        resources_needed.ore += blueprint.obsidian_robot.ore;
                        resources_needed.clay += blueprint.obsidian_robot.clay;
                        resources_needed.obsidian += blueprint.obsidian_robot.obsidian;
                    }
                    RobotType::GeodeRobot => {
                        resources_needed.ore += blueprint.geode_robot.ore;
                        resources_needed.clay += blueprint.geode_robot.clay;
                        resources_needed.obsidian += blueprint.geode_robot.obsidian;
                    }
                }
            }
            if resource_total.fits_within(&resources_needed) {
                build_options.push(combo.clone());
            }
        }
        for build_option in build_options {
            let mut robot_construction = ResourceBag::blank();
            let mut resource_total = resource_total;
            for robot in build_option {
                match robot {
                    RobotType::OreRobot => {
                        robot_construction.ore += 1;
                        resource_total.ore -= blueprint.ore_robot.ore;
                        resource_total.clay -= blueprint.ore_robot.clay;
                        resource_total.obsidian -= blueprint.ore_robot.obsidian;
                    }
                    RobotType::ClayRobot => {
                        robot_construction.clay += 1;
                        resource_total.ore -= blueprint.clay_robot.ore;
                        resource_total.clay -= blueprint.clay_robot.clay;
                        resource_total.obsidian -= blueprint.clay_robot.obsidian;
                    }
                    RobotType::ObsidianRobot => {
                        robot_construction.obsidian += 1;
                        resource_total.ore -= blueprint.obsidian_robot.ore;
                        resource_total.clay -= blueprint.obsidian_robot.clay;
                        resource_total.obsidian -= blueprint.obsidian_robot.obsidian;
                    }
                    RobotType::GeodeRobot => {
                        if time_remaining > *earliest_geode_robot_time {
                            *earliest_geode_robot_time = time_remaining;
                        } else if time_remaining < *earliest_geode_robot_time {
                            return;
                        }
                        robot_construction.geode += 1;
                        resource_total.ore -= blueprint.geode_robot.ore;
                        resource_total.clay -= blueprint.geode_robot.clay;
                        resource_total.obsidian -= blueprint.geode_robot.obsidian;
                    }
                }
            }
            // prune
            if time_remaining == 2 && robot_total.geode == 0 && robot_construction.geode == 0 {
                return;
            }
            if time_remaining <= 4 && robot_construction.obsidian > 0 {
                return;
            }
            if time_remaining <= 7 && robot_construction.clay > 0 {
                return;
            }
            if time_remaining <= 14 && robot_construction.ore > 0 {
                return;
            }
            simulate_blueprint_recursive(
                blueprint,
                geode_totals,
                resource_total,
                robot_total,
                robot_construction,
                time_remaining,
                earliest_geode_robot_time,
                true,
            );
        }
    }
    // Collect resources
    let mut resource_total = resource_total;
    resource_total.ore += robot_total.ore;
    resource_total.clay += robot_total.clay;
    resource_total.obsidian += robot_total.obsidian;
    resource_total.geode += robot_total.geode;
    // Check for robot construction
    let mut robot_total = robot_total;
    robot_total.ore += robot_construction.ore;
    robot_total.clay += robot_construction.clay;
    robot_total.obsidian += robot_construction.obsidian;
    robot_total.geode += robot_construction.geode;
    // Go to the next step
    let robot_construction = ResourceBag::blank();
    simulate_blueprint_recursive(
        blueprint,
        geode_totals,
        resource_total,
        robot_total,
        robot_construction,
        time_remaining - 1,
        earliest_geode_robot_time,
        false,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 19 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day19_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 19 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day19_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
