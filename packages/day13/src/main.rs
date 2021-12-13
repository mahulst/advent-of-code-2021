use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Points = HashMap<(isize, isize), bool>;

enum Fold {
    Y(isize),
    X(isize),
}

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let mut points: Points = HashMap::new();
    // let folds: Vec<Fold> = vec![Fold::Y(7), Fold::X(5)];
    let folds: Vec<Fold> = vec![
        Fold::X(655),
        Fold::Y(447),
        Fold::X(327),
        Fold::Y(223),
        Fold::X(163),
        Fold::Y(111),
        Fold::X(81),
        Fold::Y(55),
        Fold::X(40),
        Fold::Y(27),
        Fold::Y(13),
        Fold::Y(6),
    ];
    input.lines().for_each(|line| {
        let mut a = line.split(",");
        let x = a.next().unwrap();
        let y = a.next().unwrap();

        points.insert((x.parse().unwrap(), y.parse().unwrap()), true);
    });

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&points, folds.get(0).unwrap()));
    println!("Answer 2:");
    part_2(&points, folds);

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn part_1(points: &Points, fold: &Fold) -> usize {
    let fold_num: &isize = match fold {
        Fold::X(x) => x,
        Fold::Y(y) => y,
    };

    let mut new_points: Points = HashMap::new();
    for ((x, y), bool) in points {
        if x > fold_num {
            let new_x = fold_num * 2 - x;
            new_points.insert((new_x, *y), true);
        } else {
            new_points.insert((*x, *y), true);
        }
    }

    new_points.len()
}

fn part_2(points: &Points, folds: Vec<Fold>) -> usize {
    let mut new_points = folds.iter().fold(points.clone(), |points, next_fold| {
        let new_points = match next_fold {
            Fold::X(x) => fold_x(&points, *x),
            Fold::Y(y) => fold_y(&points, *y),
        };
        new_points
    });

    print_paper(&new_points);
    new_points.len()
}

fn print_paper(points: &Points) {
    for y in 0..6 {
        for x in 0..40 {
            if points.get(&(x as isize, y as isize)).is_some() {
                print!("#");
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn fold_x(points: &Points, fold: isize) -> Points {
    let mut new_points: Points = HashMap::new();
    for ((x, y), bool) in points {
        if x > &fold {
            let new_x = fold * 2 - x;
            new_points.insert((new_x, *y), true);
        } else {
            new_points.insert((*x, *y), true);
        }
    }

    new_points
}

fn fold_y(points: &Points, fold: isize) -> Points {
    let mut new_points: Points = HashMap::new();
    for ((x, y), bool) in points {
        if y > &fold {
            let new_y = fold * 2 - y;
            new_points.insert((*x, new_y), true);
        } else {
            new_points.insert((*x, *y), true);
        }
    }

    new_points
}
