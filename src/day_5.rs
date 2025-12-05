use core::slice::GetDisjointMutIndex;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split('-').collect_tuple().unwrap();

            let start = left.parse().unwrap();
            let end = right.parse().unwrap();

            start..=end
        })
        .collect();

    let ids = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> String {
    let count = input
        .1
        .iter()
        .copied()
        .filter(|id| input.0.iter().any(|range| range.contains(id)))
        .count();
    format!("{count}")
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> String {
    // The silly solution would be to just count, but that of course does not scale
    let mut input = input.0.clone();
    de_overlap(&mut input);

    let count: usize = input.iter().map(len).sum();

    format!("{count}")
}

fn len(range: &RangeInclusive<usize>) -> usize {
    range
        .end()
        .checked_sub(*range.start())
        .unwrap_or_else(|| panic!("{range:?}"))
        + 1
}

fn de_overlap(input: &mut Vec<RangeInclusive<usize>>) {
    let mut idx = 0;

    // I am not super happy using loops and while loops here
    loop {
        let mut prev_idx = 0;
        while prev_idx < idx {
            let current = &input[idx];
            assert!(
                current.start() <= current.end(),
                "{} <= {}",
                current.start(),
                current.end()
            );
            let prev = &input[prev_idx];

            if current.is_overlapping(prev) {
                if prev.start() <= current.start() {
                    // Shorten current
                    let new_start = *max(prev.end(), current.start()) + 1;

                    let new = new_start..=*current.end();

                    assert!(!new.is_overlapping(prev));

                    if *new.start() > *new.end() {
                        input.remove(idx);
                        idx -= 1;
                    } else {
                        assert!(new.start() <= new.end(), "{} <= {}", new.start(), new.end());
                        input[idx] = new;
                    }
                } else if prev.end() >= current.end() {
                    // Shorten current
                    let new_end = *min(prev.start(), current.end()) - 1;

                    let new = *current.start()..=new_end;

                    assert!(!new.is_overlapping(prev));

                    if *new.start() > *new.end() {
                        input.remove(idx);
                        idx -= 1;
                    } else {
                        assert!(new.start() <= new.end(), "{} <= {}", new.start(), new.end());
                        input[idx] = new;
                    }
                } else {
                    // Split current
                    let lower_new = if current.start() == prev.start() {
                        None
                    } else {
                        Some(*current.start()..=(*prev.start() - 1))
                    };
                    let upper_new = if prev.end() == current.end() {
                        None
                    } else {
                        Some((*prev.end() + 1)..=*current.end())
                    };

                    if let Some(upper_new) = &upper_new {
                        assert!(!upper_new.is_overlapping(prev));
                        assert!(
                            upper_new.start() <= upper_new.end(),
                            "{} <= {}",
                            upper_new.start(),
                            upper_new.end()
                        );
                    }

                    if let Some(lower_new) = &lower_new {
                        assert!(!lower_new.is_overlapping(prev));

                        assert!(
                            lower_new.start() <= lower_new.end(),
                            "{} <= {}",
                            lower_new.start(),
                            lower_new.end()
                        );
                    }

                    if let Some(lower_new) = &lower_new
                        && let Some(upper_new) = &upper_new
                    {
                        assert!(!lower_new.is_overlapping(upper_new));
                    }

                    let res = [current.clone()];
                    itertools::assert_equal(
                        input.splice(idx..=idx, [lower_new, upper_new].into_iter().flatten()),
                        res,
                    );
                }
            }

            prev_idx += 1;
        }

        idx += 1;
        if idx == input.len() {
            break;
        }
    }

    assert_eq!(
        input
            .iter()
            .enumerate()
            .cartesian_product(input.iter().enumerate())
            .find(|((a_idx, a), (b_idx, b))| {
                if a_idx == b_idx {
                    false
                } else {
                    a.is_overlapping(b)
                }
            }),
        None
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day5.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "14");
    }
}
