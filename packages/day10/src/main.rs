use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let rows: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&rows));
    println!("Answer 2: {}", part_2(&rows));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn part_1(rows: &Vec<Vec<char>>) -> usize {
    let corrupted_chars: Vec<char> = rows.iter().filter_map(|row| {
        let mut corrupted_chars: Vec<char> = vec![];

        row.iter().fold(&mut vec![], |opened_chars, next|{
            if is_closing(opened_chars.last(), *next) {
                opened_chars.pop();
            } else {
                if is_opening_new(*next) {
                    opened_chars.push(*next);
                } else {
                    corrupted_chars.push(*next);
                }
            }

            opened_chars
        });

        corrupted_chars.first().map(|a| *a)
    }).collect();

    corrupted_chars.iter().map(|c| score_corrupted_chars(*c)).sum()
}


fn part_2(rows: &Vec<Vec<char>>) -> usize {
    let opened_chars: Vec<Vec<char>> = rows.iter().filter_map(|row| {
        let mut corrupted_chars: Vec<char> = vec![];

        let opened_chars = row.iter().fold(&mut vec![], |opened_chars, next|{
            if is_closing(opened_chars.last(), *next) {
                opened_chars.pop();
            } else {
                if is_opening_new(*next) {
                    opened_chars.push(*next);
                } else {
                    corrupted_chars.push(*next);
                }
            }

            opened_chars
        }).to_owned();

        if corrupted_chars.len() == 0 {
            Some(opened_chars)
        } else {
            None
        }
    }).collect();

    let mut totals: Vec<usize> = opened_chars.iter().map(|opened_char| {
        opened_char.iter().rev().fold(0, |total, c|{
            let value =  match *c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            };

            total * 5 + value
        })
    }).collect();

    totals.sort();
    *totals.get(totals.len() / 2).unwrap() as usize
}

fn score_corrupted_chars(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn is_opening_new(c: char) -> bool {
    match c {
        '[' => true,
        '(' => true,
        '{' => true,
        '<' => true,
        _ => false
    }
}

fn is_closing(last: Option<&char>, next: char ) -> bool {
    match last {
        Some('[') => next == ']',
        Some('(') => next == ')',
        Some('{') => next == '}',
        Some('<') => next == '>',
        _ => false
    }
}
