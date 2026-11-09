// Game of Life, Weeks 12-13: grid, display, neighbor counts, the rule for one cell,
// generations, and preset patterns.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

// TODO: add the imports you need (the pattern and generation prompts use
// crate::input::read_int; waiting for Enter uses std::io).

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

pub fn print_grid(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!("{}", if grid[r][c] { '#' } else { '.' });
        }
        println!();
    }
}

// Neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 && grid[r as usize][c as usize] {
                count += 1;
            }
        }
    }
    count
}

pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

pub fn print_neighbor_counts(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!("{}", count_neighbors(grid, r, c));
        }
        println!();
    }
}

// The next generation. Read from `grid` and write to a NEW grid, so a cell is never
// judged using neighbors that have already been updated.
pub fn next_generation(_grid: &Grid) -> Grid {
    todo!("Implement next_generation")
}

// Preset patterns placed near the middle of the grid (the glider starts near the
// top-left so it has room to travel):
//   1 = blinker: three cells in a row
//   2 = block: a 2x2 square
//   3 = glider: five cells
pub fn load_pattern(_choice: u32) -> Grid {
    todo!("Implement load_pattern")
}

// Number of live cells.
pub fn population(_grid: &Grid) -> usize {
    todo!("Implement population")
}

// TODO: replace this Week 12 version. New play(): show the pattern menu (1 blinker,
// 2 block, 3 glider), ask how many generations, then print each generation with its
// number and population, waiting for Enter between steps.
pub fn play() {
    let mut grid = new_grid();
    grid[4][10] = true;
    grid[5][10] = true;
    grid[6][10] = true;
    print_grid(&grid);
    println!();
    print_neighbor_counts(&grid);
    println!();
    let alive = grid[5][10];
    let neighbors = count_neighbors(&grid, 5, 10);
    println!("Center cell next generation: {}", if next_state(alive, neighbors) { "alive" } else { "dead" });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(cells: &[(usize, usize)]) -> Grid {
        let mut grid = new_grid();
        for &(r, c) in cells {
            grid[r][c] = true;
        }
        grid
    }

    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    // Supplied by the instructor.
    #[test]
    fn block_is_still() {
        let block = grid_from(&[(4, 10), (4, 11), (5, 10), (5, 11)]);
        assert_eq!(next_generation(&block), block);
    }

    #[test]
    fn blinker_returns_after_two() {
        let start = grid_from(&[(4, 10), (5, 10), (6, 10)]);
        let after_two = next_generation(&next_generation(&start));
        assert_eq!(after_two, start);
    }

    // TODO (student): the glider test (after four generations on an empty grid the
    // population is 5 and the shape has moved one row and one column along its
    // direction of travel), and one more test of your own choosing.
}
