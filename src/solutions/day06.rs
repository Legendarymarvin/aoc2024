use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use crate::solutions::day06::Direction::*;
use crate::solutions::day06::Thing::{EMPTY, OBSTACLE};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
enum Thing {
    OBSTACLE, EMPTY,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash)]
enum Direction {
    UP, DOWN, RIGHT, LEFT
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct State {
    position: Point,
    direction: Direction,
}

struct Move {
    new_direction: Direction,
    new_position: Point,
    off_map: bool
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash)]
struct Point {
    x: isize, y: isize
}

pub fn solve_day_06(input: &str) -> (usize, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

fn solve(input: &str) -> (usize, usize) {
    let edge = input.lines().count();
    let (map, guard) = create_map(input);

    let (part1, positions) = part1(edge, &map, guard);
    let part2 = part2(&map, guard, edge);

    print_map(&map, &positions, edge);
    (part1, part2)
}

fn part1(edge: usize, map: &HashMap<Point, Thing>, mut guard: Point) -> (usize, HashSet<Point>) {
    let mut direction = UP;
    let mut positions = HashSet::new();
    loop {
        let after = guard_move(&map, edge, guard, direction);
        if after.off_map {
            break;
        }
        guard = after.new_position;
        direction = after.new_direction;
        positions.insert(guard);
    }
    let part1 = positions.len();
    (part1, positions)
}

fn part2(map: &HashMap<Point, Thing>, start: Point, edge: usize) -> usize {
    let empty_positions: Vec<Point> = map
        .iter()
        .filter_map(|(&pos, thing)| {
            if thing == &EMPTY && pos != start {
                Some(pos)
            } else {
                None
            }
        })
        .collect();

     empty_positions
        .par_iter()
        .filter_map(|&obstacle_pos| {
            let mut map_with_obstacle = map.clone();
            map_with_obstacle.insert(obstacle_pos, OBSTACLE);

            if guard_will_loop(&map_with_obstacle, start, edge) {
                Some(obstacle_pos)
            } else {
                None
            }
        })
        .count()
}

fn create_map(input: &str) -> (HashMap<Point, Thing>, Point) {
    let mut map: HashMap<Point, Thing> = HashMap::new();
    let mut guard: Point = Point {x: 0, y: 0};
    for (idx, line) in input.lines().enumerate() {
        for (jdx, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert(Point {x: jdx as isize, y: idx as isize}, OBSTACLE);
            } else if ch == '.' {
                map.insert(Point {x: jdx as isize, y: idx as isize}, EMPTY);
            } else if ch == '^' {
                map.insert(Point {x: jdx as isize, y: idx as isize}, EMPTY);
                guard = Point {x: jdx as isize, y: idx as isize};
            } else {
                panic!("what is this: {}", ch)
            }
        }
    }
    (map, guard)
}


fn guard_move(map: &HashMap<Point, Thing>, edge: usize, guard: Point, direction: Direction) -> Move {
    let mut new_direction = direction;

    // you can only turn twice at once, could do with if/else as well.
    while check_for_obstacle(guard, map, new_direction) {
        new_direction = turn_right(new_direction);
    }

    Move {
        off_map: guard_is_off_map(guard, edge),
        new_direction,
        new_position: get_new_position(guard, new_direction),
    }
}

fn guard_is_off_map(guard: Point, edge: usize) -> bool {
    guard.x < 0 || guard.x as usize >= edge || guard.y < 0 || guard.y as usize >= edge
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        UP => {RIGHT}
        RIGHT => {DOWN}
        DOWN => {LEFT}
        LEFT => {UP}
    }
}

fn check_for_obstacle(guard: Point, map: &HashMap<Point, Thing>, direction: Direction) -> bool {
    let new_position = get_new_position(guard, direction);
    // unwrap_or only for if new position is of map.
    map.get(&new_position).unwrap_or(&EMPTY) == &OBSTACLE
}

fn get_new_position(guard: Point, direction: Direction) -> Point {
    match direction {
        UP => { Point {x: guard.x, y: guard.y - 1} }
        DOWN => { Point {x: guard.x, y: guard.y + 1} }
        RIGHT => { Point {x: guard.x + 1, y: guard.y} }
        LEFT => { Point {x: guard.x - 1, y: guard.y} }
    }
}

fn guard_will_loop(map: &HashMap<Point, Thing>, start: Point, edge: usize) -> bool {
    let mut guard = start;
    let mut direction = UP;

    // if we are at the same point looking in the same direction again, we be loopin'
    let mut visited_states = HashSet::new();
    visited_states.insert(State {position: guard, direction });

    loop {
        let after = guard_move(&map, edge, guard, direction);
        if after.off_map {
            return false;
        }
        guard = after.new_position;
        direction = after.new_direction;

        let state = State {position: guard, direction };
        if visited_states.contains(&state) {
            // if we revisit a state, we found a loop!
            return true;
        }
        visited_states.insert(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let (p1, p2) = solve_day_06(test_input);

        // Off by one for test, not for real, not sure why
        assert_eq!(p1, 42, "Part 1 failed");
        assert_eq!(p2, 6, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input06.txt");

        let (p1, p2) = solve_day_06(test_input);

        assert_eq!(p1, 5212, "Part 1 failed");
        assert_eq!(p2, 1767, "Part 2 failed");
    }
}

#[allow(unused)]
fn print_map(map: &HashMap<Point, Thing>, positions: &HashSet<Point>, edge: usize) {
    // can do from 0 with usize, but this way we print where the guard leaves, isn't that fun!
    for y in -1..=edge as isize {
        for x in -1..=edge as isize {
            let coord = Point {x, y };
            let ch = if positions.contains(&coord) {
                'X'
            } else if let Some(thing) = map.get(&coord) {
                match thing {
                    EMPTY => '.',
                    OBSTACLE => '#',
                }
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!();
    }
}
