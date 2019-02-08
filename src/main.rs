extern crate rand;

use rand::distributions::{Bernoulli, Distribution};
use rand::{thread_rng, Rng};
use std::{thread, time};

type GridRow = [bool; SIZE];
type Grid = [GridRow; SIZE];

const SIZE: usize = 60;
const FRAC_ALIVE: f64 = 0.1;
const SLEEP_TIME_MS: u64 = 200;

fn make_grid() -> Grid {
    let d = Bernoulli::new(FRAC_ALIVE);
    let mut rng = thread_rng();
    let mut grid = [[false; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[x][y] = d.sample(&mut rng);
        }
    }
    grid
}

fn stringify_row(row: &GridRow) -> String {
    row.iter()
        .map(|c| match c {
            true => 'o',
            _ => ' ',
        })
        .collect()
}

fn alive_neighbours(grid: &Grid, x: isize, y: isize) -> u8 {
    // Note this includes the co-ord in the center.
    // grid[((x - 1)%(SIZE as isize)) as usize..(x + 2) as usize].iter().map(|x| println!("{:?}", x));
    let mut count = 0;
    for xindex in x - 1..x + 2 {
        for yindex in y - 1..y + 2 {
            if xindex < 0 || yindex < 0 {
                continue;
            }
            let xindex = xindex as usize;
            let yindex = yindex as usize;
            if xindex >= SIZE || yindex >= SIZE {
                continue;
            }

            count += grid[xindex][yindex] as u8;
        }
    }
    count
}

fn generation(grid: Grid, next: &mut Grid) {
    for (x, row) in grid.iter().enumerate() {
        for (y, alive) in row.iter().enumerate() {
            let n = match alive {
                true => alive_neighbours(&grid, x as isize, y as isize) - 1,
                _ => alive_neighbours(&grid, x as isize, y as isize),
            };
            match n {
                2 => (),
                3 => next[x][y] = true,
                _ => next[x][y] = false,
            };
        }
    }
}

fn draw(grid: Grid) {
    println!(
        "{}",
        grid.iter()
            .map(|row| stringify_row(row))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn main() {
    let mut grid = make_grid();
    let mut next_grid = [[false; SIZE]; SIZE];
    let mut iter_count = usize::max_value();

    if let Some(iters) = std::env::args().nth(1) {
        iter_count = iters.parse().expect(&format!(
            "Could not parse generation count \"{}\" as integer",
            iters
        ));
    }

    println!("{}", "\x1b[2J\x1b[H");
    for _ in 0..iter_count {
        draw(grid);
        println!("{}", "\x1b[0;0H");
        generation(grid, &mut next_grid);
        std::mem::swap(&mut next_grid, &mut grid);
        thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
    }
}
