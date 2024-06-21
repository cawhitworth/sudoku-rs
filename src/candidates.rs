use std::collections::HashSet;

use crate::grid::{ Cell, check_grid, Grid, GridState, house };
use crate::multiset::MultiSet;

pub type Candidates = HashSet<u8>;

fn all() -> Candidates {
    HashSet::from([1,2,3,4,5,6,7,8,9])
}

pub type GridCandidates = Vec<Vec<Candidates>>;

pub fn get_candidates(grid: &Grid) -> Option<GridCandidates> {
    let mut gc = vec![vec![all(); 9]; 9];

    // For every cell:
    // If the cell contains a value:
    // - that cell has no candidates
    // - remove that value from all others in the same row, column, and house

    for row in 0..9 {
        for col in 0..9 {
            let cell = &grid.get(row, col)?;
            match cell {
                Cell::Empty => continue,
                Cell::Value(v) => {
                    gc[row][col].clear();

                    for index in 0..9 {
                        gc[index][col].remove(v);
                        gc[row][index].remove(v);

                        let house_row = (row / 3)*3;
                        let house_col = (col / 3)*3;

                        let index_row = index / 3;
                        let index_col = index % 3;

                        gc[house_row + index_row][house_col + index_col].remove(v);
                    }
                }
            }
        }
    }

    Some(gc)
}