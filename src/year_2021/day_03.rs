//! Day 3: Binary Diagnostic

use crate::{enums::Part, util::output::Unsolved};
use std::fs;

pub fn solve(part: Part) -> Result<u64, Unsolved> {
    let codes = read_codes("inputs/year_2021/day_03.txt");

    match part {
        Part::Part1 => Ok(get_power_consumption(&codes)),
        Part::Part2 => Ok(get_life_support_rating(&codes)),
    }
}

fn read_codes(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("Cannot read input.")
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| if character == '0' { 0 } else { 1 })
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn get_power_consumption(codes: &[Vec<u8>]) -> u64 {
    let total_codes = codes.len();
    let code_length = codes[0].len();

    let gamma: String = (0..code_length)
        .map(|x| {
            let ones = (0..total_codes)
                .map(|y| codes[y][x] as usize)
                .sum::<usize>();
            let zeros = total_codes - ones;
            (zeros, ones)
        })
        .map(|(zeros, ones)| if zeros > ones { '0' } else { '1' })
        .collect();

    let gamma = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = 2u64.pow(code_length as u32) - gamma - 1;

    gamma * epsilon
}

fn get_life_support_rating(codes: &[Vec<u8>]) -> u64 {
    let code_length = codes[0].len();

    let o2_rating: String = {
        let mut o2_codes = codes.to_owned();
        for index in 0..code_length {
            let zeros = (0..o2_codes.len())
                .filter(|&y| o2_codes[y][index] == 0)
                .count();
            let ones = o2_codes.len() - zeros;
            o2_codes = {
                let target_value = if zeros > ones { 0 } else { 1 };
                o2_codes
                    .iter()
                    .filter_map(|code| match code[index] {
                        x if x == target_value => Some(code.clone()),
                        _ => None,
                    })
                    .collect()
            };
            if o2_codes.len() == 1 {
                break;
            }
        }

        o2_codes[0]
            .iter()
            .map(|&x| if x == 0 { '0' } else { '1' })
            .collect()
    };

    let co2_rating: String = {
        let mut co2_codes = codes.to_owned();
        for index in 0..code_length {
            let zeros = (0..co2_codes.len())
                .filter(|&y| co2_codes[y][index] == 0)
                .count();
            let ones = co2_codes.len() - zeros;
            co2_codes = {
                let target_value = if zeros > ones { 1 } else { 0 };
                co2_codes
                    .iter()
                    .filter_map(|code| match code[index] {
                        x if x == target_value => Some(code.clone()),
                        _ => None,
                    })
                    .collect()
            };
            if co2_codes.len() == 1 {
                break;
            }
        }

        co2_codes[0]
            .iter()
            .map(|&x| if x == 0 { '0' } else { '1' })
            .collect()
    };

    let o2_rating = u64::from_str_radix(&o2_rating, 2).unwrap();
    let co2_rating = u64::from_str_radix(&co2_rating, 2).unwrap();

    o2_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::{get_life_support_rating, get_power_consumption, read_codes, solve};
    use crate::enums::Part;

    #[test]
    fn part1_example() {
        let input = read_codes("inputs/year_2021/day_03.example_01.txt");
        assert_eq!(get_power_consumption(&input), 198);
    }

    #[test]
    fn part2_example() {
        let input = read_codes("inputs/year_2021/day_03.example_01.txt");
        assert_eq!(get_life_support_rating(&input), 230);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(solve(Part::Part1).unwrap(), 4160394);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(solve(Part::Part2).unwrap(), 4125600);
    }
}
