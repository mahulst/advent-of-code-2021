use std::time::{Instant};

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);

    let signal: Vec<&str> = input
        .trim()
        .lines()
        .map(|line| line.split("|").last().unwrap().trim())
        .collect();

    let signal2: Vec<(&str, &str)> = input
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            (split.next().unwrap().trim(), split.last().unwrap().trim())
        })
        .collect();

    let now = Instant::now();
    println!("Answer 1: {}", part_1(&signal));
    println!("Answer 2: {}", part_2(&signal2));

    println!("Solution took {} ms", now.elapsed().as_micros() as f64 / 1000.0);
}

fn part_1(signal: &Vec<&str>) -> usize {
    signal.into_iter().fold(0, |count, next| {
        let lengths: Vec<&str> = next
            .split(" ")
            .filter(|single| {
                let len = single.len();
                len == 2 || len == 3 || len == 4 || len == 7
            })
            .collect();

        lengths.len() + count
    })
}

type Number = Vec<char>;

#[derive(Debug)]
struct Input {
    numbers: Vec<Number>,
    output: Vec<Number>,
}

fn part_2(signal: &Vec<(&str, &str)>) -> usize {
    let vec: Vec<Input> = signal
        .iter()
        .map(|(numbers_raw, output_raw)| {
            let numbers: Vec<Number> = numbers_raw
                .split(" ")
                .map(|a| a.chars().collect())
                .collect();
            let output: Vec<Number> = output_raw.split(" ").map(|a| a.chars().collect()).collect();
            Input { numbers, output }
        })
        .collect();

    vec.iter().map(|input| {
        let one = input.numbers.iter().find(|a| a.len() == 2).unwrap();
        let four = input.numbers.iter().find(|a| a.len() == 4).unwrap();
        let seven = input.numbers.iter().find(|a| a.len() == 3).unwrap();
        let eight = input.numbers.iter().find(|a| a.len() == 7).unwrap();

        let six_lengths: Vec<&Number> = input.numbers.iter().filter(|a| a.len() == 6).collect();
        let mut zero: &Number = &vec![];
        let mut six: &Number = &vec![];
        let mut nine: &Number = &vec![];

        six_lengths.iter().for_each(|six_length_number| {
            // check if 6 hasn't been set
            if overlaps_digit(*six_length_number, one).len() != one.len() {
                six = six_length_number.clone();
            } else if overlaps_digit(*six_length_number, four).len() != four.len() {
                zero = six_length_number.clone();
            } else {
                nine = six_length_number.clone();
            }
        });

        let five_lengths: Vec<&Number> = input.numbers.iter().filter(|a| a.len() == 5).collect();
        let mut three: &Number = &vec![];
        let mut five: &Number = &vec![];
        let mut two: &Number = &vec![];

        five_lengths.iter().for_each(|five_length_number| {
            if overlaps_digit(*five_length_number, one).len() == one.len() {
                three = five_length_number.clone();
            } else {
                let top_right = diff_digit(&six, &one);

                if overlaps_digit(five_length_number, &top_right).len() > 0 {
                    two = five_length_number.clone();
                } else {
                    five = five_length_number.clone();
                }
            }
        });

        let output: Vec<&str> = input
            .output
            .iter()
            .map(|number| {
                if match_digit(number, one) {
                    return "1";
                } else if match_digit(number, two) {
                    return "2";
                } else if match_digit(number, three) {
                    return "3";
                } else if match_digit(number, four) {
                    return "4";
                } else if match_digit(number, five) {
                    return "5";
                } else if match_digit(number, six) {
                    return "6";
                } else if match_digit(number, seven) {
                    return "7";
                } else if match_digit(number, eight) {
                    return "8";
                } else if match_digit(number, nine) {
                    return "9";
                } else if match_digit(number, zero) {
                    return "0";
                }

                panic!("unmatched number");
            })
            .collect();

        output.join("").parse::<usize>().unwrap()
    }).sum()
}

fn match_digit(a: &Number, b: &Number) -> bool {
    a.len() == b.len() && a.iter().all(|char| b.contains(char))
}

fn overlaps_digit(a: &Number, b: &Number) -> Number {
    let overlap = b
        .iter()
        .filter(|number| a.contains(number))
        .map(|a| a.clone())
        .collect();

    overlap
}

fn diff_digit(a: &Number, b: &Number) -> Number {
    let overlap = b
        .iter()
        .filter(|number| !a.contains(number))
        .map(|a| a.clone())
        .collect();

    overlap
}

#[cfg(test)]
mod tests {
    use crate::{diff_digit, match_digit, overlaps_digit};

    #[test]
    fn it_should_return_overlapping_chars() {
        // Arrange
        let a = vec!['a', 'b', 'c'];
        let b = vec!['b', 'c', 'd'];

        // Act
        let result = overlaps_digit(&a, &b);

        // Assert
        assert_eq!(result, vec!['b', 'c']);
    }

    #[test]
    fn it_should_get_top_right() {
        // Arrange
        let six = vec!['d', 'e', 'f', 'b', 'g', 'c'];
        let one = vec!['b', 'a'];

        // Act
        let result = diff_digit(&six, &one);

        // Assert
        assert_eq!(result, vec!['a']);
    }

    #[test]
    fn it_should_match_equal() {
        // Arrange
        let a = vec!['a', 'b'];
        let b = vec!['b', 'a'];
        let c = vec!['b'];
        let d = vec!['b', 'c'];

        // Assert
        assert_eq!(match_digit(&a, &b), true);
        assert_eq!(match_digit(&a, &c), false);
        assert_eq!(match_digit(&a, &d), false);
    }
}
