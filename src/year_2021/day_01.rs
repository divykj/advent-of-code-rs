//! Day 1: Sonar Sweep

use crate::{
    enums::Part,
    util::{input::read_numbers, output::Unsolved},
};

pub fn solve(part: Part) -> Result<usize, Unsolved> {
    let readings = read_numbers("inputs/year_2021/day_01.txt");

    match part {
        Part::Part1 => Ok(times_measurement_increased(readings, 1)),
        Part::Part2 => Ok(times_measurement_increased(readings, 3)),
    }
}

fn times_measurement_increased(readings: Vec<u64>, window_size: usize) -> usize {
    readings
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect::<Vec<u64>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::{solve, times_measurement_increased};
    use crate::{enums::Part, util::input::read_numbers};

    #[test]
    fn part1_example() {
        let input = read_numbers("inputs/year_2021/day_01.example_01.txt");
        assert_eq!(times_measurement_increased(input, 1), 7);
    }

    #[test]
    fn part2_example() {
        let input = read_numbers("inputs/year_2021/day_01.example_02.txt");
        assert_eq!(times_measurement_increased(input, 3), 5);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(solve(Part::Part1).unwrap(), 1482);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(solve(Part::Part2).unwrap(), 1518);
    }
}
