//! Day 7: Laboratories
use core::{Grid, MeasureElapsed, get_data_path};
use std::{fs::{File, read_to_string}, io::LineWriter};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle07.txt")).unwrap();

    let mut grid = Grid::from_lines(
        &input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect::<Vec<String>>()
    )?;

    propagate_beams_fully(&mut grid);

    let mut writer = LineWriter::new(
        File::create(get_data_path("output/puzzle07_beams.txt")).unwrap()
    );
    grid.write_into(&mut writer)
        .map_err(|_| "Failed to write output")?;

    let mut beam_split_count = 0;
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if let Some('^') = grid.get((i, j)) && let Some('|') = grid.get((i, j - 1)) {
                beam_split_count += 1;
            }
        }
    }

    println!("Beam split count: {beam_split_count}");

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle07.txt")).unwrap();

    let mut grid = Grid::from_lines(
        &input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect::<Vec<String>>()
    )?;

    propagate_beams_fully(&mut grid);

    let mut path_count = Grid::new(grid.width(), grid.height(), 0i64);
    for i in 0..grid.width() {
        if let Some('|') = grid.get((i, grid.height() - 1)) {
            path_count.set((i, grid.height() - 1), 1);
        }
    }

    for j in (0..grid.height() - 1).rev() {
        for i in 0..grid.width() {
            match grid.get((i, j)) {
                Some('^') => {
                    if let Some(count) = path_count.get((i - 1, j + 1)) {
                        path_count.set((i, j), path_count.get((i, j)).unwrap_or(0) + count);
                    }
                    if let Some(count) = path_count.get((i + 1, j + 1)) {
                        path_count.set((i, j), path_count.get((i, j)).unwrap_or(0) + count);
                    }
                }
                Some('|') | Some('S') => {
                    if let Some(count) = path_count.get((i, j + 1)) {
                        path_count.set((i, j), count);
                    }
                }
                _ => {}
            }
        }
    }

    let start = grid.find(&'S')
        .ok_or_else(|| format!("Failed to find starting point S"))?;
    let total_path_count = path_count.get(start).unwrap_or(0);

    println!("Quantum tachyon path count: {total_path_count}");

    let mut gradient = grid.clone();
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if let Some('|') = grid.get((i, j)) && let Some(count) = path_count.get((i, j)) {
                let value = count.to_string().len().clamp(0, 15) as u32;
                gradient.set((i, j), char::from_digit(value, 16).unwrap_or('?'));
            }
        }
    }
    let mut writer = LineWriter::new(
        File::create(get_data_path("output/puzzle07_path_count.txt")).unwrap()
    );
    gradient.write_into(&mut writer)
        .map_err(|_| "Failed to write output")?;

    Ok(())
}

fn propagate_beams_fully(grid: &mut Grid<char>) {
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            let from = (i, j);
            match grid.get(from) {
                Some('S') | Some('|') => {
                    propagate_beam(grid, from);
                }
                Some('^') => {
                    if let Some('|') = grid.get((i, j - 1)) {
                        if let Some('.') = grid.get((i - 1, j)) {
                            grid.set((i - 1, j), '|');
                            propagate_beam(grid, (i - 1, j));
                        }
                        if let Some('.') = grid.get((i + 1, j)) {
                            grid.set((i + 1, j), '|');
                            propagate_beam(grid, (i + 1, j));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn propagate_beam(grid: &mut Grid<char>, from: (i32, i32)) {
    let next = (from.0, from.1 + 1);
    if let Some('.') = grid.get(next) {
        grid.set(next, '|');
    }
}
