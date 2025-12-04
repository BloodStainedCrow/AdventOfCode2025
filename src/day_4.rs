use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Vec<bool>>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|pos| pos == '@').collect())
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> String {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input.len() {
            if input[y][x] {
                // The is a roll here

                let surround_count = (-1..=1)
                    .cartesian_product(-1..=1)
                    .filter_map(|offs| {
                        if offs == (0, 0) {
                            None
                        } else {
                            let x = x.checked_add_signed(offs.0)?;
                            let y = y.checked_add_signed(offs.1)?;

                            let roll = input.get(y)?.get(x)?;

                            roll.then_some(())
                        }
                    })
                    .count();

                if surround_count < 4 {
                    count += 1;
                }
            }
        }
    }

    format!("{count}")
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> String {
    let mut input = input.clone();
    let mut count = 0;

    loop {
        let mut changed = false;

        for y in 0..input.len() {
            for x in 0..input.len() {
                if input[y][x] {
                    // The is a roll here

                    let surround_count = (-1..=1)
                        .cartesian_product(-1..=1)
                        .filter_map(|offs| {
                            if offs == (0, 0) {
                                None
                            } else {
                                let x = x.checked_add_signed(offs.0)?;
                                let y = y.checked_add_signed(offs.1)?;

                                let roll = input.get(y)?.get(x)?;

                                roll.then_some(())
                            }
                        })
                        .count();

                    if surround_count < 4 {
                        count += 1;
                        input[y][x] = false;
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    format!("{count}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day4.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "13");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "43");
    }
}
