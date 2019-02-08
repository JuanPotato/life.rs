extern crate rand;

use std::{thread, time};
use rand::{Rng, thread_rng};

type GridRow = [bool; SIZE];
type Grid = [GridRow; SIZE];

const SIZE: usize        = 30;
const INIT: usize        = 100;
const SLEEP_TIME_MS: u64 = 200;

fn make_grid() -> Grid {
    let mut rng = thread_rng();
    let mut grid = [[false; SIZE]; SIZE];

    // Could also use rng.gen_bool() with a probability during generation.
    for _ in 0..INIT {
        let x: usize = rng.gen_range(0, SIZE);
        let y: usize = rng.gen_range(0, SIZE);
        grid[x][y] = true;
    }
    grid
}

fn stringify_row(row: &GridRow) -> String {
    row.iter()
        .map(|c| match c {
                    true => 'o',
                    _    => ' '
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
                continue
            }
            let xindex = xindex as usize;
            let yindex = yindex as usize;
            if xindex >= SIZE || yindex >= SIZE {
                continue
            }

            count += grid[xindex][yindex] as u8;
        }
    }
    count
}

fn generation(grid: Grid) -> Grid {
    let mut next = grid.clone();
    for (x, row) in grid.iter().enumerate() {
        for (y, alive) in row.iter().enumerate() {
            let n = match alive {
                true => alive_neighbours(&grid, x as isize, y as isize) - 1,
                _    => alive_neighbours(&grid, x as isize, y as isize)
            };
            match n {
                2 => (),
                3 => next[x][y] = true,
                _ => next[x][y] = false
            };

        }
    }
    next
}

fn draw(grid: Grid) {
    println!("{}", grid.iter()
             .map(|row| stringify_row(row))
             .collect::<Vec<String>>()
             .join("\n")
             );
}

fn main() {
    let mut g = make_grid();
    println!("{}", "\x1b[2J\x1b[H");
    loop {
        draw(g);
        println!("{}", "\x1b[0;0H");
        g = generation(g);
        thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
    }
}
