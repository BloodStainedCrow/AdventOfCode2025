use std::{
    cmp::{max, min},
    iter,
    ops::RangeInclusive,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<[usize; 2]>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> String {
    let max_size = input
        .iter()
        .copied()
        .cartesian_product(input.iter().copied())
        .map(|(a, b)| {
            let width = a[0].abs_diff(b[0]) + 1;
            let height = a[1].abs_diff(b[1]) + 1;

            width * height
        })
        .max()
        .unwrap();

    format!("{max_size}")
}

fn check_point_is_inside(
    [x, y]: [usize; 2],
    [width, height]: [usize; 2],
    horizontals: &[(RangeInclusive<usize>, usize)],
    verticals: &[(usize, RangeInclusive<usize>)],
) -> bool {
    debug_assert!(horizontals.is_sorted_by_key(|v| v.1));
    debug_assert!(verticals.is_sorted_by_key(|v| v.0));

    if verticals
        .iter()
        .take_while(|(edge_x, _edge_y)| *edge_x <= x)
        .any(|vertical| vertical.0 == x && vertical.1.contains(&y))
    {
        return true;
    }
    if horizontals
        .iter()
        .take_while(|(_edge_x, edge_y)| *edge_y <= y)
        .any(|vertical| vertical.1 == y && vertical.0.contains(&x))
    {
        return true;
    }

    let left_dist = x;
    let right_dist = width - x;
    let top_dist = y;
    let bottom_dist = height - y;
    let min = min(min(left_dist, right_dist), min(top_dist, bottom_dist));

    let count = if min == left_dist {
        verticals
            .iter()
            .take_while(|(edge_x, _edge_y)| *edge_x < x)
            .filter(|(_edge_x, edge_y)| ((*edge_y.start())..(*edge_y.end())).contains(&y))
            .count()
    } else if min == right_dist {
        verticals
            .iter()
            .rev()
            .take_while(|(edge_x, _edge_y)| *edge_x > x)
            .filter(|(_edge_x, edge_y)| ((*edge_y.start())..(*edge_y.end())).contains(&y))
            .count()
    } else if min == top_dist {
        horizontals
            .iter()
            .take_while(|(_edge_x, edge_y)| *edge_y < y)
            .filter(|(edge_x, _edge_y)| ((*edge_x.start())..(*edge_x.end())).contains(&x))
            .count()
    } else if min == bottom_dist {
        horizontals
            .iter()
            .rev()
            .take_while(|(_edge_x, edge_y)| *edge_y > y)
            .filter(|(edge_x, _edge_y)| ((*edge_x.start())..(*edge_x.end())).contains(&x))
            .count()
    } else {
        unreachable!()
    };

    !count.is_multiple_of(2)
}

#[derive(Debug, Clone)]
enum Line {
    Horizontal { x: RangeInclusive<usize>, y: usize },
    Vertical { x: usize, y: RangeInclusive<usize> },
}

fn check_line_is_inside(
    mut line: Line,
    horizontals: &[(RangeInclusive<usize>, usize)],
    verticals: &[(usize, RangeInclusive<usize>)],
) -> bool {
    debug_assert!(horizontals.is_sorted_by_key(|v| v.1));
    debug_assert!(verticals.is_sorted_by_key(|v| v.0));

    match &mut line {
        Line::Horizontal { x, y } => {
            let mut crossed_verticals = verticals.iter().filter(|(vertical_x, vertical_y)| {
                ((*x.start() + 1)..(*x.end())).contains(vertical_x)
                    && ((*vertical_y.start())..(*vertical_y.end())).contains(y)
            });
            // Check that the line does not cross any edges of the polygon
            crossed_verticals.next().is_none()
        }
        Line::Vertical { x, y } => {
            let mut crossed_horizontals =
                horizontals.iter().filter(|(horizontal_x, horizontal_y)| {
                    ((*y.start() + 1)..(*y.end())).contains(horizontal_y)
                        && ((*horizontal_x.start())..(*horizontal_x.end())).contains(x)
                });
            // Check that the line does not cross any edges of the polygon
            crossed_horizontals.next().is_none()
        }
    }
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> String {
    // Ensure all vertices are 90 degree corners
    assert!(
        input
            .iter()
            .zip(input.iter().skip(1))
            .zip(input.iter().skip(2))
            .all(|((prev, next), next_next)| {
                ((prev[0] == next[0]) ^ (next[0] == next_next[0]))
                    && ((prev[1] == next[1]) ^ (next[1] == next_next[1]))
            })
    );

    let width = input.iter().map(|v| v[0]).max().unwrap() + 1;
    let height = input.iter().map(|v| v[1]).max().unwrap() + 1;

    let horizontals = input
        .iter()
        .zip(input.iter().skip(1))
        .chain(iter::once((input.last().unwrap(), input.first().unwrap())))
        .filter_map(|(prev, next)| {
            let start = min(prev[0], next[0]);
            let end = max(prev[0], next[0]);
            (prev[1] == next[1]).then_some((start..=end, prev[1]))
        })
        .sorted_by_key(|v| v.1)
        .collect_vec();

    let verticals = input
        .iter()
        .zip(input.iter().skip(1))
        .chain(iter::once((input.last().unwrap(), input.first().unwrap())))
        .filter_map(|(prev, next)| {
            let start = min(prev[1], next[1]);
            let end = max(prev[1], next[1]);
            (prev[0] == next[0]).then_some((prev[0], start..=end))
        })
        .sorted_by_key(|v| v.0)
        .collect_vec();

    let max_size = input
        .iter()
        .copied()
        .cartesian_product(input.iter().copied())
        // Filter duplicates
        .filter(|(top_left, bottom_right)| top_left <= bottom_right)
        // Check large rectangles first
        .sorted_by_key(|(a, b)| {
            let width = a[0].abs_diff(b[0]) + 1;
            let height = a[1].abs_diff(b[1]) + 1;

            width * height
        })
        .rev()
        .find(|(a, b)| {
            let top_left = [min(a[0], b[0]), min(a[1], b[1])];
            let top_right = [max(a[0], b[0]), min(a[1], b[1])];
            let bottom_right = [max(a[0], b[0]), max(a[1], b[1])];
            let bottom_left = [min(a[0], b[0]), max(a[1], b[1])];

            check_point_is_inside(top_left, [width, height], &horizontals, &verticals)
                && check_point_is_inside(top_right, [width, height], &horizontals, &verticals)
                && check_point_is_inside(bottom_right, [width, height], &horizontals, &verticals)
                && check_point_is_inside(bottom_left, [width, height], &horizontals, &verticals)
                && check_line_is_inside(
                    Line::Horizontal {
                        x: top_left[0]..=top_right[0],
                        y: top_left[1],
                    },
                    &horizontals,
                    &verticals,
                )
                && check_line_is_inside(
                    Line::Horizontal {
                        x: bottom_left[0]..=bottom_right[0],
                        y: bottom_left[1],
                    },
                    &horizontals,
                    &verticals,
                )
                && check_line_is_inside(
                    Line::Vertical {
                        x: top_left[0],
                        y: top_left[1]..=bottom_left[1],
                    },
                    &horizontals,
                    &verticals,
                )
                && check_line_is_inside(
                    Line::Vertical {
                        x: top_right[0],
                        y: top_right[1]..=bottom_right[1],
                    },
                    &horizontals,
                    &verticals,
                )
        })
        .map(|(a, b)| {
            let width = a[0].abs_diff(b[0]) + 1;
            let height = a[1].abs_diff(b[1]) + 1;
            width * height
        })
        .unwrap();

    #[cfg(debug_assertions)]
    print_grid(width, height, &horizontals, &verticals);

    format!("{max_size}")
}

#[cfg(debug_assertions)]
fn print_grid(
    width: usize,
    height: usize,
    horizontals: &[(RangeInclusive<usize>, usize)],
    verticals: &[(usize, RangeInclusive<usize>)],
) {
    for y in 0..height {
        for x in 0..width {
            if check_point_is_inside([x, y], [width, height], horizontals, verticals) {
                print!("X");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day9.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "50");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "24");
    }
}
