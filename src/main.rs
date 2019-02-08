extern crate rand;

use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;
use std::{thread, time};

type GridRow = [bool; SIZE];
type Grid = [GridRow; SIZE];

const SIZE: usize = 60;
const ALIVE_CHANCE: f64 = 0.1;
const SLEEP_TIME_MS: u64 = 200;

fn make_grid() -> Grid {
    let d = Bernoulli::new(ALIVE_CHANCE);
    let mut rng = thread_rng();
    let mut grid = [[false; SIZE]; SIZE];

    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[x][y] = d.sample(&mut rng);
        }
    }
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

        if xindex >= SIZE || yindex >= SIZE {
            continue;
        }

        count += grid[yindex][xindex] as u8;
    }
    count
}

fn generation(grid: Grid, next: &mut Grid) {
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

fn stringify_row(row: &GridRow) -> String {
    row.iter()
        .map(|c| match c {
            true => 'o',
            _ => ' ',
        })
        .collect()
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
