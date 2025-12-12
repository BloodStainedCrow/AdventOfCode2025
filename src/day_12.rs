use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Shape {
    shape: [[bool; 3]; 3],
}

impl Shape {
    const fn mirror_vertical(mut self) -> Self {
        self.shape.reverse();
        self
    }

    const fn rotate_cw(self) -> Self {
        Self {
            shape: [
                [self.shape[2][0], self.shape[1][0], self.shape[0][0]],
                [self.shape[2][1], self.shape[1][1], self.shape[0][1]],
                [self.shape[2][2], self.shape[1][2], self.shape[0][2]],
            ],
        }
    }
}

#[derive(Debug)]
struct Tree {
    width: usize,
    height: usize,
    num_presents: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    present_shapes: Vec<Shape>,

    trees: Vec<Tree>,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    #[allow(clippy::unused_peekable)]
    let mut input = input.trim().split("\n\n").peekable();

    let presents = input
        .peeking_take_while(|str| str.contains('#'))
        .map(|str| {
            let Some((_index, size)) = str.split(':').collect_tuple() else {
                unreachable!();
            };

            let size = size.trim();

            Shape {
                shape: size
                    .lines()
                    .map(|line| line.chars().map(|c| c == '#').collect_array().unwrap())
                    .collect_array()
                    .unwrap(),
            }
        })
        .collect();

    let trees = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (size, list) = line.split(':').collect_tuple().unwrap();

            let (width, height) = size.split('x').collect_tuple().unwrap();

            Tree {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                num_presents: list
                    .split_whitespace()
                    .map(|count| count.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    Input {
        present_shapes: presents,
        trees,
    }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> String {
    let count = input
        .trees
        .iter()
        .filter(|tree| check_if_tree_works(tree, &input.present_shapes))
        .count();

    format!("{count}")
}

fn check_if_tree_works(tree: &Tree, presents: &[Shape]) -> bool {
    let size = tree.width * tree.height;

    let size_needed: usize = tree
        .num_presents
        .iter()
        .zip(presents.iter())
        .map(|(count, shape)| count * (shape.shape.iter().flatten().filter(|spot| **spot).count()))
        .sum();

    if size_needed > size {
        return false;
    }

    let mut space = vec![vec![false; tree.width]; tree.height];

    let rotated_and_flipped_shapes = presents
        .iter()
        .map(|shape| {
            iter::repeat(*shape)
                .zip((0..=3).cartesian_product(0..=1))
                .map(|(mut shape, (rotations, flip))| {
                    for _ in 0..rotations {
                        shape = shape.rotate_cw();
                    }

                    if flip == 1 {
                        // Mirroring vertically is enough, since (for rectangles)
                        // horizontal mirroring can be expressed as a mix of rotation and vertical mirroring
                        shape = shape.mirror_vertical();
                    }

                    shape
                })
                .unique()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rotated_and_flipped_shapes.iter().position(|list| {
            // Try to find a shape without a fixed spot
            !(0..3)
                .cartesian_product(0..3)
                // A spot is fixed if there exists an (x, y) where all rotations/mirrors have a spot there
                .any(|(x, y)| list.iter().all(|shape| shape.shape[y][x]))
        }),
        None,
        "Shape does not have a fixed spot (regarding rotation/mirroring)"
    );

    rec_naive(
        &mut space,
        0,
        &mut tree.num_presents.clone(),
        &rotated_and_flipped_shapes,
    )
}

// I expected this to be unable to finish in time for the main input.
// This turned out to be fine. I am very surprised.
fn rec_naive(
    current: &mut [Vec<bool>],
    my_shape_idx: usize,
    to_place: &mut [usize],
    presents: &[Vec<Shape>],
) -> bool {
    let Some(rotated_and_flipped_shapes) = presents.get(my_shape_idx) else {
        #[cfg(debug_assertions)]
        draw_board(current);
        return true;
    };

    if to_place[my_shape_idx] > 0 {
        to_place[my_shape_idx] -= 1;

        'rows: for y in 0..current.len() {
            'pos: for x in 0..current[y].len() {
                // Doing the shape loop inside here is an optimization, that only works if
                // There are any spaces, which are always filled in any shape, since
                // if we find any overlap we skip to the next position. That would be invalid otherwise.
                // I cannot quantify how much faster it is (likely many orders of magnitude), since it does not complete the calculation otherwise
                // This is asserted in check_if_tree_works
                for &shape in rotated_and_flipped_shapes {
                    for (y_offs, row) in shape.shape.iter().enumerate() {
                        for (x_offs, val) in row.iter().enumerate() {
                            if *val {
                                let x = x + x_offs;
                                let y = y + y_offs;

                                if let Some(row) = current.get(y) {
                                    if let Some(current_val) = row.get(x) {
                                        if *current_val {
                                            // This shape is blocked here
                                            continue 'pos;
                                        }

                                        // This shape fits
                                    } else {
                                        continue 'rows;
                                    }
                                } else {
                                    to_place[my_shape_idx] += 1;
                                    return false;
                                }
                            }
                        }
                    }

                    // The shape fully fits

                    // Put in the shape
                    for (y_offs, row) in shape.shape.iter().enumerate() {
                        for (x_offs, val) in row.iter().enumerate() {
                            if *val {
                                let x = x + x_offs;
                                let y = y + y_offs;

                                debug_assert!(!current[y][x]);
                                current[y][x] = true;
                            }
                        }
                    }

                    if rec_naive(current, my_shape_idx, to_place, presents) {
                        // We found a solution!!!

                        return true;
                    }

                    // Remove the shape again
                    for (y_offs, row) in shape.shape.iter().enumerate() {
                        for (x_offs, val) in row.iter().enumerate() {
                            if *val {
                                let x = x + x_offs;
                                let y = y + y_offs;

                                debug_assert!(current[y][x]);
                                current[y][x] = false;
                            }
                        }
                    }
                }
            }
        }

        to_place[my_shape_idx] += 1;
        false
    } else {
        rec_naive(current, my_shape_idx + 1, to_place, presents)
    }
}

#[cfg(debug_assertions)]
fn draw_board(current: &[Vec<bool>]) {
    for row in current {
        for spot in row {
            if *spot {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day12.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "2");
    }
}
