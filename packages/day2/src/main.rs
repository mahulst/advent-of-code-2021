use std::str::FromStr;
use regex::Regex;

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));

    let input = input_reader::read(&input_path);
    let numbers: Vec<Movements> = input.lines().map(|str| { str.parse::<Movements>().expect("Invalid input, expected movements") }).collect();

    println!("Answer 1: {}", part_1(&numbers));
    println!("Answer 2: {}", part_2(&numbers));
}

#[derive(PartialEq, Debug)]
pub enum Movements {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Movements {
    type Err = ();
    fn from_str(input: &str) -> Result<Movements, ()> {
        let re = Regex::new(r"(?P<direction>[a-z]*) (?P<amount>\d*)$").unwrap();

        match re.captures(input) {
            Some(caps) => {
                let movement = caps["direction"].parse::<String>().expect("Unrecognized input");
                let result = match movement.as_ref() {
                    "forward" => Ok(Movements::Forward(caps["amount"].parse().unwrap())),
                    "up" => Ok(Movements::Up(caps["amount"].parse().unwrap())),
                    "down" => Ok(Movements::Down(caps["amount"].parse().unwrap())),
                    _ => Err(())
                };
                result
            }
            None => Err(()),
        }
    }
}

fn part_1(depths: &[Movements]) -> usize {
    let result = depths.iter().fold((0,0), |(x, y), next| {
        match next {
            Movements::Forward(amount) => (x + amount, y),
            Movements::Up(amount) => (x, y - amount ),
            Movements::Down(amount) => (x, y + amount),
        }
    });

    result.0 * result.1
}

fn part_2(depths: &[Movements]) -> usize {
    let result = depths.iter().fold((0,0, 0), |(x, y, aim), next| {
        match next {
            Movements::Forward(amount) => (x + amount, y + aim * amount, aim),
            Movements::Up(amount) => (x, y , aim - amount ),
            Movements::Down(amount) => (x, y, aim + amount),
        }
    });

    result.0 * result.1
}

#[cfg(test)]
mod tests {
    use crate::Movements;

    #[test]
    fn it_should_parse_row_to_enum() {
        // Arrange
        let row = "forward 15";

        // Act
        let movement = row.parse::<Movements>();

        // Assert
        assert_eq!(movement.unwrap(), Movements::Forward(15));
    }
}
