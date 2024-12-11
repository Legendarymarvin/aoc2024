use std::collections::{HashMap, HashSet, VecDeque};
const UP: Point = Point { y: -1, x: 0 };
const LEFT: Point = Point { y: 0, x: -1 };
const RIGHT: Point = Point { y: 0, x: 1 };
const DOWN: Point = Point { y: 1, x: 0 };
const DIRECTIONS: [Point; 4] = [UP, LEFT, RIGHT, DOWN];

pub fn solve_day_10(input: &str) -> (usize, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    y: i32,
    x: i32,
}

fn solve(test_input: &str) -> (usize, usize) {
    let mut peaks = HashSet::new();
    let mut starts = Vec::new();
    let mut matrix = HashMap::new();
    // only works on squares, otherwise create two limits.
    let mut limit = 0;

    test_input.lines().enumerate().for_each(|(y, line)| {
        limit = line.len();
        line.chars().enumerate().for_each(|(x, ch)| {
            let point = Point {
                y: y as i32,
                x: x as i32,
            };
            let value = ch.to_digit(10).unwrap();
            matrix.insert(point, value);
            // only needed for part 2;
            if value == 9 {
                peaks.insert(point);
            }
            if value == 0 {
                starts.push(point);
            }
        })
    });

    let p1 = part1(&starts, &matrix, limit);
    let p2 = part2(&peaks, &starts, &matrix);

    (p1, p2)
}

fn part2(peaks: &HashSet<Point>, zeros: &Vec<Point>, matrix: &HashMap<Point, u32>) -> usize {
    let mut counts: HashMap<Point, usize> = HashMap::new();
    zeros.iter().for_each(|&zero| {
        counts.insert(zero, 1);
    });

    let mut positions_by_value: HashMap<u32, Vec<Point>> = HashMap::new();
    for (&point, &value) in matrix {
        positions_by_value
            .entry(value)
            .or_insert_with(Vec::new)
            .push(point);
    }

    for value in 1..=9 {
        for &point in positions_by_value.get(&value).unwrap_or(&Vec::new()) {
            let mut total_paths = 0;
            for &dir in DIRECTIONS.iter() {
                let step = make_step(point, dir);
                if let Some(&prev_value) = matrix.get(&step) {
                    if prev_value == value - 1 {
                        total_paths += counts.get(&step).unwrap_or(&0);
                    }
                }
            }
            counts.insert(point, total_paths);
        }
    }

    peaks.iter()
        .map(|&point| counts.get(&point).unwrap_or(&0))
        .sum()
}

fn part1(zeros: &Vec<Point>, matrix: &HashMap<Point, u32>, limit: usize) -> usize {
    zeros.iter().map(|&zero| {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut peaks_found = HashSet::new();

        queue.push_back(zero);
        visited.insert(zero);

        while let Some(p) = queue.pop_front() {
            let current_value = *matrix.get(&p).unwrap();

            for dir in [UP, LEFT, RIGHT, DOWN].iter() {
                let step = make_step(p, *dir);
                if is_valid(step, limit as i32) {
                    let next_value = *matrix.get(&step).unwrap();
                    if next_value == current_value + 1 {
                        if next_value == 9 {
                            peaks_found.insert(step);
                        } else if !visited.contains(&step) {
                            queue.push_back(step.clone());
                            visited.insert(step.clone());
                        }
                    }
                }
            }
        }
        peaks_found.len()
    }).sum()
}

fn is_valid(p0: Point, limit: i32) -> bool {
    p0.x >= 0 && p0.y >= 0 && p0.x < limit && p0.y < limit
}

fn make_step(p0: Point, dir: Point) -> Point {
    Point {
        x: p0.x + dir.x,
        y: p0.y + dir.y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

        let (p1, p2) = solve_day_10(test_input);

        // Off by one for test, not for real, not sure why
        assert_eq!(p1, 36, "Part 1 failed");
        assert_eq!(p2, 81, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input10.txt");

        let (p1, p2) = solve_day_10(test_input);

        assert_eq!(p1, 538, "Part 1 failed");
        assert_eq!(p2, 1110, "Part 2 failed");
    }
}
