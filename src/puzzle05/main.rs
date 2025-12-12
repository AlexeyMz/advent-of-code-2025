//! Day 5: Cafeteria
use core_lib::{MeasureElapsed, RangeSet, get_data_path};
use std::fs::{read_to_string};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle05.txt")).unwrap();
    let db = Database::parse(&input)?;

    let mut fresh_count = 0;
    for id in db.available_ids {
        if db.fresh_ranges.find_range(id).is_some() {
            fresh_count += 1;
        }
    }

    println!("Fresh ingredient ID count: {fresh_count}");

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle05.txt")).unwrap();

    let db = Database::parse(&input)?;
    let fresh_total_count = db.fresh_ranges.ranges()
        .iter().fold(0, |acc, r| acc + (*r.end() - *r.start() + 1));

    println!("Fresh ingredient total range length: {fresh_total_count}");

    return Ok(());
}

struct Database {
    fresh_ranges: RangeSet<i64>,
    available_ids: Vec<i64>,
}

impl Database {
    fn parse(input: &str) -> Result<Database, String> {
        let mut ranges = vec![];
        let mut available_ids = vec![];
        let mut scan_ranges = true;

        for line in input.lines() {
            if scan_ranges {
                if line.is_empty() {
                    scan_ranges = false
                } else {
                    let boundaries: Vec<&str> = line.split("-").collect();
                    if boundaries.len() == 2 {
                        let boundaries: Vec<_> = boundaries
                            .into_iter().map(|s| s.parse::<i64>()).collect();
                        if let [Ok(first), Ok(second)] = &boundaries[..] {
                            ranges.push(*first..=*second);
                            continue;
                        }
                    }
                    return Err(format!("Invalid range: {line}"));
                }
            } else {
                if line.is_empty() {
                    break;
                } else {
                    if let Ok(id) = line.parse::<i64>() {
                        available_ids.push(id);
                    } else {
                        return Err(format!("Invalid available ID: {line}"));
                    }
                }
            }
        }

        Ok(Database { fresh_ranges: RangeSet::new(&ranges), available_ids })
    }
}
