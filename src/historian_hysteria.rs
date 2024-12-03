use crate::historian_hysteria::location_id_list::LocationIdList;
use std::fs;

mod location_id_list;

/// https://adventofcode.com/2024/day/1
pub fn solve_historian_hysteria() {
    let input = fs::read_to_string("./day_1/input.txt").expect("Could not read input.txt.");
    let location_ids = parse_input(input);
    let difference = calculate_distance(location_ids.clone());
    println!("total difference is {difference}");

    let similarity = calculate_similarity(location_ids.clone());
    println!("similarity score is {similarity}");
}

fn parse_input(input: String) -> Vec<LocationIdList> {
    let mut location_ids: Vec<LocationIdList> = Vec::new();

    let tokens = input.split("\n").collect::<Vec<&str>>();
    for token in tokens {
        let parts = token.split_whitespace().collect::<Vec<&str>>();

        // If we haven't already created all the location lists we need, do that now
        while location_ids.len() < parts.len() {
            location_ids.push(LocationIdList { ids: Vec::new() });
        }

        // push each part to its respective location id list
        for i in 0..parts.len() {
            location_ids[i]
                .ids
                .push(parts[i].parse().expect("Could not parse location id"));
        }
    }

    // before returning, ensure the ids are sorts in all the lists
    for i in 0..location_ids.len() {
        location_ids[i].ids.sort();
    }

    // return the location id lists
    location_ids
}

fn calculate_distance(location_ids: Vec<LocationIdList>) -> u64 {
    let mut distance: u64 = 0;

    // peek at the number of ids in the first list
    let mut total = location_ids[0].ids.len();

    for i in 0..total {
        let mut difference: i64 = -1;

        for j in 0..location_ids.len() {
            if difference == -1 {
                difference = location_ids[j].ids[i] as i64
            } else {
                difference = difference - location_ids[j].ids[i] as i64;
            }
        }

        distance += difference.abs() as u64
    }

    // return the distance
    distance
}

fn calculate_similarity(location_ids: Vec<LocationIdList>) -> u64 {
    let mut similarity_score: u64 = 0;

    // peek at the number of ids in the first list
    let mut total = location_ids[0].ids.len();

    for num in location_ids[0].ids.clone() {
        let mut occurances: i64 = 0;
        for i in 1..location_ids.len() {
            for j in 0..total {
                if location_ids[i].ids[j] == num {
                    occurances += 1;
                }
            }
        }

        similarity_score += num * occurances as u64;
    }

    // return similarity score
    similarity_score
}
