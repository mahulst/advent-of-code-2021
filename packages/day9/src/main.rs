use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut y = 0;
    let mut x = 0;
    let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();
    input.lines().rev().for_each(|line| {
        x = 0;
        line.chars().for_each(|h| {
            let height = h.to_digit(10).unwrap() as i32;
            height_map.insert((x, y), height);
            x += 1;
        });
        y += 1;
    });

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&height_map));
    println!("Answer 2: {}", part_2(&height_map));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn part_1(height_map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut count = 0;
    height_map.iter().for_each(|((x, y), height)| {
        let top = height_map.get(&(*x, y + 1)).unwrap_or(&10);
        let bottom = height_map.get(&(*x, y - 1)).unwrap_or(&10);
        let left = height_map.get(&(x - 1, *y)).unwrap_or(&10);
        let right = height_map.get(&(x + 1, *y)).unwrap_or(&10);

        if height < top && height < bottom && height < left && height < right {
            count += height + 1
        }
    });

    count
}

fn part_2(height_map: &HashMap<(i32, i32), i32>) -> usize {
    let mut counts = vec![];
    let mut global_hash_set: HashSet<(i32, i32)> = HashSet::new();

    height_map.iter().for_each(|((x, y), height)| {
        let height = height_map.get(&(*x,*y)).unwrap_or(&10);

        if height < &9  && !global_hash_set.contains(&(*x,*y)){
            let mut local_hash_set: HashSet<(i32, i32)> = HashSet::new();
            get_neighbours(&height_map, (*x,*y), &mut local_hash_set);
            counts.push(local_hash_set.len());
            global_hash_set.extend(local_hash_set);
        }

    });
    counts.sort();
    counts.iter().rev().take(3).product::<usize>()
}

fn get_neighbours(height_map: &HashMap<(i32, i32), i32>, (x, y): (i32, i32), counted_coords: &mut HashSet<(i32, i32)>) -> i32 {
    let mut count = 0;
    let top_coord = (x, y + 1);
    let bottom_coord = (x, y - 1);
    let left_coord = (x - 1, y);
    let right_coord = (x + 1, y);

    let top = height_map.get(&top_coord).unwrap_or(&10);
    let bottom = height_map.get(&bottom_coord).unwrap_or(&10);
    let left = height_map.get(&left_coord).unwrap_or(&10);
    let right = height_map.get(&right_coord).unwrap_or(&10);

    if top < &9 && !counted_coords.contains(&top_coord) {
        counted_coords.insert(top_coord);
        // count += top + 1;
        count += get_neighbours(height_map, top_coord, counted_coords);
    }
    if bottom < &9 && !counted_coords.contains(&bottom_coord) {
        counted_coords.insert(bottom_coord);
        // count += bottom + 1;
        count += get_neighbours(height_map, bottom_coord, counted_coords);
    }
    if left < &9 && !counted_coords.contains(&left_coord) {
        counted_coords.insert(left_coord);
        // count += left + 1;
        count += get_neighbours(height_map, left_coord, counted_coords);
    }
    if right < &9 && !counted_coords.contains(&right_coord) {
        counted_coords.insert(right_coord);
        // count += right + 1;
        count += get_neighbours(height_map, right_coord, counted_coords);
    }

    // println!("@@@@@@@@@@@@@@@@@@");
    // println!("{}", counted_coords.len());
    // counted_coords.iter().for_each(|a|{
    //     println!("{:?}", a);
    // });
    // println!("@@@@@@@@@@@@@@@@@@");

    counted_coords.len() as i32
}
