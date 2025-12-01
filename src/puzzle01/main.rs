use core::{get_data_path};
use std::fs::{read_to_string};

use regex::Regex;

fn main() {
    use std::time::Instant;
    let before = Instant::now();
    basic().unwrap();
    advanced().unwrap();
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle01.txt")).unwrap();

    let parser = RotationParser::new()?;
    let rotations = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parser.parse(line))
        .collect::<Result<Vec<i32>, String>>()?;

    let mut position = 50;
    let mut secret_code = 0;
    for rotation in rotations {
        position += rotation;
        position = position.rem_euclid(100);
        if position == 0 {
            secret_code += 1;
        }
    }

    println!("Secret code is {secret_code}");

    return Result::Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle01.txt")).unwrap();

    let parser = RotationParser::new()?;
    let rotations = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parser.parse(line))
        .collect::<Result<Vec<i32>, String>>()?;

    let debug = false;

    let mut position = 50;
    let mut secret_code = 0;
    for rotation in rotations {
        if rotation == 0 {
            continue;
        }

        let before = position;
        position += rotation;

        let mut whole_rotations = position.div_euclid(100).abs();
        if rotation < 0 && before == 0 {
            whole_rotations -= 1;
        }

        secret_code += whole_rotations;
        position = position.rem_euclid(100);

        if debug {
            let sign = if rotation > 0 { "+" } else { "" };
            println!("Rotate {sign}{rotation}: {before} -> {position}");
            if whole_rotations > 0 {
                println!("Click {whole_rotations} times");
            }
        }

        if position == 0 && rotation < 0 {
            secret_code += 1;
            if debug {
                println!("Click once from stopping at zero");
            }
        }
    }

    println!("Secret code by CLICK method is {secret_code}");

    return Result::Ok(());
}

struct RotationParser {
    regex: Regex,
}

impl RotationParser {
    fn new() -> Result<RotationParser, String> {
        let regex = Regex::new(r"^(L|R)(\d+)$")
            .map_err(|_| "Failed to construct regex to parse")?;
        Result::Ok(RotationParser { regex })
    }

    fn parse(&self, line: &str) -> Result<i32, String> {
        let (_, [direction, count]) = self.regex
            .captures(line)
            .map(|c| c.extract())
            .ok_or(format!("Failed to parse rotation: {}", line))?;

        let count: i32 = count.parse().map_err(|_| format!("Failed to parse rotation count: {}", count))?;

        match direction {
            "L" => Result::Ok(-count),
            "R" => Result::Ok(count),
            _ => Result::Err(format!("Unexpected rotation direction: {}", direction))
        }
    }
}
