//! Day 8: Playground
use core_lib::{KDSpace, KDTree, MeasureElapsed, get_data_path};
use facet::Facet;
use priority_queue::PriorityQueue;
use std::{collections::{HashMap, HashSet}, fs::{File, read_to_string}, io::LineWriter};

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

    let box_indices: HashMap<_, _> = boxes.iter()
        .enumerate()
        .map(|(k, &v)| (v, k))
        .collect();

    let tree = KDTree::construct(
        JunctionBoxSpace { boxes: boxes.clone() }
    );

    let mut edges: Vec<(usize, usize)> = vec![];

    // Visualize kd-tree:
    // for (from, to) in tree.iter_edges() {
    //     let from_index = boxes.iter().position(|b| *b == from);
    //     let to_index = boxes.iter().position(|b| *b == to);
    //     if let Some(i) = from_index && let Some(j) = to_index {
    //         edges.push((i, j));
    //     }
    // }

    // Visualize nearest box for each one:
    // for (i, junction) in boxes.iter().enumerate() {
    //     if let Some(nearest) = tree.find_nearest(&junction, |b| *b != *junction) {
    //         if let Some(&j) = box_indices.get(&nearest) {
    //             edges.push((i, j));
    //         }
    //     }
    // }

    let mut min_edges = PriorityQueue::new();
    let mut visited_graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, _) in boxes.iter().enumerate() {
        push_min_edge(&mut min_edges, i, &boxes, &box_indices, &tree, &visited_graph);
    }

    let mut connection_count = 1000;
    while connection_count > 0 {
        if let Some((edge, _)) = min_edges.pop() {
            // eprintln!("  Pop {edge:?} {}", d.0);
            if !has_edge(&visited_graph, &edge) {
                connection_count -= 1;
                visited_graph.entry(edge.0).or_default().insert(edge.1);
                visited_graph.entry(edge.1).or_default().insert(edge.0);
                push_min_edge(&mut min_edges, edge.0, &boxes, &box_indices, &tree, &visited_graph);
                push_min_edge(&mut min_edges, edge.1, &boxes, &box_indices, &tree, &visited_graph);
            }
        } else {
            break;
        }
    }

    for (source, targets) in visited_graph.iter() {
        for target in targets {
            if *source < *target {
                edges.push((*source, *target));
            }
        }
    }

    let mut visited = HashSet::new();
    let mut component_sizes = Vec::new();
    for i in 0..boxes.len() {
        let component_size = visit_component(&visited_graph, i, &mut visited);
        if component_size > 1 {
            component_sizes.push(component_size);
        }
    }
    component_sizes.sort();
    component_sizes.reverse();
    let result = component_sizes[0..3].iter().fold(1, |acc, v| acc * v);
    println!("Multiplied sized of 3 largest components: {result}");

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

fn push_min_edge(
    min_edges: &mut PriorityQueue<(usize, usize), EdgePriority>,
    from_index: usize,
    boxes: &Vec<JunctionBox>,
    box_indices: &HashMap<JunctionBox, usize>,
    tree: &KDTree<JunctionBoxSpace>,
    visited_graph: &HashMap<usize, HashSet<usize>>
) {
    let junction = boxes[from_index];
    if let Some(to) = tree.find_nearest(&junction, |b| {
        let b_index = *box_indices.get(b).unwrap();
        return b_index != from_index && !has_edge(visited_graph, &(from_index, b_index));
    }) {
        if let Some(&to_index) = box_indices.get(&to) {
            let distance = JunctionBoxSpace::item_distance(&junction, &to);
            // eprintln!("Push {:?}: {distance}", normalize_edge((from_index, to_index)));
            min_edges.push(
                (from_index, to_index),
                EdgePriority(distance)
            );
        }
    }
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
#[facet(rename_all = "camelCase")]
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

#[derive(PartialEq)]
struct EdgePriority(f64);

impl Eq for EdgePriority {}

impl PartialOrd for EdgePriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|ordering| ordering.reverse())
    }
}

impl Ord for EdgePriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn has_edge(graph: &HashMap<usize, HashSet<usize>>, edge: &(usize, usize)) -> bool {
    match graph.get(&edge.0) {
        Some(targets) => targets.contains(&edge.1),
        _ => match graph.get(&edge.1) {
            Some(targets) => targets.contains(&edge.0),
            _ => false,
        },
    }
}

fn visit_component(
    graph: &HashMap<usize, HashSet<usize>>,
    from_index: usize,
    visited: &mut HashSet<usize>
) -> i64 {
    let mut component_size = 0;
    if visited.contains(&from_index) {
        return component_size;
    }
    let mut stack = vec![from_index];
    visited.insert(from_index);
    while let Some(source) = stack.pop() {
        component_size += 1;
        if let Some(targets) = graph.get(&source) {
            for target in targets {
                if !visited.contains(target) {
                    visited.insert(*target);
                    stack.push(*target);
                }
            }
        }
    }
    return component_size;
}
