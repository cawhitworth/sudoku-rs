mod multiset;
mod grid;
mod candidates;
use candidates::{get_candidates, GridCandidates};
use grid::{check_grid, Grid};

fn solve_step(g: &mut Grid) -> grid::GridState {
    let result = check_grid(g);
    if result == grid::GridState::Invalid {
        return result
    }

    match get_candidates(g) {
        None => result,
        Some(gc) => {
            for row in 0..8 {
                for col in 0..8 {
                    if gc[row][col].len() == 1 {
                        let c: Vec<&u8> = (&gc[row][col]).into_iter().collect();
                        g.set(row, col, grid::Cell::Value(*c[0]));
                    }
                }
            }
            check_grid(g)
        }
    }
}

fn main() {
    let partial = String::from(
           ".842..7..\
            ...86.42.\
            3.51....9\
            .59.26.7.\
            2.3...8.6\
            ...49....\
            .9.7.236.\
            .1..5.24.\
            5.26.....");

    let mut g = Grid::from(&partial);

    let result = check_grid(&g);

    println!("{:?}", result);

    println!("{}", g);

    let s2 = solve_step(&mut g);
    println!("{:?}", result);

    println!("{}", g);
}

