//! Day 6: Trash Compactor
use core::{MeasureElapsed, get_data_path};
use std::fs::read_to_string;

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle06.txt")).unwrap();
    let problems = parse_problems_normal(&input)?;

    let mut total = 0;
    for problem in problems {
        total += problem.compute();
    }

    println!("Total of normal problem results: {total}");

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle06.txt")).unwrap();
    let problems = parse_problems_cephalopod(&input)?;

    let mut total = 0;
    for problem in problems {
        total += problem.compute();
    }

    println!("Total of cephalopod problem results: {total}");

    return Ok(());
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(str: &str) -> Option<Operator> {
        match str {
            "+" => Some(Operator::Add),
            "*" => Some(Operator::Multiply),
            _ => None
        }
    }
}

struct Problem {
    numbers: Vec<i64>,
    op: Operator,
}

impl Problem {
    fn compute(&self) -> i64 {
        match self.op {
            Operator::Add => self.numbers.iter().fold(0, |acc, n| acc + n),
            Operator::Multiply => self.numbers.iter().fold(1, |acc, n| acc * n),
        }
    }
}

fn parse_problems_normal(input: &str) -> Result<Vec<Problem>, String> {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let items: Vec<_> = line.split_whitespace().collect();
        if let Some(first) = items.first() && Operator::parse(&first).is_some() {
            operators = items.into_iter()
                .map(|s| Operator::parse(s))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| format!("Failed to parse operators row: {line}"))?;
        } else {
            let numbers = items.into_iter()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| format!("Failed to parse numbers row: {line}"))?;
            rows.push(numbers);
        }
    }

    let mut problems = vec![];
    for (i, &op) in operators.iter().enumerate() {
        let numbers = rows.iter().map(|row| row[i]).collect();
        problems.push(Problem { numbers, op });
    }

    Ok(problems)
}

fn parse_problems_cephalopod(input: &str) -> Result<Vec<Problem>, String> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let items: Vec<_> = line.split_whitespace().collect();
        if let Some(first) = items.first() && Operator::parse(&first).is_some() {
            operators = items.into_iter()
                .map(|s| Operator::parse(s))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| format!("Failed to parse operators row: {line}"))?;
        } else {
            rows.push(line.chars().collect());
        }
    }

    let mut groups: Vec<Vec<i64>> = Vec::new();
    let mut current_group: Vec<i64> = Vec::new();
    let column_count = rows.iter().map(|c| c.len()).max().unwrap_or(0);
    for i in 0..column_count {
        let column: Vec<_> = rows.iter()
            .filter_map(|row| row.get(i))
            .filter_map(|ch| ch.to_digit(10).map(|d| d as i64))
            .collect();

        let is_last = (i + 1) == column_count;
        if !column.is_empty() {
            let number = column.iter().fold(0, |acc, digit| acc * 10 + digit);
            current_group.push(number);
        }

        if (column.is_empty() || is_last) && !current_group.is_empty() {
            groups.push(current_group.into_iter().collect());
            current_group = vec![];
        }
    }

    let problems = operators.into_iter()
        .zip(groups.into_iter())
        .map(|(op, numbers)| Problem { numbers, op })
        .collect();

    Ok(problems)
}
