use std::{
    cmp::{self},
    collections::HashMap,
};

use aoc_runner_derive::{aoc, aoc_generator};
use good_lp::{Expression, ProblemVariables, SolverModel, constraint, default_solver, variable};
use itertools::Itertools;

#[derive(Debug)]
struct Line {
    goal: Vec<bool>,

    /// Last is curly
    buttons: Vec<Vec<usize>>,
}

type Input = Vec<Line>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut sections = line.split_whitespace();

            let goal = sections.next().unwrap();
            let goal = goal.strip_prefix('[').unwrap();
            let goal = goal.strip_suffix(']').unwrap();

            let goal = goal.chars().map(|c| c == '#').collect();

            let buttons = sections
                .map(|button| {
                    button
                        .strip_prefix(['(', '{'])
                        .unwrap()
                        .strip_suffix([')', '}'])
                        .unwrap()
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();

            Line { goal, buttons }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> String {
    let mut count = 0;
    let value: usize = input
        .iter()
        .map(|Line { goal, buttons }| {
            count += 1;
            let mut current = goal.clone();
            let mut used_buttons = vec![];
            let mut dedup = HashMap::default();

            rec_part1(
                &mut current,
                &mut used_buttons,
                &mut dedup,
                &buttons[0..(buttons.len() - 1)],
            )
        })
        .sum();

    format!("{value}")
}

// TODO: This would be so much better with bitsets
fn rec_part1(
    current: &mut [bool],
    used_buttons: &mut Vec<usize>,
    dedup: &mut HashMap<Vec<bool>, Vec<(Vec<usize>, usize)>>,
    buttons: &[Vec<usize>],
) -> usize {
    if current.iter().all(|v| !v) {
        return 0;
    }

    if let Some(v) = dedup.get(current)
        && let Some(v) = v.iter().find_map(|(k, v)| {
            // Check if we have ever seen this state while the same (or fewer) buttons were outlawed
            k.iter()
                .all(|used_button| used_buttons.contains(used_button))
                .then_some(v)
        })
    {
        return *v;
    }

    #[cfg(debug_assertions)]
    let old_lamps = current.to_vec();
    #[cfg(debug_assertions)]
    let old_used = used_buttons.clone();

    let mut min = usize::MAX / 2;
    for (id, button) in buttons.iter().enumerate() {
        if used_buttons.contains(&id) {
            continue;
        }
        used_buttons.push(id);
        // Press this button
        for idx in button {
            current[*idx] = !current[*idx];
        }

        // Continue the recursion
        let num_buttons = rec_part1(current, used_buttons, dedup, buttons);
        min = cmp::min(min, num_buttons);

        // Undo the press of this button
        for idx in button {
            current[*idx] = !current[*idx];
        }
        used_buttons.pop();
    }

    #[cfg(debug_assertions)]
    assert_eq!(old_lamps, current);
    #[cfg(debug_assertions)]
    assert_eq!(&old_used, used_buttons);

    dedup
        .entry(current.to_vec())
        .or_default()
        .push((used_buttons.clone(), min + 1));
    min + 1
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> String {
    let mut count = 0;
    let value: usize = input
        .iter()
        .map(|Line { goal: _, buttons }| {
            dbg!(count);
            count += 1;
            let (buttons, [goal]) = buttons.split_at(buttons.len() - 1) else {
                unreachable!()
            };

            let equations = goal
                .iter()
                .enumerate()
                .map(|(idx, goal)| Equation {
                    goal: *goal,
                    factors: buttons
                        .iter()
                        .positions(|button| button.contains(&idx))
                        .collect(),
                })
                .collect::<Vec<_>>();

            let num_buttons = buttons.len();

            part_2_lp(&equations, num_buttons)
        })
        .sum();

    format!("{value}")
}

fn part_2_lp(equations: &[Equation], num_factors: usize) -> usize {
    let mut vars = ProblemVariables::new();

    let factors = (0..num_factors)
        .map(|_| vars.add(variable().integer().bounds(0..)))
        .collect_vec();

    let problem = vars
        .optimise(
            good_lp::ObjectiveDirection::Minimisation,
            factors.iter().sum::<Expression>(),
        )
        .using(default_solver);

    let res = problem
        .with_all(equations.iter().map(|eq| {
            constraint!(
                eq.factors
                    .iter()
                    .map(|fac| factors[*fac])
                    .sum::<Expression>()
                    == u32::try_from(eq.goal).unwrap()
            )
        }))
        .solve();

    match res {
        #[allow(clippy::cast_possible_truncation)]
        Ok(sol) => (sol.into_inner().objective().round() as i64)
            .try_into()
            .unwrap(),
        Err(err) => panic!("Could not find solution: {err}"),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Equation {
    goal: usize,
    factors: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day10.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "7");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "33");
    }
}
