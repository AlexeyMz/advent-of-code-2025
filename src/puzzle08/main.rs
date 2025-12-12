//! Day 7: Laboratories
use core_lib::{MeasureElapsed, get_data_path};
use facet::Facet;
use std::{fs::{File, read_to_string}, io::LineWriter};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    // advanced().unwrap();
    // time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle08.txt")).unwrap();
    let boxes = parse_junction_boxes(&input)?;

    let max_coord = boxes.iter()
        .flat_map(|b| [b.0, b.1, b.2])
        .max()
        .unwrap_or(1);
    let scale = 10.0 / (10.0_f64).powf((max_coord as f64).log10().round().clamp(1.0, 50.0));
    let data = PuzzleData { scale, boxes };
    let mut writer = LineWriter::new(
        File::create(get_data_path("output/puzzle08_data.json")).unwrap()
    );
    facet_json::to_writer_std_pretty(&mut writer, &data)
        .map_err(|err| format!("Failed to serialize puzzle data to JSON: {:?}", err))?;

    return Ok(());
}

// fn advanced() -> Result<(), String> {
//     let input = read_to_string(get_data_path("input/puzzle08_test.txt")).unwrap();
//     Ok(())
// }

type JunctionBox = (i32, i32, i32);

fn parse_junction_boxes(input: &str) -> Result<Vec<JunctionBox>, String> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_junction_box(line))
        .collect::<Result<Vec<_>, _>>()
}

fn parse_junction_box(line: &str) -> Result<JunctionBox, String> {
    let coords = line.splitn(3, ',')
        .map(|p| p.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();

    if let Ok(coords) = coords && let [x, y, z] = &coords[..] {
        Ok((*x, *y, *z))
    } else {
        Err(format!("Invalid junction box position: {line}"))
    }
}

#[derive(Facet)]
struct PuzzleData {
    scale: f64,
    boxes: Vec<JunctionBox>,
}
