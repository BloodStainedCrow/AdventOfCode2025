use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<RangeInclusive<u64>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .split(',')
        .map(|range| {
            let (min, max) = range.split('-').collect_tuple().unwrap();
            min.parse().unwrap()..=max.parse().unwrap()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> String {
    let sum: u64 = input
        .iter()
        .flat_map(std::clone::Clone::clone)
        .filter(|val| {
            let id_str = format!("{val}");
            let (left_half, right_half) = id_str.split_at(id_str.len() / 2);

            left_half == right_half
        })
        .sum();

    format!("{sum}")
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> String {
    let sum: u64 = input
        .iter()
        .flat_map(std::clone::Clone::clone)
        .filter(|val| {
            let id_str = format!("{val}");
            let id_str = id_str.as_bytes();
            for num_parts in 2..=id_str.len() {
                if id_str.len() % num_parts != 0 {
                    continue;
                }
                let substr_len = id_str.len() / num_parts;
                let (substr, rest) = id_str.split_at(substr_len);
                if rest.chunks(substr_len).all(|rest| rest == substr) {
                    // Value is invalid
                    return true;
                }
            }

            false
        })
        .sum();

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "1227775554");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "4174379265");
    }
}
