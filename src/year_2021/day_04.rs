//! Day 4: Giant Squid

use crate::{enums::Part, util::output::Unsolved};
use std::{collections::HashSet, fs, panic};

#[derive(Debug)]
struct Board(Vec<Vec<u8>>);

pub fn solve(part: Part) -> Result<u64, Unsolved> {
    let (draws, boards) = read_input("inputs/year_2021/day_04.txt");

    match part {
        Part::Part1 => Ok(get_winning_score(&draws, &boards)),
        Part::Part2 => Ok(get_last_winning_score(&draws, &boards)),
    }
}

fn read_input(filename: &str) -> (Vec<u8>, Vec<Board>) {
    let mut boards = vec![];

    let input = fs::read_to_string(filename).expect("Failed to read input");

    let mut lines_iter = input.lines();

    let draws = lines_iter
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    while lines_iter.by_ref().next().is_some() {
        boards.push(Board(
            lines_iter
                .by_ref()
                .take(5)
                .map(|row| {
                    row.trim()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap_or_else(|_| panic!("Can't parse {}", s)))
                        .collect()
                })
                .collect(),
        ));
    }

    (draws, boards)
}

fn get_winning_score(draws: &[u8], boards: &[Board]) -> u64 {
    let mut drawn_numbers: HashSet<u8> = draws[..4].iter().copied().collect();

    for draw in draws[4..].iter() {
        drawn_numbers.insert(*draw);

        for board in boards.iter() {
            if is_winning(board, &drawn_numbers) {
                return get_board_score(board, &drawn_numbers, *draw);
            }
        }
    }

    panic!("No winning board found");
}

fn get_last_winning_score(draws: &[u8], boards: &[Board]) -> u64 {
    let mut remaining_boards = (0..boards.len()).collect::<Vec<usize>>();
    let mut drawn_numbers: HashSet<u8> = draws[..4].iter().copied().collect();

    for draw in draws[4..].iter() {
        drawn_numbers.insert(*draw);

        let mut winning_boards = vec![];
        for board_index in remaining_boards.iter() {
            let board = &boards[*board_index];
            if is_winning(board, &drawn_numbers) {
                winning_boards.push(*board_index);
            }
        }

        remaining_boards.retain(|&i| !winning_boards.contains(&i));
        if remaining_boards.is_empty() {
            return get_board_score(&boards[winning_boards[0]], &drawn_numbers, *draw);
        }
    }

    panic!("All boards didn't win");
}

fn is_winning(board: &Board, draws: &HashSet<u8>) -> bool {
    for i in 0..5 {
        if board.0[i].iter().all(|&n| draws.contains(&n)) {
            return true;
        }

        if (0..5).map(|j| board.0[j][i]).all(|n| draws.contains(&n)) {
            return true;
        }
    }

    false
}

fn get_board_score(board: &Board, draws: &HashSet<u8>, last_draw: u8) -> u64 {
    let mut score = 0;

    for row in board.0.iter() {
        for &n in row {
            if !draws.contains(&n) {
                score += n as u64;
            }
        }
    }

    score * last_draw as u64
}

#[cfg(test)]
mod tests {
    use super::{get_winning_score, read_input, solve};
    use crate::{enums::Part, year_2021::day_04::get_last_winning_score};

    #[test]
    fn part1_example() {
        let (draws, boards) = read_input("inputs/year_2021/day_04.example_01.txt");
        assert_eq!(get_winning_score(&draws, &boards), 4512);
    }

    #[test]
    fn part2_example() {
        let (draws, boards) = read_input("inputs/year_2021/day_04.example_01.txt");
        assert_eq!(get_last_winning_score(&draws, &boards), 1924);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(solve(Part::Part1).unwrap(), 29440);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(solve(Part::Part2).unwrap(), 13884);
    }
}
