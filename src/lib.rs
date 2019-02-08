extern crate rand;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;

pub struct Grid {
    rows: Box<[Box<[bool]>]>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        if width == 0 || height == 0 {
            panic!("cannot create zero size grid");
        }
        Grid {
            rows: vec![vec![false; width].into_boxed_slice(); height].into_boxed_slice(),
        }
    }

    pub fn randomise(&mut self, live_chance: f64) {
        let d = Bernoulli::new(live_chance);
        let mut rng = thread_rng();

        for y in 0..self.len() {
            for x in 0..self[0].len() {
                self[y][x] = d.sample(&mut rng);
            }
        }
    }

    pub fn set_cells(&mut self, coords: &[(usize, usize)], x_offset: usize, y_offset: usize) {
        for (x, y) in coords {
            self[y_offset + y][x_offset + x] = true;
        }
    }

    pub fn stringify(&self) -> String {
        self
            .iter()
            .map(|row| stringify_row(row))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn iter(&self) -> std::slice::Iter<Box<[bool]>> {
        self.rows.iter()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

fn stringify_row(row: &Box<[bool]>) -> String {
    row.iter()
        .map(|c| match c {
            true => 'o',
            _ => ' ',
        })
        .collect()
}

impl Index<usize> for Grid {
    type Output = Box<[bool]>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
