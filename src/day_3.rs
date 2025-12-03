use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u8>>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|char| char as u8 - 48).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> String {
    let sum: u32 = input
        .iter()
        .map(|bank| {
            assert!(bank.len() >= 2);

            let tens = bank.iter().take(bank.len() - 1).max().unwrap();
            let tens_pos = bank.iter().position(|v| v == tens).unwrap();
            let ones = bank.iter().skip(tens_pos + 1).max().unwrap();

            u32::from((*tens) * 10 + *ones)
        })
        .sum();

    format!("{sum}")
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> String {
    const NUM_BATTERIES: usize = 12;
    let sum: u64 = input
        .iter()
        .map(|bank| {
            assert!(bank.len() >= NUM_BATTERIES);

            let mut used_range: usize = 0;

            (0..NUM_BATTERIES)
                .rev()
                .map(|batt_idx| {
                    let v = bank
                        .iter()
                        .skip(used_range)
                        .take(bank.len() - batt_idx - used_range)
                        .max()
                        .unwrap();
                    let pos = bank
                        .iter()
                        .skip(used_range)
                        .position(|found| found == v)
                        .unwrap();
                    used_range += pos + 1;

                    u64::from(*v) * 10_u64.pow(batt_idx.try_into().unwrap())
                })
                .sum::<u64>()
        })
        .sum();

    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day3.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "357");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "3121910778619");
    }
}
