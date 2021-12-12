use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Nodes = HashMap<String, Vec<String>>;

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut nodes: Nodes = HashMap::new();

    input.lines().for_each(|line| {
        let mut a = line.split("-");
        let start = a.next().unwrap();
        let end = a.next().unwrap();

        let paths = nodes.entry(start.to_string()).or_insert(vec![]);
        paths.push(end.to_string());

        let paths = nodes.entry(end.to_string()).or_insert(vec![]);
        paths.push(start.to_string())
    });

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&nodes));
    println!("Answer 2: {}", part_2(&nodes));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn get_num_paths<'a>(node: &'a str, nodes: &Nodes, visited: &mut HashSet<&'a str>) -> usize {
    if node == "end" {
        return 1;
    }
    let has_visited = visited.contains(node);
    visited.insert(node);

    let path_ways = nodes.get(node).unwrap();

    let mut total = 0;
    for path in path_ways {
        if is_small_cave(&node) && has_visited {
            continue;
        }

        let mut path_visited = &mut visited.clone();
        total += get_num_paths(path, nodes, &mut path_visited);
    }

    total
}

fn get_num_paths_double<'a>(
    node: &'a str,
    nodes: &Nodes,
    visited: &mut HashSet<&'a str>,
    visited_double: bool,
) -> usize {
    if node == "end" {
        return 1;
    }

    let mut visited_double_this_path = false;

    let has_visited = visited.contains(node);
    visited.insert(node);

    let path_ways = nodes.get(node).unwrap();

    let mut total = 0;
    for path in path_ways {
        if path == "start" {
            continue;
        }

        if is_small_cave(node) && has_visited {
            if visited_double {
                return 0;
            }

            visited_double_this_path = true
        }

        let mut path_visited = &mut visited.clone();
        total += get_num_paths_double(
            path,
            nodes,
            &mut path_visited,
            visited_double_this_path || visited_double,
        );
    }

    total
}

fn is_small_cave(node: &str) -> bool {
    node.to_lowercase() == node
}

fn part_1(nodes: &Nodes) -> usize {
    get_num_paths("start", nodes, &mut HashSet::new())
}

fn part_2(nodes: &Nodes) -> usize {
    get_num_paths_double("start", nodes, &mut HashSet::new(), false)
}
