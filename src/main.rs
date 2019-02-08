extern crate grid;

use grid::Grid;
use std::{thread, time};

const WIDTH: usize = 80;
const HEIGHT: usize = 20;
const ALIVE_CHANCE: f64 = 0.2;
const SLEEP_TIME_MS: u64 = 200;

fn make_grid() -> Grid {
    let mut grid = Grid::new(WIDTH, HEIGHT);
    grid.randomise(ALIVE_CHANCE);
    grid
}

fn alive_neighbours(grid: &Grid, x: isize, y: isize) -> u8 {
    // grid[((x - 1)%(SIZE as isize)) as usize..(x + 2) as usize].iter().map(|x| println!("{:?}", x));
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
        if x_o + x < 0 || y_o + y < 0 {
            continue;
        }
        let xindex = (x_o + x) as usize;
        let yindex = (y_o + y) as usize;

        if xindex >= WIDTH || yindex >= HEIGHT {
            continue;
        }

        count += grid[yindex][xindex] as u8;
    }
    count
}

fn generation(grid: &Grid, next: &mut Grid) {
    for (y, row) in grid.iter().enumerate() {
        for (x, alive) in row.iter().enumerate() {
            next[y][x] = match alive_neighbours(&grid, x as isize, y as isize) {
                2 => *alive,
                3 => true,
                _ => false,
            };
        }
    }
}

fn draw(grid: &Grid) {
    println!("{}", grid.stringify());
}

fn main() {
    let mut grid = make_grid();
    let mut next_grid = Grid::new(WIDTH, HEIGHT);
    let mut iter_count = usize::max_value();

    if let Some(iters) = std::env::args().nth(1) {
        iter_count = iters.parse().expect(&format!(
            "Could not parse generation count \"{}\" as integer",
            iters
        ));
    }

    println!("{}", "\x1b[2J\x1b[H");
    for _ in 0..iter_count {
        draw(&grid);
        println!("{}", "\x1b[0;0H");
        generation(&grid, &mut next_grid);
        std::mem::swap(&mut next_grid, &mut grid);
        thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
    }
}
