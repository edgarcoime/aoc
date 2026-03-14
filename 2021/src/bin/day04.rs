use aoc::read_input;
use std::collections::HashSet;
use std::mem;

// Board needs to keep track of the numbers on the board
// as well as which numbers are marked (Set since we don't need to store a value)?
#[derive(Debug)]
struct Board {
    cells: Vec<Vec<usize>>,
    marked: HashSet<(usize, usize)>,
}

impl Board {
    fn cells_iter(&self) -> impl Iterator<Item = &usize> {
        self.cells.iter().flatten()
    }

    fn indexed_cells_iter(&self) -> impl Iterator<Item = (usize, usize, &usize)> {
        self.cells.iter().enumerate().flat_map(|(row, col)| {
            col.iter()
                .enumerate()
                .map(move |(col, val)| (row, col, val))
        })
    }

    fn calculate_unmarked_sum(&self) -> usize {
        self.indexed_cells_iter()
            .filter_map(|(row, col, val)| {
                match self.marked.contains(&(row, col)) {
                    true => None,
                    false => Some(*val),
                }
            })
            .sum()
    }

    // Returns if a draw was a hit on the board
    fn mark(&mut self, draw: usize) -> bool {
        let hit = self
            .indexed_cells_iter()
            .find(|(_, _, val)| **val == draw)
            .map(|(row, col, _)| (row, col));

        match hit {
            None => false,
            Some((row, col)) => {
                let entry = (row, col);
                self.marked.insert(entry);
                let size = self.cells.len();

                let vertical_hit = (0..size).all(|r| self.marked.contains(&(r, col)));
                if vertical_hit {
                    println!("vertical hit: row {row}, col {col}");
                    return true;
                }

                let horizontal_hit = (0..size).all(|c| self.marked.contains(&(row, c)));
                if horizontal_hit {
                    println!("horizontal hit: row {row}, col {col}");
                    return true;
                }

                false
            }
        }
    }
}

#[derive(Debug)]
struct ParsedInput {
    draws: Vec<usize>,
    boards: Vec<Board>,
}

fn parse(input: &str) -> ParsedInput {
    let mut lines = input.lines();
    let first = lines.next().unwrap();

    // Parse draws
    let draws: Vec<usize> = first
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    // Parse boards
    let mut boards: Vec<Board> = Vec::new();
    let mut board_cells: Vec<Vec<usize>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if !board_cells.is_empty() {
                // If cells are filled and current line is empty then that means done
                boards.push(Board {
                    // NOTE: Equivalent to `board_cells.clone()` but more efficient since we don't
                    // need the old value
                    cells: mem::take(&mut board_cells),
                    marked: HashSet::new(),
                });
            }

            continue;
        }

        // Parse the line and create Vec of usizes
        let row_cells: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        board_cells.push(row_cells);
    }

    boards.push(Board {
        cells: board_cells,
        marked: HashSet::new(),
    });

    ParsedInput { draws, boards }
}

fn part1(input: &str) -> Option<i64> {
    let state = parse(input);
    let draws = state.draws;
    let mut boards = state.boards;

    for draw in &draws {
        for board in &mut boards {
            let hit = board.mark(*draw);
            if hit {
                let unmarked_sum = board.calculate_unmarked_sum();
                let ans = *draw as i64 * unmarked_sum as i64;
                println!("{:?} {:?}", draw, board);
                println!("unmarked sum: {unmarked_sum}, ans: {ans}");
                return Some(ans);
            }
        }
    }

    None
}

fn part2(input: &str) -> Option<i64> {
    let state = parse(input);
    let draws = state.draws;
    let mut boards = state.boards;
    let mut completed = vec![false; boards.len()];
    let mut completed_count = 0usize;
    let total_boards = boards.len();

    for draw in &draws {
        for (idx, board) in boards.iter_mut().enumerate() {
            let hit = board.mark(*draw);

            if hit {
                println!("board {idx} hit with draw {draw}");
                completed_count += 1;
                completed[idx] = true;

                if completed_count >= total_boards {
                    let unmarked_sum = board.calculate_unmarked_sum();
                    let ans = *draw as i64 * unmarked_sum as i64;
                    println!("{:?} {:?}", draw, board);
                    println!("unmarked sum: {unmarked_sum}, ans: {ans}");
                    return Some(ans);
                }
            }
        }
    }

    None
}

fn main() {
    let input1 = read_input("inputs/day04.txt");
    // let p1 = part1(&input1);
    // println!("{p1:?}");

    // let input2 = read_input("inputs/day04.txt");
    let input2 = read_input("examples/day04.txt");
    let p2 = part2(&input2);
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = read_input("examples/day04.txt");
        assert_eq!(part1(&input), Some(4512));
    }

    #[test]
    fn test_part2_sample() {
        let input = read_input("examples/day04.txt");
        assert_eq!(part2(&input), Some(1924));
    }
}
