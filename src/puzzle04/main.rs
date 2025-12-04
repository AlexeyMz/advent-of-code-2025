//! Day 4: Printing Department
use core::{Grid, MeasureElapsed, get_data_path};
use std::{fs::{File, read_to_string}, io::{LineWriter, Write}};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle04.txt")).unwrap();

    let grid = Grid::from_lines(
        &input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect::<Vec<String>>()
    )?;

    let mut output = grid.clone();
    let total_accessible = remove_paper_rolls(&grid, &mut output);

    println!("Total accessible paper rolls: {total_accessible}");

    let mut writer = LineWriter::new(
        File::create(get_data_path("output/puzzle04_accessible.txt")).unwrap()
    );
    output.write_into(&mut writer)
        .map_err(|_| "Failed to write output")?;

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle04.txt")).unwrap();

    let grid = Grid::from_lines(
        &input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect::<Vec<String>>()
    )?;

    let mut output = grid.clone();
    let mut total_removed = 0;

    let mut writer = LineWriter::new(
        File::create(get_data_path("output/puzzle04_removed_by_step.txt")).unwrap()
    );

    loop {
        let from = output.clone();
        let removed_at_step = remove_paper_rolls(&from, &mut output);
        if removed_at_step == 0 {
            break;
        }

        total_removed += removed_at_step;

        let roll_word = if removed_at_step == 1 { "roll" } else { "rolls" };
        writeln!(writer, "Removed {removed_at_step} {roll_word} of paper:")
            .map_err(|_| "Failed to write log at step")?;
        output.write_into(&mut writer)
            .map_err(|_| "Failed to write grid at step")?;
        writeln!(writer)
            .map_err(|_| "Failed to write empty line at step")?;

        for i in 0..output.width() {
            for j in 0..output.height() {
                if let Some('x') = output.get((i, j)) {
                    output.set((i, j), '.');
                }
            }
        }
    }

    println!("Total removable paper rolls: {total_removed}");

    return Ok(());
}

fn remove_paper_rolls(from: &Grid<char>, output: &mut Grid<char>) -> i32 {
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut removed_count = 0;
    for i in 0..from.width() {
        for j in 0..from.height() {
            if let Some('@') = from.get((i, j)) {
                let mut paper_around = 0;
                for (di, dj) in directions {
                    if let Some('@') = from.get((i + di, j + dj)) {
                        paper_around += 1;
                    }
                }
                if paper_around < 4 {
                    removed_count += 1;
                    output.set((i, j), 'x');
                }
            }
        }
    }

    removed_count
}
