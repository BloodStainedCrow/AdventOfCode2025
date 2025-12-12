use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = HashMap<String, Vec<String>>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (source, dests) = line.split(':').collect_tuple().unwrap();

            let destinations = dests
                .split_whitespace()
                .map(std::borrow::ToOwned::to_owned)
                .collect_vec();

            (source.to_string(), destinations)
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> String {
    let mut dedup = HashMap::default();

    let num_paths = rec_part1(input, "you", "out", &mut dedup);

    format!("{num_paths}")
}

// This only works for cycle free graphs
fn rec_part1<'a>(
    input: &'a Input,
    current: &'a str,
    goal: &str,
    dedup: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(v) = dedup.get(current) {
        return *v;
    }

    if current == goal {
        return 1;
    }

    let paths = input[current]
        .iter()
        .map(|next| rec_part1(input, next, goal, dedup))
        .sum();

    dedup.insert(current, paths);

    paths
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> String {
    let mut dedup = HashMap::default();

    let num_paths = rec_part2(input, "svr", "out", &vec!["dac", "fft"], &mut dedup);

    format!("{num_paths}")
}

// This only works for cycle free graphs
fn rec_part2<'a>(
    input: &'a Input,
    current: &'a str,
    goal: &str,
    needed_subparts: &Vec<&'a str>,
    dedup: &mut HashMap<&'a str, HashMap<Vec<&'a str>, usize>>,
) -> usize {
    if let Some(v) = dedup.get(current)
        && let Some(v) = v.get(needed_subparts)
    {
        return *v;
    }

    if current == goal {
        if needed_subparts.is_empty() {
            return 1;
        }
        return 0;
    }

    // We always need to clone here, in order to allow putting this into the hashmap later
    let mut needed_subparts = needed_subparts.clone();
    if needed_subparts.contains(&current) {
        needed_subparts.retain(|v| *v != current);
    }

    let paths = input[current]
        .iter()
        .map(|next| rec_part2(input, next, goal, &needed_subparts, dedup))
        .sum();

    dedup
        .entry(current)
        .or_default()
        .insert(needed_subparts, paths);

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = include_str!("../example/day11_part1.txt");
    const EXAMPLE_PART2: &str = include_str!("../example/day11_part2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART1)), "5");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_PART2)), "2");
    }
}
