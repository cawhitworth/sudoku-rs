use core::fmt;
use std::convert::TryInto;

use crate::multiset::MultiSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell
{
    Value(u8),
    Empty
}

impl Cell
{
    pub fn from(c: char) -> Cell 
    {
        match c {
            '.' => Cell::Empty,
            n if n >= '1' && n <='9' =>
                Cell::Value(n.to_digit(10).unwrap().try_into().unwrap()),
            _ => panic!("Invalid character for Cell")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GridState {
    Complete,
    Incomplete,
    Invalid
}

#[derive(Debug)]
pub struct Grid
{
    grid: Vec<Vec<Cell>>
}

impl fmt::Display for Grid
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: Vec<String> = vec![];
        for row in 0..8 {
            let line = self.grid[row]
                .iter()
                .map( |c| match c {
                    Cell::Empty => ".".to_string(),
                    Cell::Value(v) => v.to_string()
                })
                .collect();

            output.push(line);
        }
        write!(f, "{}", output.join("\n"))
    }
}

impl Grid
{
    pub fn new() -> Grid
    {
        Grid {
            grid: vec![vec![Cell::Empty; 9]; 9]
        }
    }

    pub fn from(grid: &String) -> Grid
    {
        let mut g = Self::new();

        for (i, c) in grid.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            g.grid[row][col] = Cell::from(c);
        }

        g
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row >= 0 && row < 9 && col >= 0 && col < 9 {
            Some(self.grid[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, c: Cell) -> Option<Cell> {
        if row >= 0 && row < 9 && col >= 0 && col < 9 {
            self.grid[row][col] = c;
            Some(self.grid[row][col])
        } else {
            None
        }
    }
}

pub fn house(row: usize, col: usize) -> usize {
    ((row /3) * 3) + (col/3)
}

pub type Set = MultiSet<u8>;

pub fn check_grid(grid: &Grid) -> GridState {

    let mut row_entries = Set::new(9);
    let mut col_entries = Set::new(9);
    let mut house_entries = Set::new(9);

    for row in 0..9 {
        for col in 0..9 {
            let house = house(row, col);

            let c = &grid.grid[row][col];

            match c {
                Cell::Value(n) => {
                    if !row_entries.insert(row, *n) {
                        return GridState::Invalid
                    }
                    if !col_entries.insert(col, *n) {
                        return GridState::Invalid
                    }
                    if !house_entries.insert(house, *n) {
                        return GridState::Invalid
                    }
                },
                Cell::Empty => {
                    return GridState::Incomplete
                }
            }
        }
    }

    GridState::Complete
}