use std::collections::{HashMap, HashSet, VecDeque};


pub fn solve_day_12(input: &str) -> (usize, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

#[derive(Debug)]
struct Region {
    // I expected the letter to be actually important for part 2, can be ignored.
    // letter: char,
    size: usize,
    border: usize,
    sides: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn solve(input: &str) -> (usize, usize) {
    let mut plotted = HashSet::new();
    let mut matrix = HashMap::new();
    // as so often, AoC is nice enough to be square
    let mut limit = 0;

    for (row, line) in input.lines().enumerate() {
        limit = line.len();
        for (col, c) in line.chars().enumerate() {
            matrix.insert((row, col), c);
        }
    }

    let mut regions = Vec::new();

    // flooding
    for row in 0..limit {
        for col in 0..limit {
            if plotted.contains(&(row, col)) {
                continue;
            }
            let region = flood_fill_region(&mut plotted, &matrix, limit, row, col);
            regions.push(region);
        }
    }

    let p1 = regions.iter().map(|r| {
        r.border * r.size
    }).sum();
    let p2 = regions.iter().map(|r| {
        r.sides * r.size
    }).sum();
    (p1, p2)
}

fn flood_fill_region(plotted: &mut HashSet<(usize, usize)>, matrix: &HashMap<(usize, usize), char>, limit: usize, row: usize, col: usize) -> Region {
    let mut coords = HashSet::new();
    let letter = *matrix.get(&(row, col)).unwrap();
    let mut region = Region {
        //letter,
        size: 0,
        border: 0,
        sides: 0,
    };
    let mut queue = VecDeque::new();
    queue.push_back((row, col));

    while let Some((y, x)) = queue.pop_front() {
        coords.insert((y as i32, x as i32));
        if plotted.contains(&(y, x)) {
            continue;
        }
        plotted.insert((y, x));
        region.size += 1;
        for &(dy, dx) in &DIRECTIONS {
            let neighbor_y = y as isize + dy;
            let neighbor_x = x as isize + dx;

            if legal_coord(limit, neighbor_y, neighbor_x) {
                let neighbor_pos = (neighbor_y as usize, neighbor_x as usize);
                let neighbor_letter = *matrix.get(&neighbor_pos).unwrap();

                if neighbor_letter == letter {
                    coords.insert((y as i32, x as i32));
                    if !plotted.contains(&neighbor_pos) {
                        queue.push_back(neighbor_pos);
                    }
                } else {
                    region.border += 1;
                }
            } else {
                region.border += 1;
            }
        }
    };
    region.sides = count_sides(&coords);
    region
}

fn legal_coord(limit: usize, neighbor_y: isize, neighbor_x: isize) -> bool {
    neighbor_y >= 0
        && neighbor_y < limit as isize
        && neighbor_x >= 0
        && neighbor_x < limit as isize
}

fn count_sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;
    for dir in DIRECTIONS {
        let outside_perimeter = get_outside_coords(region, dir);
        let duplicates = count_duplicate_sides(dir, &outside_perimeter);
        side_count += outside_perimeter.len() - duplicates;
    }
    side_count
}

fn count_duplicate_sides(dir: (isize, isize), outside_perimeter: &HashSet<(i32, i32)>) -> usize {
    let mut duplicates = HashSet::new();
    for side in outside_perimeter.iter() {
        // not really "duplicates"
        // we just move in a direction and if we are still in the outside perimeter we did not change direction
        // if we remove all that are encounted while moving in the same direction, we get the number of corners/sides
        let mut potential_duplicate = (side.0 + dir.1 as i32, side.1 + dir.0 as i32);
        while outside_perimeter.contains(&potential_duplicate) {
            duplicates.insert(potential_duplicate);
            potential_duplicate = (potential_duplicate.0 + dir.1 as i32, potential_duplicate.1 + dir.0 as i32);
        }
    }
    duplicates.len()
}

fn get_outside_coords(region: &HashSet<(i32, i32)>, dir: (isize, isize)) -> HashSet<(i32, i32)> {
    // coords just one outside of the region.
    let mut outside_perimeter = HashSet::new();
    for pos in region {
        let potential_outside = (pos.0 + dir.0 as i32, pos.1 + dir.1 as i32);
        if !region.contains(&potential_outside) {
            outside_perimeter.insert(potential_outside);
        }
    }
    outside_perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let (p1, p2) = solve_day_12(test_input);

        assert_eq!(p1, 1930, "Part 1 failed");
        assert_eq!(p2, 1206, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input12.txt");

        let (p1, p2) = solve_day_12(test_input);

        assert_eq!(p1, 1377008, "Part 1 failed");
        assert_eq!(p2, 815788, "Part 2 failed");
    }
}
