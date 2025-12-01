use aoc_runner_derive::{aoc, aoc_generator};

enum Dir {
    Left = 0,
    Right = 1,
}

struct Rotation {
    dir: Dir,
    num_clicks: u32,
}

type Input = Vec<Rotation>;

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let dir = match line.chars().next().unwrap() {
                'R' => Dir::Right,
                'L' => Dir::Left,
                _ => unreachable!(),
            };
            let count = line.split_at(1).1.parse().unwrap();

            Rotation {
                dir,
                num_clicks: count,
            }
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> String {
    const NUM_POS: u32 = 100;

    let mut current_pos: u32 = 50;

    let mut times_zero = 0;
    for rot in input {
        let num_clicks = rot.num_clicks % NUM_POS;
        match rot.dir {
            Dir::Left => {
                current_pos = current_pos
                    .checked_sub(num_clicks)
                    .unwrap_or_else(|| NUM_POS - (num_clicks - current_pos));
            }
            Dir::Right => {
                current_pos += num_clicks;
                current_pos %= NUM_POS;
            }
        }

        if current_pos == 0 {
            times_zero += 1;
        }
    }

    format!("{times_zero}")
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> String {
    const NUM_POS: u32 = 100;

    let mut current_pos: u32 = 50;

    let mut times_zero = 0;
    for rot in input {
        times_zero += rot.num_clicks / NUM_POS;
        let num_clicks = rot.num_clicks % NUM_POS;
        match rot.dir {
            Dir::Left => {
                current_pos = current_pos.checked_sub(num_clicks).map_or_else(
                    || {
                        if current_pos != 0 {
                            times_zero += 1;
                        }
                        NUM_POS - (num_clicks - current_pos)
                    },
                    |v| v,
                );
                if current_pos == 0 {
                    times_zero += 1;
                }
            }
            Dir::Right => {
                current_pos += num_clicks;
                if current_pos >= NUM_POS {
                    times_zero += 1;
                }
                current_pos %= NUM_POS;
            }
        }
    }

    format!("{times_zero}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day1.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "6");
    }
}
