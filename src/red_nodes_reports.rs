mod direction;
mod report;

use crate::red_nodes_reports::direction::Direction;
use crate::red_nodes_reports::report::Report;
use std::cmp::PartialEq;
use std::fs;

pub fn solve_red_nosed_reports() {
    let input = fs::read_to_string("./day_2/input.txt").expect("Could not read input.txt.");
    let reports = parse_input(input);

    let num_safe = calculate_safe(reports.clone(), 1);
    println!("Safe without recheck: {}", num_safe);

    let num_safe2 = calculate_safe(reports.clone(), 2);
    print!("Safe with recheck: {}", num_safe2);
}

fn is_levels_safe(levels: Vec<u8>) -> bool {
    let mut in_range = true;
    let mut all_same_direction = true;
    let mut last_direction = Direction::NotYetSet;
    let mut next_direction = Direction::NotYetSet;

    // println!("checking safety for {:?}", levels);
    for i in 0..levels.len() {
        let level = levels[i];
        // println!("index {i}: {level}");

        // check adjacent levels are within range
        if i > 0 && i - 1 >= 0 {
            let neighbor = levels[i - 1];
            let difference = (level as i8 - neighbor as i8).abs();
            in_range = difference >= 1 && difference <= 3;
            // println!("{neighbor}, {level} in range? {in_range}")
        }

        if in_range && i + 1 < levels.len() {
            let neighbor = levels[i + 1];
            let difference = (level as i8 - neighbor as i8).abs();
            in_range = difference >= 1 && difference <= 3;
            // println!("{level}, {neighbor} in range? {in_range}");

            next_direction = if neighbor > level {
                Direction::Increasing
            } else if neighbor < level {
                Direction::Decreasing
            } else {
                Direction::Equal
            };

            all_same_direction =
                last_direction == next_direction || last_direction == Direction::NotYetSet
        }

        // println!("in range? {in_range}, all same direction? {all_same_direction}");

        if !in_range || !all_same_direction {
            // println!("break!");
            break;
        }

        last_direction = next_direction;
    }

    in_range && all_same_direction
}

fn calculate_safe(reports: Vec<Report>, times_to_check: u8) -> u16 {
    let mut safe = 0;

    for report in reports {
        let mut attempt = 0;
        let mut levels = report.levels.clone();
        let mut is_already_safe = false;
        while attempt < times_to_check && !is_already_safe {
            // println!("attempt {}", attempt);
            if attempt == 0 {
                is_already_safe = is_levels_safe(levels.clone());
                if is_already_safe {
                    println!("{:?} safe", report);
                    safe += 1;
                } else {
                    println!("{:?} unsafe", report);
                }
            } else if !is_already_safe {
                let mut is_safe = false;
                for i in 0..report.levels.len() {
                    let mut levels = report.levels.clone();
                    levels.remove(i);
                    is_safe = is_levels_safe(levels);

                    if is_safe {
                        break;
                    }
                }

                if is_safe {
                    safe += 1;
                    println!("{:?} safe after recheck", report);
                } else {
                    println!("{:?} unsafe after recheck", report);
                }
            }

            attempt += 1;
        }
    }

    safe
}

fn parse_input(input: String) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();

    let tokens = input.split("\n").collect::<Vec<&str>>();

    for token in tokens {
        let mut report = Report { levels: Vec::new() };

        for part in token.split(" ") {
            report
                .levels
                .push(part.parse().expect("Could not parse level."));
        }

        reports.push(report);
    }

    reports
}
