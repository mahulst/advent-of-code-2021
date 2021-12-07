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

    let fish: Vec<usize> = input
        .trim()
        .split(",")
        .map(|str| {
            str.parse::<usize>()
                .expect("Invalid input, expected number")
        })
        .collect();


    println!("Answer 1: {}", part_1(&fish));
    let now = Instant::now();
    // ... do things
    println!("{}", now.elapsed().as_micros());

}

#[derive(Debug)]
struct BunchOfFishes {
    days_left: usize,
    amount: usize,
}

fn part_1(fish: &Vec<usize>) -> usize {
    let mut fishes: Vec<BunchOfFishes> = vec![];
    for i in 0..8 {
        fishes.push(BunchOfFishes {
            amount: 0,
            days_left: i,
        });
    }

    fish.iter().for_each(|fish| {
        let mut bunch_of_fish: &mut BunchOfFishes = fishes
            .iter_mut()
            .find(|bunch_of_fish| fish == &bunch_of_fish.days_left)
            .unwrap();

        bunch_of_fish.amount += 1;
    });
    for i in 0..256 {
        let mut spawned_fishes = 0;
        fishes.iter_mut().for_each(|bunch_of_fish| {
            if bunch_of_fish.days_left == 0 {
                bunch_of_fish.days_left = 6;
                spawned_fishes += bunch_of_fish.amount;
            } else {
                bunch_of_fish.days_left -= 1;
            }
        });

        fishes.push(BunchOfFishes {
            amount: spawned_fishes,
            days_left: 8,
        });
    }


    count_fish(&fishes)
}

fn count_fish (fishes: &Vec<BunchOfFishes>) -> usize {
    fishes.iter().fold(0, |acc, next| {
        acc + next.amount
    })
}
#[cfg(test)]
mod tests {
    use crate::{Coord, Line};

    #[test]
    fn it_should_parse_row_to_enum() {
        // Arrange
        let row = "849,469 -> 648,670";

        // Act
        let line = row.parse::<Line>();

        // Assert
        assert_eq!(
            line.unwrap(),
            Line {
                start: Coord { x: 849, y: 469 },
                end: Coord { x: 648, y: 670 },
            }
        );
    }
}
