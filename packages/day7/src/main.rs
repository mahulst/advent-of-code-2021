use num;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::{Instant, Duration};

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let crab_submarines: Vec<usize> = input
        .trim()
        .split(",")
        .map(|str| {
            str.parse::<usize>()
                .expect("Invalid input, expected number")
        })
        .collect();

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&crab_submarines));
    println!("Answer 2: {}", part_2(&crab_submarines));

    println!("{}", now.elapsed().as_micros());

}

fn part_1(crab_subs: &Vec<usize>) -> usize {
    let max = crab_subs.iter().max().unwrap();
    let min = crab_subs.iter().min().unwrap();

    let mut fuel_spent: Vec<(usize, usize)> = vec![];
    for i in *min..*max {
        let fuel: i32 = crab_subs.iter().fold(0, |acc, next|{
            let diff= *next as i32 - i as i32;

            acc + diff.abs()
        });
        fuel_spent.push((i, fuel as usize));
    }

    let min_fuel = fuel_spent.iter().min_by(|a, b |{ a.1.cmp(&b.1)});

    min_fuel.unwrap().1
}

fn part_2(crab_subs: &Vec<usize>) -> usize {
    let max = crab_subs.iter().max().unwrap();
    let min = crab_subs.iter().min().unwrap();

    let mut fuel_spent: Vec<(usize, usize)> = vec![];
    for i in *min..*max {
        let fuel: i32 = crab_subs.iter().fold(0, |acc, next|{
            let diff= *next as i32 - i as i32;
            let target = diff.abs();
            acc + target * (target + 1) / 2
        });
        fuel_spent.push((i, fuel as usize));
    }

    let min_fuel = fuel_spent.iter().min_by(|a, b |{ a.1.cmp(&b.1)});

    min_fuel.unwrap().1
}

