//! Day 2: Dive!

use crate::{enums::Part, util::output::Unsolved};
use std::fs;

pub fn solve(part: Part) -> Result<u64, Unsolved> {
    let instructions = read_instructions("inputs/year_2021/day_02.txt");

    match part {
        Part::Part1 => Ok(get_product_p1(&instructions)),
        Part::Part2 => Ok(get_product_p2(&instructions)),
    }
}

enum Instruction {
    Up(u64),
    Down(u64),
    Forward(u64),
}

fn read_instructions(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .expect("Cannot read input.")
        .lines()
        .map(|raw_instruction| {
            let raw_instruction = raw_instruction.trim().split(" ").collect::<Vec<_>>();
            let distance = raw_instruction[1].parse::<u64>().unwrap();

            match raw_instruction[0] {
                "up" => Instruction::Up(distance),
                "down" => Instruction::Down(distance),
                "forward" => Instruction::Forward(distance),
                direction => panic!("Unknown direction: {}", direction),
            }
        })
        .collect()
}

struct State {
    forward: u64,
    depth: u64,
}

fn get_product_p1(instructions: &[Instruction]) -> u64 {
    let final_state = instructions.iter().fold(
        State {
            forward: 0,
            depth: 0,
        },
        |state, instruction| match instruction {
            Instruction::Up(distance) => State {
                forward: state.forward,
                depth: state.depth - distance,
            },
            Instruction::Down(distance) => State {
                forward: state.forward,
                depth: state.depth + distance,
            },
            Instruction::Forward(distance) => State {
                forward: state.forward + distance,
                depth: state.depth,
            },
        },
    );

    final_state.forward * final_state.depth
}

struct Aim(u64);

fn get_product_p2(instructions: &[Instruction]) -> u64 {
    let (final_state, _) = instructions.iter().fold(
        (
            State {
                forward: 0,
                depth: 0,
            },
            Aim(0),
        ),
        |(state, aim), instruction| match instruction {
            Instruction::Up(distance) => (state, Aim(aim.0 - distance)),
            Instruction::Down(distance) => (state, Aim(aim.0 + distance)),
            Instruction::Forward(distance) => (
                State {
                    forward: state.forward + distance,
                    depth: state.depth + aim.0 * distance,
                },
                aim,
            ),
        },
    );

    final_state.forward * final_state.depth
}

#[cfg(test)]
mod tests {
    use super::{get_product_p1, get_product_p2, solve};
    use crate::{enums::Part, year_2021::day_02::read_instructions};

    #[test]
    fn part1_example() {
        let input = read_instructions("inputs/year_2021/day_02.example_01.txt");
        assert_eq!(get_product_p1(&input), 150);
    }

    #[test]
    fn part2_example() {
        let input = read_instructions("inputs/year_2021/day_02.example_01.txt");
        assert_eq!(get_product_p2(&input), 900);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(solve(Part::Part1).unwrap(), 1670340);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(solve(Part::Part2).unwrap(), 1954293920);
    }
}
