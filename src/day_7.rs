use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<Vec<bool>>, [usize; 2]);

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    let mut start = None;

    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, v)| match v {
                        '^' => true,
                        '.' => false,
                        'S' => {
                            assert!(start.is_none());
                            start = Some([x, y]);
                            false
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start.unwrap(),
    )
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> String {
    let mut cache = HashMap::default();
    let count = rec_part1(&input.0, input.1, &mut cache);
    format!("{count}")
}

fn rec_part1(
    field: &[Vec<bool>],
    [x, mut y]: [usize; 2],
    dedup: &mut HashMap<[usize; 2], usize>,
) -> usize {
    while let Some(row) = field.get(y)
        && let Some(pos) = row.get(x)
    {
        if dedup.get(&[x, y]).is_some() {
            return 0;
        }
        if *pos {
            // Split
            let self_value =
                rec_part1(field, [x - 1, y], dedup) + rec_part1(field, [x + 1, y], dedup) + 1;
            dedup.insert([x, y], self_value);
            return self_value;
        }
        y += 1;
    }

    0
}

fn rec_part2(
    field: &[Vec<bool>],
    [x, mut y]: [usize; 2],
    dedup: &mut HashMap<[usize; 2], usize>,
) -> usize {
    let start_y = y;
    if let Some(cached) = dedup.get(&[x, y]) {
        return *cached;
    }

    while let Some(row) = field.get(y)
        && let Some(pos) = row.get(x)
    {
        if *pos {
            // Split
            let self_value =
                rec_part2(field, [x - 1, y], dedup) + rec_part2(field, [x + 1, y], dedup);
            dedup.insert([x, start_y], self_value);
            return self_value;
        }
        y += 1;
    }

    1
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> String {
    let mut cache = HashMap::default();
    let count = rec_part2(&input.0, input.1, &mut cache);
    format!("{count}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day7.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "21");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "40");
    }
}
