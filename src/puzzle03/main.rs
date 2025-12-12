//! Day 3: Lobby
use core_lib::{get_data_path, MeasureElapsed};
use std::{fs::read_to_string};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle03.txt")).unwrap();

    let battery_banks = parse_battery_banks(&input)?;

    let mut total_joltage = 0;
    for bank in battery_banks {
        let a = *bank.iter().take(bank.len() - 1).max().unwrap();
        let a_index = bank.iter().position(|&v| v == a).unwrap();
        let b = *bank.iter().skip(a_index + 1).max().unwrap();
        let joltage = a * 10 + b;
        total_joltage += joltage;
    }

    println!("Total max joltage: {total_joltage}");

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle03.txt")).unwrap();

    let battery_banks = parse_battery_banks(&input)?;

    let select_count = 12;

    let mut total_joltage: i64 = 0;
    for bank in battery_banks {
        let mut joltage: i64 = 0;
        let mut from_index = 0;

        eprintln!(" - {}", bank.iter().map(|v| char::from_digit(*v as u32, 10).unwrap()).collect::<String>());
        eprint!("   ");

        for i in 0..select_count {
            let take_count = bank.len() - from_index - (select_count - i - 1);
            let max = *bank.iter()
                .skip(from_index)
                .take(take_count)
                .max()
                .unwrap();
            let max_offset = bank.iter()
                .skip(from_index)
                .take(take_count)
                .position(|&v| v == max)
                .unwrap();

            for _ in 0..max_offset {
                eprint!(".");
            }
            eprint!("^");

            joltage = joltage * 10 + (max as i64);
            from_index += max_offset + 1;
        }

        for _ in from_index..bank.len() {
            eprint!(".");
        }
        eprintln!();

        total_joltage += joltage;
    }

    println!("Total max joltage: {total_joltage}");

    return Ok(());
}

type BatteryBank = Vec<i32>;

fn parse_battery_banks(input: &str) -> Result<Vec<BatteryBank>, String> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_battery_bank)
        .collect::<Result<Vec<BatteryBank>, _>>()
}

fn parse_battery_bank(line: &str) -> Result<BatteryBank, String> {
    line
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as i32))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| format!("Invalid battery bank: {line}"))
}
