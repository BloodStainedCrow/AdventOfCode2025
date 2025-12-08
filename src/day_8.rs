use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::graph::UnGraph;

type Input = Vec<[i64; 3]>;

#[aoc_generator(day8)]
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

#[aoc(day8, part1)]
fn part1(input: &Input) -> String {
    const NUM_CONNS: usize = 10;
    let mut graph = UnGraph::new_undirected();
    for input in input {
        graph.add_node(*input);
    }

    let edges_by_length = graph
        .node_indices()
        .cartesian_product(graph.node_indices())
        .filter(|(a, b)| a < b)
        .sorted_by_key(|(start, end)| {
            let start = graph.node_weight(*start).unwrap();
            let end = graph.node_weight(*end).unwrap();

            (end[0] - start[0]) * (end[0] - start[0])
                + (end[1] - start[1]) * (end[1] - start[1])
                + (end[2] - start[2]) * (end[2] - start[2])
        })
        .collect_vec();

    for (start, end) in edges_by_length.into_iter().take(NUM_CONNS) {
        graph.add_edge(start, end, ());
    }

    let networks = petgraph::algo::tarjan_scc(&graph);

    let value: usize = networks
        .iter()
        .map(std::vec::Vec::len)
        .sorted()
        .rev()
        .take(3)
        .product();

    format!("{value}")
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> String {
    let mut graph = UnGraph::new_undirected();
    for input in input {
        graph.add_node(*input);
    }

    let mut last = None;

    let edges_by_length = graph
        .node_indices()
        .cartesian_product(graph.node_indices())
        .filter(|(a, b)| a < b)
        .filter(|(a, b)| !graph.contains_edge(*a, *b))
        .sorted_by_key(|(start, end)| {
            let start = graph.node_weight(*start).unwrap();
            let end = graph.node_weight(*end).unwrap();

            (end[0] - start[0]) * (end[0] - start[0])
                + (end[1] - start[1]) * (end[1] - start[1])
                + (end[2] - start[2]) * (end[2] - start[2])
        })
        .collect_vec();

    for (start, end) in edges_by_length {
        last = Some((start, end));
        graph.add_edge(start, end, ());

        if petgraph::algo::connected_components(&graph) == 1 {
            break;
        }
    }

    let last_start = graph.node_weight(last.unwrap().0).unwrap();
    let last_end = graph.node_weight(last.unwrap().1).unwrap();
    let value = last_start[0] * last_end[0];

    format!("{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example/day8.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "40");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "25272");
    }
}
