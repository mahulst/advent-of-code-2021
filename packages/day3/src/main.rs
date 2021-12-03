use std::str::FromStr;
use regex::Regex;
use derive_more::Add;
use std::num::{ParseIntError, IntErrorKind};
use std::fmt::{Debug, Formatter, Pointer, Write};

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));

    let input = input_reader::read(&input_path);
    let numbers: Vec<Vec<BitCount>> = input.lines().map(|str| {
        str.chars().map(|bit| {
            match bit {
                '0' => BitCount {
                    zeros: 1,
                    ones: 0,
                },
                '1' => BitCount {
                    zeros: 0,
                    ones: 1,
                },
                _ => panic!("invalid input")
            }
        }).collect()
    }).collect();

    println!("Answer 1: {:?}", part_1(&numbers));
    println!("Answer 2: {:?}", part_2(numbers));
}

#[derive(Copy, Clone, Add)]
struct BitCount {
    zeros: usize,
    ones: usize,
}

impl Debug for BitCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(if &self.zeros > &self.ones { '0' } else { '1' })
    }
}

fn count_bits(bits: &Vec<Vec<BitCount>>) -> Vec<BitCount> {
    bits.iter().fold(vec![], |bitcounts: Vec<BitCount>, next| {
        if bitcounts.is_empty() {
            next.clone()
        } else {
            bitcounts.iter().zip(next).map(|(a, b)| {
                *a + *b
            }).collect()
        }
    })
}

fn bit_counts_to_byte_string(bit_counts: &Vec<BitCount>) -> String {
    bit_counts.iter().map(|BitCount { zeros, ones }| {
        if zeros > ones {
            "0"
        } else {
            "1"
        }
    }).collect::<Vec<&str>>().join("")
}

fn part_1(bits: &Vec<Vec<BitCount>>) -> Result<usize, ParseIntError> {
    let result = count_bits(bits);

    let gamma = bit_counts_to_byte_string(&result);
    let epsilon = result.iter().map(|BitCount { zeros, ones }| {
        if zeros > ones {
            "1"
        } else {
            "0"
        }
    }).collect::<Vec<&str>>().join("");

    let gamma_num = usize::from_str_radix(&gamma, 2)?;
    let epsilon_num = usize::from_str_radix(&epsilon, 2)?;

    return Ok(gamma_num * epsilon_num);
}

fn part_2(bits: Vec<Vec<BitCount>>) -> Result<usize, ParseIntError> {
    let co2 = (fold_bit_string(&bits, &mut vec![], compare_most_used));
    let oxygen = (fold_bit_string(&bits, &mut vec![], compare_least_used));

    let co2_num = usize::from_str_radix(&bit_counts_to_byte_string(&co2), 2)?;
    let oxygen_num = usize::from_str_radix(&bit_counts_to_byte_string(&oxygen), 2)?;

    Ok(co2_num * oxygen_num)
}

fn fold_bit_string(bits: &Vec<Vec<BitCount>>, result: &mut Vec<BitCount>, compare: fn(BitCount, &BitCount) -> bool) -> Vec<BitCount> {
    let bitcounted = count_bits(&bits);

    let look_at = result.len();

    let relevant_bit = bitcounted.get(look_at).expect("look_at not found in bits").clone();

    result.push(relevant_bit);

    let new_bits: Vec<Vec<BitCount>> = bits.into_iter().map(|a| a.to_vec()).filter(|bit_count| {
        let bit_count_at_look_at = bit_count.get(look_at).expect("look_at not found in bit_count");
        compare(relevant_bit, bit_count_at_look_at)
    }).collect();


    if new_bits.len() == 1 {
        new_bits.first().unwrap().to_vec()
    } else {
        fold_bit_string(&new_bits, result, compare)
    }
}

fn compare_most_used(relevant_bit: BitCount, bit_count_at_look_at: &BitCount) -> bool {
    if relevant_bit.zeros < relevant_bit.ones || relevant_bit.zeros == relevant_bit.ones {
        bit_count_at_look_at.ones == 1
    } else {
        bit_count_at_look_at.zeros == 1
    }
}

fn compare_least_used(relevant_bit: BitCount, bit_count_at_look_at: &BitCount) -> bool {
    if relevant_bit.zeros < relevant_bit.ones || relevant_bit.zeros == relevant_bit.ones {
        bit_count_at_look_at.zeros == 1
    } else {
        bit_count_at_look_at.ones == 1
    }
}
