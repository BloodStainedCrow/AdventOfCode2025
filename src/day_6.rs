use std::mem;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Position};

#[derive(Debug, Clone, Copy)]
enum Kind {
    Add,
    Mul,
}

#[derive(Debug)]
struct Problem {
    data: Vec<u64>,
    kind: Kind,
}

type Input = Vec<Problem>;

#[aoc_generator(day6, part1)]
fn parse_part1(input: &str) -> Input {
    let columns: usize = input.lines().next().unwrap().split_whitespace().count();
    let mut ret = vec![(vec![], None); columns];

    for (column, v) in input
        .lines()
        .flat_map(|line| line.split_whitespace().enumerate())
    {
        if let Ok(v) = v.parse() {
            ret[column].0.push(v);
        } else {
            match v {
                "+" => ret[column].1 = Some(Kind::Add),
                "*" => ret[column].1 = Some(Kind::Mul),
                _ => unreachable!("{}", v),
            }
        }
    }

    ret.into_iter()
        .map(|(data, kind)| Problem {
            data,
            kind: kind.unwrap(),
        })
        .collect()
}

#[aoc_generator(day6, part2)]
fn parse_part2(input: &str) -> Input {
    let strides = input
        .lines()
        .last()
        .unwrap()
        .split_at(1)
        .1
        .split_inclusive(['+', '*'])
        .map(|slice| slice.chars().count())
        .with_position()
        .map(|(pos, v)| if pos == Position::Last { v + 1 } else { v });
    let mut ret: Vec<(Vec<u64>, _)> = strides
        .clone()
        .map(|length| (vec![0; length - 1], None))
        .collect();

    let mut lines = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    for (column, v) in lines
        .iter_mut()
        .flat_map(|line| split_arbitrary_mut(line, strides.clone()).enumerate())
    {
        if v.contains(&'+') {
            ret[column].1 = Some(Kind::Add);
        } else if v.contains(&'*') {
            ret[column].1 = Some(Kind::Mul);
        } else {
            for (i, digit) in v.iter().enumerate() {
                if digit.is_whitespace() {
                    continue;
                }

                let digit = format!("{digit}").parse().unwrap();
                if let Some(old) = ret[column].0.get_mut(i) {
                    *old *= 10;
                    *old += digit;
                } else {
                    ret[column].0.push(digit);
                }
            }
        }
    }

    ret.into_iter()
        .map(|(data, kind)| Problem {
            data,
            kind: kind.unwrap(),
        })
        .collect()
}

pub fn split_arbitrary_mut<T>(
    slice: &mut [T],
    sizes: impl IntoIterator<Item = usize>,
) -> impl Iterator<Item = &mut [T]> {
    SplitArbitraryMut {
        slice,
        sizes: sizes.into_iter(),
    }
}

struct SplitArbitraryMut<'a, T, C: Iterator<Item = usize>> {
    slice: &'a mut [T],
    sizes: C,
}

impl<'a, T, C: Iterator<Item = usize>> Iterator for SplitArbitraryMut<'a, T, C> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.sizes.next();

        match next {
            Some(size) => {
                let slice = mem::take(&mut self.slice);
                let split = slice.split_at_mut(size);
                self.slice = split.1;
                Some(split.0)
            }
            None => None,
        }
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> String {
    let count = input
        .iter()
        .map(|prob| match prob.kind {
            Kind::Add => prob.data.iter().sum::<u64>(),
            Kind::Mul => prob.data.iter().product(),
        })
        .sum::<u64>();
    format!("{count}")
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> String {
    let count = input
        .iter()
        .map(|prob| match prob.kind {
            Kind::Add => prob.data.iter().sum::<u64>(),
            Kind::Mul => prob.data.iter().product(),
        })
        .sum::<u64>();
    format!("{count}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day6.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE)), "4277556");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE)), "3263827");
    }
}
