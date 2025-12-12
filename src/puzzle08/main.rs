//! Day 7: Laboratories
use core_lib::{KDSpace, KDTree, MeasureElapsed, get_data_path};
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

    let tree = KDTree::construct(
        JunctionBoxSpace { boxes: boxes.clone() }
    );

    let mut edges: Vec<(usize, usize)> = vec![];
    // for (from, to) in tree.iter_edges() {
    //     let from_index = boxes.iter().position(|b| *b == from);
    //     let to_index = boxes.iter().position(|b| *b == to);
    //     if let Some(i) = from_index && let Some(j) = to_index {
    //         edges.push((i, j));
    //     }
    // }

    for (i, junction) in boxes.iter().enumerate() {
        if let Some(nearest) = tree.find_nearest(&junction, |b| *b != *junction) {
            if let Some(j) = boxes.iter().position(|b| *b == nearest) {
                edges.push((i, j));
            }
        }
    }

    let max_coord = boxes.iter()
        .flat_map(|b| [b.0, b.1, b.2])
        .max()
        .unwrap_or(1);
    let scale = 10.0 / (10.0_f64).powf((max_coord as f64).log10().round().clamp(1.0, 50.0));
    let data = PuzzleData { scale, boxes, edges };
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
    edges: Vec<(usize, usize)>,
}

struct JunctionBoxSpace {
    boxes: Vec<JunctionBox>,
}

impl KDSpace for JunctionBoxSpace {
    type Key = i32;
    type Item = JunctionBox;
    type Distance = f64;

    fn items(self) -> impl Iterator<Item = Self::Item> {
        self.boxes.into_iter()
    }

    fn get_key(item: &Self::Item, depth: usize) -> Self::Key {
        match depth % 3 {
            0 => item.0,
            1 => item.1,
            _ => item.2,
        }
    }

    fn key_distance(from: &Self::Key, to: &Self::Key) -> Self::Distance {
        return ((from - to) as f64).powf(2.0);
    }

    fn item_distance(from: &Self::Item, to: &Self::Item) -> Self::Distance {
        return
            ((from.0 - to.0) as f64).powf(2.0) +
            ((from.1 - to.1) as f64).powf(2.0) +
            ((from.2 - to.2) as f64).powf(2.0);
    }
}
