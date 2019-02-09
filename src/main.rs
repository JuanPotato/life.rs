extern crate grid;

use grid::Grid;
use std::{thread, time};

/// The number of columns that will be printed to the terminal (same as the number of columns in
/// the game grid).
const WIDTH: usize = 80;

/// The number of rows that will be printed to the terminal.
///
/// Note that the number of rows in the game grid is twice this amount, as each printed row is two
/// game rows.
const HEIGHT: usize = 20;

const ALIVE_CHANCE: f64 = 0.2;
const SLEEP_TIME_MS: u64 = 60;

fn make_grid() -> Grid {
    let mut grid = Grid::new(WIDTH, HEIGHT * 2);
    grid.randomise(ALIVE_CHANCE);
    grid
}

/// Count the number of neighbouring cells that are alive, not including itself
/// Wraps around the grid edges, effectively making it a torus
fn alive_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0;
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (x_o, y_o) in &offsets {
        count += grid[(x_o + x as isize, y_o + y as isize)] as u8;
    }
    count
}

/// Compute the next generation from `grid` and place the result in `next`
fn generation(grid: &Grid, next: &mut Grid) {
    for (x, y, &alive) in grid.iter() {
        next[(x, y)] = match alive_neighbours(&grid, x, y) {
            2 => alive,
            3 => true,
            _ => false,
        };
    }
}

fn draw(grid: &Grid) {
    println!("{}", grid.stringify());
    println!("\x1b[0;0H");
}

fn main() {
    let mut grid = make_grid();
    let mut next_grid = Grid::new(WIDTH, HEIGHT * 2);
    let mut iter_count = usize::max_value();

    if let Some(iters) = std::env::args().nth(1) {
        iter_count = iters.parse().unwrap_or_else(|_| {
            panic!("Could not parse generation count \"{}\" as integer", iters)
        });
    }

    println!("\x1b[2J\x1b[H");
    for _ in 0..iter_count {
        draw(&grid);
        generation(&grid, &mut next_grid);
        std::mem::swap(&mut next_grid, &mut grid);
        thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
    }
}
