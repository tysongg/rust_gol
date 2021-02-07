use rust_gol::{CellState, Grid};
use std::thread::sleep;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let delay = Duration::from_millis(250);
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        let mut grid = Grid::new(usize::from(w), usize::from(h));

        let pattern = convert_to_pattern(vec![
            vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
        ]);

        grid.set_pattern(
            grid.columns / 2 - pattern[0].len() / 2,
            grid.rows / 2 - pattern.len() / 2,
            pattern,
        )
        .unwrap();

        print_grid(&grid);

        loop {
            grid.tick().unwrap();
            print_grid(&grid);
            sleep(delay);
        }
    }
}

fn convert_to_pattern(input: Vec<Vec<u8>>) -> Vec<Vec<CellState>> {
    let mut rows: Vec<Vec<CellState>> = vec![];
    for row in input.iter() {
        let mut column: Vec<CellState> = vec![];
        for cell in row.iter() {
            match cell {
                1 => column.push(CellState::Alive),
                _ => column.push(CellState::Dead),
            }
        }
        rows.push(column);
    }
    return rows;
}

fn print_grid(grid: &Grid) {
    for y in 0..grid.rows {
        for x in 0..grid.columns {
            match grid.state[y][x] {
                CellState::Alive => {
                    print!("X");
                }
                CellState::Dead => {
                    print!(".");
                }
            }
        }
        println!("");
    }
    // let seperator = (0..grid.columns).map(|_| "=").collect::<String>();
    // println!("{}", seperator)
}
