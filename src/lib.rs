extern crate rand;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

use rand::{thread_rng, Rng};

/// A representation of the grid of the Game of Life.
pub struct Grid {
    rows: Box<[Box<[bool]>]>,
}

impl Grid {
    /// Create a new empty (false-initialised) Grid. Width and height must be positive.
    pub fn new(width: usize, height: usize) -> Grid {
        if width == 0 || height == 0 {
            panic!("cannot create zero size grid");
        }

        Grid {
            rows: vec![vec![false; width].into_boxed_slice(); height].into_boxed_slice(),
        }
    }

    /// Set every cell's state randomly.
    /// `live_chance` is the probability that a given cell will be `true`.
    pub fn randomise(&mut self, live_chance: f64) {
        let mut rng = thread_rng();

        for y in 0..self.len() {
            for x in 0..self[0].len() {
                self[y][x] = rng.gen_bool(live_chance);
            }
        }
    }

    /// Set the cell at each (x, y) coordinate to `true`.
    ///
    /// `x_offset` and `y_offset` are added to each x and y.
    pub fn set_cells(&mut self, coords: &[(usize, usize)], x_offset: usize, y_offset: usize) {
        for (x, y) in coords {
            self[y_offset + y][x_offset + x] = true;
        }
    }

    /// Get an ASCII-art representation of the whole grid, with lines to indicate the edges.
    ///
    /// Note: the representation uses half the number of rows since two grid rows are rendered in
    /// each printed row.
    pub fn stringify(&self) -> String {
        let mut lines = Vec::with_capacity((self.len() / 2) + 2);
        lines.push("+".to_owned() + &"-".repeat(self[0].len()) + "+");
        self.rows
            .chunks(2)
            .map(|rows| stringify_row_pair(&rows[0], &rows[1]))
            .for_each(|s| lines.push(s));
        lines.push("+".to_owned() + &"-".repeat(self[0].len()) + "+");

        lines.join("\n")
    }

    /// Get an iterator over the rows of the grid.
    pub fn iter(&self) -> std::slice::Iter<Box<[bool]>> {
        self.rows.iter()
    }

    /// Get the number of rows in the grid.
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// Get an ASCII-art representation of a single line, intended for use with `Grid.stringify()`.
fn stringify_row_pair(up_row: &Box<[bool]>, down_row: &Box<[bool]>) -> String {
    let mut s = String::with_capacity(up_row.len() + 2);
    s.push('|');
    up_row.iter()
        .zip(down_row.iter())
        .map(|pair| match pair {
            (false, true) => '▄',
            (true, false) => '▀',
            (true, true) => '█',
            _ => ' '
        })
        .for_each(|c| s.push(c));
    s.push('|');

    s
}

/// Index directly into the rows of the grid.
impl Index<usize> for Grid {
    type Output = Box<[bool]>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

/// Index directly into the rows of the grid.
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
