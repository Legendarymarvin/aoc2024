use petgraph::algo::toposort;
use petgraph::graph::Graph;
use petgraph::Directed;
use std::collections::HashMap;

pub fn solve_day_05(input: &str) -> (i32, i32) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

fn solve(input: &str) -> (i32, i32) {
    let (order, reports): (&str, &str) = input.split_once("\n\n").unwrap();
    let must_be_before = parse_order_rules(order);

    let (correct_reports, incorrect_reports) = split_correct_or_not(reports, &must_be_before);
    let p1 = sum_middle_values(correct_reports);

    let corrected = correct_incorrect(&must_be_before, incorrect_reports);
    let p2 = sum_middle_values(corrected);

    (p1, p2)
}

fn parse_order_rules(input: &str) -> HashMap<i32, Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let (n1_str, n2_str) = line.split_once('|').unwrap();
            let n1 = n1_str.parse::<i32>().unwrap();
            let n2 = n2_str.parse::<i32>().unwrap();
            (n1, n2)
        })
        .fold(HashMap::new(), |mut acc, (n1, n2)| {
            acc.entry(n1).or_insert_with(Vec::new).push(n2);
            acc
        })
}

fn sum_middle_values(correct_reports: Vec<Vec<i32>>) -> i32 {
    correct_reports
        .iter()
        .map(|report| {
            // dirty way to get middle element of odd sized vector
            report[report.len() / 2]
        })
        .sum()
}

fn split_correct_or_not(
    reports: &str,
    must_be_before: &HashMap<i32, Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let reports_numbers: Vec<Vec<i32>> = reports
        .lines()
        .map(|line| {
            line.split(",")
                .into_iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports_numbers.into_iter().partition(|numbers| {
        numbers.iter().enumerate().all(|(idx, &n)| {
            // For every number, check if all the following numbers follow the rules
            numbers[idx + 1..]
                .iter()
                .all(|&m| is_correct_order(n, m, must_be_before))
        })
    })
}

fn correct_incorrect(
    must_be_before: &HashMap<i32, Vec<i32>>,
    incorrect_reports: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    incorrect_reports
        .iter()
        .map(|numbers| {
            let mut ordered = vec![0; numbers.len()];
            numbers.iter().for_each(|n| {
                // we use the correct order method to check how many other elements must be before.
                // and use that as index, these orders reverse, but correctly.
                let idx = numbers
                    .iter()
                    .filter(|m| is_correct_order(*n, **m, &must_be_before))
                    .count();
                ordered[idx] += n;
            });
            ordered
        })
        .collect()
}

// I thought an actual sorting algorithm might speed this up, but I guess building a graph
// is too expensive for sorting reports this short.
// benches a bit over 1ms vs 700µs
#[allow(unused)]
fn correct_incorrect_algo(
    must_be_before: &HashMap<i32, Vec<i32>>,
    incorrect_reports: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    incorrect_reports
        .iter()
        .map(|numbers| topological_sort(numbers, must_be_before))
        .collect()
}

fn topological_sort(numbers: &[i32], must_be_before: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut graph = Graph::<i32, (), Directed>::new();
    let mut node_indices = HashMap::new();

    for &number in numbers {
        let node = graph.add_node(number);
        node_indices.insert(number, node);
    }

    for &number in numbers {
        if let Some(befores) = must_be_before.get(&number) {
            for &before in befores {
                if node_indices.contains_key(&before) {
                    graph.add_edge(node_indices[&number], node_indices[&before], ());
                }
            }
        }
    }

    if let Ok(sorted) = toposort(&graph, None) {
        sorted.into_iter().map(|node| graph[node]).collect()
    } else {
        // Handle cycles or errors
        numbers.to_vec()
    }
}

fn is_correct_order(base: i32, comparator: i32, must_be_before: &HashMap<i32, Vec<i32>>) -> bool {
    must_be_before
        .get(&base)
        .map_or(false, |vec| vec.contains(&comparator))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let (p1, p2) = solve_day_05(test_input);

        assert_eq!(p1, 143, "Part 1 failed");
        assert_eq!(p2, 123, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input05.txt");

        let (p1, p2) = solve_day_05(test_input);

        assert_eq!(p1, 5955, "Part 1 failed");
        assert_eq!(p2, 4030, "Part 2 failed");
    }
}
