use regex::Regex;
use num;
use std::cmp;
use std::collections::{HashMap};
use std::ops::{RangeInclusive};
use std::str::FromStr;

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let lines: Vec<Line> = input
        .lines()
        .map(|str| {
            str.parse::<Line>()
                .expect("Invalid input, expected line format")
        })
        .collect();

    println!("Answer 1: {}", part_1(&lines));
    println!("Answer 2: {}", part_2(&lines));
}

fn draw_orthogonal_lines(lines: &Vec<Line>) -> HashMap<Coord, i32> {
    let map: HashMap<Coord, i32> =
        lines
            .iter()
            .filter(|line| is_orthogonal(line))
            .fold(HashMap::new(), |mut map, next| {
                for x in range(next.start.x, next.end.x) {
                    for y in range(next.start.y, next.end.y) {
                        let count = map.entry(Coord { x, y }).or_insert(0);
                        *count += 1;
                    }
                }
                map
            });

    map
}

fn draw_diagonal_lines(lines: &Vec<Line>, map: &mut HashMap<Coord, i32>) {
    lines
        .iter()
        .filter(|line| !is_orthogonal(line))
        .for_each(|next| {
            let dir_x = normalize(next.end.x, next.start.x);
            let dir_y = normalize(next.end.y, next.start.y);

            let mut i = 0;
            for _ in num::range_step(next.start.x , next.end.x + dir_x , dir_x) {
                let coord = Coord { x: next.start.x + (dir_x * i), y: next.start.y + (dir_y * i) };
                let count = map.entry(coord).or_insert(0);
                *count += 1;
                i += 1;
            }
        });
}

fn normalize(a: i32, b: i32) -> i32 {
    num::signum(a as i32 - b as i32)
}

fn part_1(lines: &Vec<Line>) -> i32 {
    let map = draw_orthogonal_lines(lines);
    let count = map.values().filter(|a| a > &&1).count();

    count as i32
}

fn part_2(lines: &Vec<Line>) -> i32 {
    let mut map: HashMap<Coord, i32> = draw_orthogonal_lines(lines);
    draw_diagonal_lines(lines, &mut map);
    let count = map.values().filter(|a| a > &&1).count();

    count as i32
}

fn range(a: i32, b: i32) -> RangeInclusive<i32> {
    cmp::min(a, b)..=cmp::max(a, b)
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Line {
    start: Coord,
    end: Coord,
}

impl FromStr for Line {
    type Err = ();
    fn from_str(input: &str) -> Result<Line, ()> {
        let re = Regex::new(r"(?P<x1>\d*),(?P<y1>\d*) -> (?P<x2>\d*),(?P<y2>\d*)$").unwrap();

        match re.captures(input) {
            Some(caps) => {
                let start = Coord {
                    x: caps["x1"].parse().unwrap(),
                    y: caps["y1"].parse().unwrap(),
                };
                let end = Coord {
                    x: caps["x2"].parse().unwrap(),
                    y: caps["y2"].parse().unwrap(),
                };

                Ok(Line { start, end })
            }
            None => Err(()),
        }
    }
}

fn is_orthogonal(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
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
