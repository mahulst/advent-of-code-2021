use std::collections::{HashMap, HashSet};

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let numbers_drawn: Vec<usize> = vec![31,50,79,59,39,53,58,95,92,55,40,97,81,22,69,26,6,23,3,29,83,48,18,75,47,49,62,45,35,34,1,88,54,16,56,77,28,94,52,15,0,87,93,90,60,67,68,85,80,51,20,96,61,66,63,91,8,99,70,13,71,17,7,38,44,43,5,25,72,2,57,33,82,78,89,21,30,11,73,84,4,46,14,19,12,10,42,32,64,98,9,74,86,27,24,65,37,41,76,36];

    let mut boards: Vec<Board> = input_reader::read(&input_path).split("\n\n").map(|board_string|{
        let mut numbers = HashMap::new();
        board_string.lines().enumerate().for_each(|(y, line)| {
            line.split_whitespace().enumerate().for_each(|(x, number)|{
                numbers.insert(number.parse::<usize>().unwrap(), (x,y));
            })
        });
        let mut board = Board::new(5);
        board.set_numbers(numbers);
        board
    }).collect();

    // println!("Answer 1: {:?}", part_1(&mut boards, &numbers_drawn));
    // boards are mutated, so either build reset or run one part at a time
    println!("Answer 2: {:?}", part_2(&mut boards.clone(), &numbers_drawn));
}

fn part_2(boards: &mut Vec<Board>, numbers_drawn: &Vec<usize>) -> Option<usize> {
    let mut boards_won = 0;
    let total_boards = boards.len();

    for num in numbers_drawn {
        for board in &mut boards[..] {
            if !board.is_complete() {
                board.draw_number(&num);

                if board.is_complete() {
                   boards_won += 1;

                    if boards_won == total_boards {
                        return Some(score_board_that_won(board, *num));
                    }
                }
            }
        }
    }

    None
}

fn part_1(boards: &mut Vec<Board>, numbers_drawn: &Vec<usize>) -> Option<usize> {
    for num in numbers_drawn {
        for board in &mut boards[..] {
            board.draw_number(&num);
            if board.is_complete() {
                return Some(score_board_that_won(board, *num));
            }
        }
    }

    None
}

fn score_board_that_won (board: &Board, lucky_num: usize) -> usize {
    let sum: usize  = board.numbers.keys().filter(|num| {!board.numbers_drawn.contains(num)}).sum();
    sum * lucky_num
}

#[derive(Debug, Clone)]
struct Board {
    rows_drawn: HashMap<usize, usize>,
    columns_drawn: HashMap<usize, usize>,
    numbers_drawn: HashSet<usize>,
    numbers: HashMap<usize, (usize, usize)>,
    size: usize,
}

impl Board {
    fn new (size: usize) -> Board {
        Board {
            rows_drawn: Default::default(),
            columns_drawn: Default::default(),
            numbers_drawn: Default::default(),
            numbers: Default::default(),
            size
        }
    }
    fn set_numbers(&mut self, numbers: HashMap<usize, (usize, usize)>) {
        self.numbers = numbers;
    }

    fn get_number(&self, num: &usize) -> Option<&(usize, usize)> {
        self.numbers.get(&num)
    }

    fn draw_number(&mut self, num: &usize) {
        self.numbers_drawn.insert(*num);

        let coord = self.get_number(num).cloned();

        if coord.is_none() {
            return;
        }
        let (x,y) = coord.unwrap();

        *self.rows_drawn.entry(y).or_insert(0) += 1;

        *self.columns_drawn.entry(x).or_insert(0) += 1;

        ;
    }

    fn is_complete(&self) -> bool {

        self.rows_drawn.values().max().unwrap_or(&0) == &self.size || self.columns_drawn.values().max().unwrap_or(&0) == &self.size
    }
}


#[cfg(test)]
mod tests {
    use crate::Board;
    use std::collections::HashMap;

    #[test]
    fn it_should_get_number() {
        // Arrange
        let mut board = Board::new(2);
        let mut numbers = HashMap::new();
        numbers.insert(1, (0,0));
        numbers.insert(2, (1,0));
        numbers.insert(3, (0,1));
        numbers.insert(4, (1,1));
        board.set_numbers(numbers);

        // Act
        let coord = board.get_number(&2);

        // Assert
        assert_eq!(coord.unwrap(), &(1,0));
    }

    #[test]
    fn it_should_draw_a_number() {
        // Arrange
        let mut board = Board::new(2);
        let mut numbers = HashMap::new();
        numbers.insert(1, (0,0));
        numbers.insert(2, (1,0));
        numbers.insert(3, (0,1));
        numbers.insert(4, (1,1));
        board.set_numbers(numbers);

        // Act
        board.draw_number(&2);

        // Assert
        assert_eq!(board.rows_drawn, HashMap::from([(0, 1)]));
        assert_eq!(board.columns_drawn, HashMap::from([(1, 1)]));
    }

    #[test]
    fn it_should_determine_win_state() {
        // Arrange
        let mut board = Board::new(2);
        let mut numbers = HashMap::new();
        numbers.insert(1, (0,0));
        numbers.insert(2, (1,0));
        numbers.insert(3, (0,1));
        numbers.insert(4, (1,1));
        board.set_numbers(numbers);
        board.draw_number(&1);
        board.draw_number(&4);
        assert_eq!(board.is_complete(), false);

        // Act
        board.draw_number(&2);

        // Assert
        assert_eq!(board.is_complete(), true);
    }
}
