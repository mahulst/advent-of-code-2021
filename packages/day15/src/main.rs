use pathfinding::prelude::{absdiff, astar};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::time::Instant;
type Map = HashMap<Coord, isize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut map: Map = HashMap::new();

    let mut y = 0;
    input.lines().for_each(|line| {
        let mut x = 0;
        line.chars().for_each(|c| {
            let risk = c.to_digit(10).unwrap();

            map.insert(Coord { x, y }, risk as isize);
            x += 1;
        });
        y += 1;
    });

    let now = Instant::now();

    print_map(&map);

    // let new_map = find_distances(&map, &Coord { x: 0, y: 0 });
    let new_map_2 = extend_map(&map);
    let max_x = new_map_2.keys().map(|coord| coord.x).max().unwrap();
    let max_y = new_map_2.keys().map(|coord| coord.x).max().unwrap();

    let result = astar(
        &Coord { x: 0, y: 0 },
        |p| {
            let neighbours = get_neighbours(&new_map_2, p);

            neighbours
                .into_iter()
                .map(|p| (p, *new_map_2.get(&p).unwrap()))
        },
        |p| ((p.x - max_x).abs() + (p.y - max_y).abs()) / 3,
        |p| *p == Coord { x: max_x, y: max_y },
    );
    println!("Answer 2: {:?}", result.unwrap());


    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );

    let new_map_3 = find_distances(&new_map_2, &Coord { x: 0, y: 0 });
    // print_map(&new_map_3);
    // println!("Answer 2: {:?}", get_last(&new_map_3));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn get_last(map: &Map) -> isize {
    let max_x = map.keys().map(|coord| coord.x).max().unwrap();
    let max_y = map.keys().map(|coord| coord.x).max().unwrap();
    dbg!(max_x, max_y);
    *map.get(&Coord { x: max_x, y: max_y }).unwrap()
}

fn extend_map(map: &Map) -> Map {
    let width = map.keys().map(|coord| coord.x).max().unwrap() + 1;

    let mut new_map: Map = HashMap::new();

    for (coord, risk) in map {
        for i in 0..5 {
            for j in 0..5 {
                let mut new_risk = risk + i + j;
                if new_risk > 9 {
                    new_risk -= 9;
                }
                let new_coord = Coord {
                    x: coord.x + width * i,
                    y: coord.y + width * j,
                };
                new_map.insert(new_coord, new_risk);
            }
        }
    }

    new_map
}

fn get_neighbours<'a>(map: &'a Map, coord: &'a Coord) -> Vec<Coord> {
    let directions = vec![
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: -1 },
        Coord { x: -1, y: 0 },
    ];

    return directions
        .iter()
        .filter_map(|direction| -> Option<Coord> {
            map.get(&(*direction + *coord)).map(|_| *direction + *coord)
        })
        .collect();
}

fn find_distances(map: &Map, coord: &Coord) -> Map {
    let mut distances: Map = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut to_visit: VecDeque<(Coord, isize)> = VecDeque::new();
    let mut to_visit_set: HashMap<Coord, isize> = HashMap::new();
    let d = map.get(&Coord { x: 0, y: 0 }).unwrap();

    to_visit.push_front((*coord, 0));
    to_visit_set.insert(*coord, 0);

    while !to_visit.is_empty() {
        let (coord, distance) = to_visit.pop_back().unwrap();
        visited.insert(coord);
        if visited.len() % 1000 == 0 {
            // println!("visited {}", visited.len());
        }

        let current_distance = distances.get(&coord).unwrap_or(&99999);
        if distance < *current_distance {
            // println!("{:?} risk:({}) distance: {} result: {} (was {})", coord, risk, distance, distance + risk, current_distance);
            distances.insert(coord, distance);
        }

        for neighbour in get_neighbours(map, &coord) {
            let val = to_visit_set.get(&neighbour).unwrap_or(&9999999999);
            if !visited.contains(&neighbour) {
                let neighbour_risk = map.get(&neighbour).unwrap();

                if distance + neighbour_risk < *val {
                    to_visit.retain(|(c, score)| *c != neighbour);

                    // println!("{:?} distance: {}, neighbour_risk: {}", neighbour, distance, neighbour_risk);
                    to_visit.push_front((neighbour.clone(), distance + neighbour_risk));
                    to_visit_set.insert(neighbour.clone(), distance + neighbour_risk);
                }
            }
        }
    }

    distances
}

fn print_map(map: &Map) {
    let max_x = map.keys().map(|coord| coord.x).max().unwrap();
    let max_y = map.keys().map(|coord| coord.x).max().unwrap();

    for y in 0..=max_x {
        for x in 0..=max_y {
            print!("{:04}", map.get(&Coord { x, y }).unwrap_or(&0));
        }
        println!();
    }
}
