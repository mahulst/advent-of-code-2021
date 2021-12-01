fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));

    let input = input_reader::read(&input_path);
    let numbers: Vec<usize> = input.lines().map(|str| { str.parse::<usize>().expect("Invalid input, expected numbers") }).collect();

    println!("Answer 1: {}", part_1(&numbers));
    println!("Answer 2: {}", part_2(&numbers));
}

fn part_1(depths: &[usize]) -> usize {
    depths.windows(2).fold(0, |total, depths| {
        if depths[0] < depths[1] {
            return total + 1;
        }
        return total;
    })
}

fn part_2(depths: &[usize]) -> usize {
    depths.windows(3).map(|depths| depths.iter().sum()).collect::<Vec<usize>>().windows(2).fold(0, |total, depths| {
        if depths[0] < depths[1] {
            return total + 1;
        }
        return total;
    })
}
