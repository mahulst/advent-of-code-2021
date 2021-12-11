use std::collections::{HashMap, HashSet};
use std::time::Instant;

const OCTO_SIZE: isize = 10;
type Octos = HashMap<(isize, isize), usize>;
fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut octos: Octos = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    input.lines().for_each(|line| {
        x = 0;
        line.chars().map(|c| c.to_digit(10).unwrap()).for_each(
            (|c| {
                octos.insert((x, y), c as usize);
                x += 1;
            }),
        );
        y += 1;
    });

    let now = Instant::now();
    print_octos(&octos);
    // println!("Answer 1: {}", part_1(&mut octos));
    println!("Answer 2: {}", part_2(&mut octos));

    print_octos(&octos);
    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn update_octo(octos: &mut Octos, flashed: &mut HashSet<(isize, isize)>, coord: (isize, isize)) {
    let directions: Vec<(isize, isize)> = vec![
        (1, 0), // RIGHT
        (1, 1),
        (0, 1), // TOP
        (-1, 1),
        (-1, 0), // BOTTOM
        (-1, -1),
        (0, -1), // LEFT
        (1, -1),
    ];

    let octo = octos.get_mut(&coord);
    if let Some(o) = octo {
        // println!("updating {:?} to ({})", coord, *o + 1);
        if *o == 9 {
            // println!("flash {:?}", coord);
            *o = 0;
            flashed.insert(coord);
            for direction in directions {
                update_octo(octos, flashed, (coord.0 + direction.0, coord.1 + direction.1));
            }
        } else {
            if !flashed.contains(&coord) {
                *o += 1;
            }
        }
    }
}

fn part_1(octos: &mut Octos) -> usize {
    let mut total_flashes = 0;
    for i in 0..=99 {
        let mut flashed = HashSet::new();
        for y in 0..OCTO_SIZE {
            for x in 0..OCTO_SIZE {
                update_octo(octos, &mut flashed, (x, y));
            }
        }
        total_flashes += flashed.len();
    }
    total_flashes
}

fn part_2(octos: &mut Octos) -> usize {
    for i in 1..=999 {
        let mut flashed = HashSet::new();
        for y in 0..OCTO_SIZE {
            for x in 0..OCTO_SIZE {
                update_octo(octos, &mut flashed, (x, y));
            }
        }

        if flashed.len() == 100 {
            return i;
        }
    }

    return 0;
}

fn print_octos(octos: &Octos) {
    for y in 0..OCTO_SIZE {
        for x in 0..OCTO_SIZE {
            print!("{}", octos.get(&(x, y)).unwrap());
        }
        print!("\n");
    }
}
