use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Segments = HashMap<(char, char), char>;
type Polymer = HashMap<(char, char), i64>;
fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut segments: Segments = HashMap::new();
    // let chars: Vec<char> = vec!['N', 'N', 'C', 'B'];

    let chars: Vec<char> = vec![
        'O', 'F', 'S', 'V', 'V', 'S', 'F', 'O', 'C', 'B', 'N', 'O', 'N', 'H', 'K', 'F', 'H', 'N',
        'P', 'K',
    ];

    let mut start: Polymer = HashMap::new();

    chars.windows(2).for_each(|window| {
        let a = window.get(0).unwrap();
        let b = window.get(1).unwrap();

        let mut count = start.entry((*a, *b)).or_insert(0);
        *count += 1;
    });

    input.lines().for_each(|line| {
        let mut a = line.split(" -> ");
        let window = a.next().unwrap();
        let added = a.next().unwrap();

        let chars: Vec<char> = window.chars().collect();

        segments.insert(
            (*chars.get(0).unwrap(), *chars.get(1).unwrap()),
            added.chars().next().unwrap(),
        );
    });

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&segments, &start, 10));
    println!("Answer 2: {}", part_1(&segments, &start, 40));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn part_1(segments: &Segments, start: &Polymer, steps: i64) -> i64 {
    let mut start_polymer = start.clone();

    for i in 0..steps {
        let mut step_polymer: Polymer = HashMap::new();
        segments.iter().for_each(|((a, b), extra)| {
            if let Some(count) = start_polymer.get(&(*a, *b)) {
                let count_a = step_polymer.entry((*a, *extra)).or_insert(0);
                *count_a += count;
                let count_b = step_polymer.entry((*extra, *b)).or_insert(0);
                *count_b += count;
            }
        });

        start_polymer = step_polymer;
    }

    count(&start_polymer)
}

fn count(polymer: &Polymer) -> i64 {
    let mut map = HashMap::new();
    polymer.iter().for_each(|((a, b), count)| {
        let a = map.entry(b).or_insert(0);
        *a += count;
    });

    let (_, max) = map
        .iter()
        .max_by(|(_, num_a), (_, num_b)| num_a.cmp(num_b))
        .unwrap();
    let (_, min) = map
        .iter()
        .max_by(|(_, num_a), (_, num_b)| num_b.cmp(num_a))
        .unwrap();

    max - min
}
