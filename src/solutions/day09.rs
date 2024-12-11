use std::collections::HashMap;
use rayon::prelude::*;

pub fn solve_day_09(input: &str) -> (usize, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

struct FileWithID {
    file_id: usize,
    start: usize,
    length: usize,
}

fn solve(input: &str) -> (usize, usize) {
    // needed for part 1
    let mut free_indices = Vec::<usize>::new();
    let mut filesystem = HashMap::<usize, usize>::new();

    // needed for part 2
    let mut free_blocks = HashMap::<usize, usize>::new();
    let mut file_blocks = HashMap::<usize, (usize, usize)>::new();

    // needed for the parsing loop
    let mut filesystem_position: usize = 0;
    let mut file_id: usize = 0;
    let mut is_file_segment = true;

    for n in input.lines().next().unwrap().chars() {
        let segment_length: usize = n.to_digit(10).unwrap() as usize;
        if is_file_segment {
            for j in 0..segment_length {
                filesystem.insert(filesystem_position + j, file_id);
            }
            file_blocks.insert(filesystem_position, (file_id, segment_length));
            is_file_segment = false;
            file_id += 1;
        } else {
            free_blocks.insert(filesystem_position, segment_length);
            for j in 0..segment_length {
                free_indices.push(filesystem_position + j);
            }
            is_file_segment = true;
        }
        filesystem_position = filesystem_position + segment_length;
    }

    let p1 = part1(&mut free_indices, &mut filesystem);
    let p2 = part2(&mut free_blocks, &mut file_blocks);

    (p1, p2)
}

fn part2(free_blocks: &mut HashMap<usize, usize>, file_blocks: &mut HashMap<usize, (usize, usize)>) -> usize {
    let mut files_with_ids = file_blocks
        .iter()
        .map(|(&start, &(file_id, length))| FileWithID {
            file_id,
            start,
            length,
        })
        .collect::<Vec<FileWithID>>();

    files_with_ids.sort_by(|a, b| b.file_id.cmp(&a.file_id));

    for file in &files_with_ids {
        let start_idx = file.start;
        let file_len = file.length;

        // this is faster than testing the free spaces from start to end, taking the first fitting one.
        let mut possible_free_spaces = free_blocks
            .iter()
            .filter(|(&pos, &size)| pos < start_idx && size >= file_len)
            .map(|(&k, &t)| (k, t))
            .collect::<Vec<(usize, usize)>>();

        possible_free_spaces.sort_by_key(|&(k, _)| k);

        if let Some(&(k, t)) = possible_free_spaces.first() {
            free_blocks.remove(&k);

            // If free space remains after allocation, insert the remaining space
            if t > file_len {
                free_blocks.insert(k + file_len, t - file_len);
            }

            file_blocks.remove(&start_idx);
            file_blocks.insert(k, (file.file_id, file_len));
            free_blocks.insert(start_idx, file_len);
        }
    }

    file_blocks
        .par_iter()
        .map(|(&k, &(file_id, length))| {
            (0..length)
                .into_par_iter()
                .map(|offset| (k + offset) * file_id)
                .sum::<usize>()
        })
        .sum()
}

fn part1(free_indices: &mut Vec<usize>, filesystem: &mut HashMap<usize, usize>) -> usize {
    let mut rev_free = free_indices.iter().rev().collect::<Vec<&usize>>();
    let mut keys: Vec<usize> = filesystem.keys().cloned().collect();
    keys.sort();
    while rev_free.len() > 1 {
        let free_idx = rev_free.pop().unwrap();
        let file_idx = keys.pop().unwrap();
        let file = filesystem.get(&file_idx).unwrap().clone();
        if *free_idx > file_idx {
            break;
        }
        filesystem.remove(&file_idx);
        filesystem.insert(*free_idx, file);
    }
    filesystem.par_iter().map(|(k, v)| k * v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "2333133121414131402";

        let (p1, p2) = solve_day_09(test_input);

        assert_eq!(p1, 1928, "Part 1 failed");
        assert_eq!(p2, 2858, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input09.txt");

        let (p1, p2) = solve_day_09(test_input);

        assert_eq!(p1, 6337921897505, "Part 1 failed");
        assert_eq!(p2, 6362722604045, "Part 2 failed");
    }
}
