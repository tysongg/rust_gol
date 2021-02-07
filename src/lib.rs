pub type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Clone)]
pub struct GridError;

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

#[derive(Debug)]
pub struct Grid {
    pub columns: usize,
    pub rows: usize,

    pub state: Vec<Vec<CellState>>,
}

impl Grid {
    pub fn new(columns: usize, rows: usize) -> Grid {
        return Grid {
            columns: columns,
            rows: rows,
            state: vec![vec![CellState::Dead; columns]; rows],
        };
    }

    pub fn set_pattern(&mut self, x: usize, y: usize, pattern: Vec<Vec<CellState>>) -> Result<()> {
        if y + pattern.len() > self.rows {
            return Err("Invalid position");
        }
        if x + pattern[0].len() > self.columns {
            return Err("Invalid position");
        }

        let mut row_index = 0;
        for row in pattern.iter() {
            let mut cell_index = 0;
            for cell in row.iter() {
                self.state[row_index + y][cell_index + x] = *cell;
                cell_index += 1;
            }
            row_index += 1;
        }
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        // Fetch all cells that are currently alive
        let mut live_cells: Vec<Point> = vec![];
        for (y, row) in self.state.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    CellState::Alive => live_cells.push(Point { x, y }),
                    _ => (),
                }
            }
        }

        let mut cells_to_kill: Vec<&Point> = vec![];
        let mut cells_to_birth: Vec<&Point> = vec![];

        let mut dead_neighbors_to_check: Vec<Point> = vec![];
        for cell in live_cells.iter() {
            // Look for livin neighbors
            let live_neighbors = self.get_alive_neighbor_count(cell);
            if live_neighbors < 2 || live_neighbors > 3 {
                cells_to_kill.push(cell);
            }

            // Mark cells we need to check to see if they are now alive
            for direction in [
                Direction::TopLeft,
                Direction::Top,
                Direction::TopRight,
                Direction::Right,
                Direction::BottomRight,
                Direction::Bottom,
                Direction::BottomLeft,
                Direction::Left,
            ]
            .iter()
            {
                match self.get_neighbor_state(cell, direction) {
                    Ok(CellState::Dead) => {
                        let neighbor_position = self.get_neighbor_point(cell, direction)?;

                        if !dead_neighbors_to_check.contains(&neighbor_position) {
                            dead_neighbors_to_check.push(neighbor_position);
                        }
                    }
                    _ => (),
                }
            }
        }

        // Check the dead cells to see if they should become alive
        for cell in dead_neighbors_to_check.iter() {
            let live_neighbor_count = self.get_alive_neighbor_count(cell);
            if live_neighbor_count == 3 {
                cells_to_birth.push(cell);
            }
        }

        // Kill Cells
        for cell in cells_to_kill {
            self.state[cell.y][cell.x] = CellState::Dead;
        }

        // Birth Cells
        for cell in cells_to_birth {
            self.state[cell.y][cell.x] = CellState::Alive;
        }

        Ok(())
    }

    fn get_neighbor_state(
        &self,
        position: &Point,
        direction: &Direction,
    ) -> std::result::Result<CellState, &'static str> {
        let neighbor_position = self.get_neighbor_point(position, direction)?;

        Ok(self.state[neighbor_position.y][neighbor_position.x])
    }

    fn get_neighbor_point(
        &self,
        position: &Point,
        direction: &Direction,
    ) -> std::result::Result<Point, &'static str> {
        if position.x == 0
            || position.y == 0
            || position.x >= self.columns - 1
            || position.y >= self.rows - 1
        {
            return Err("No valid neighbor");
        }
        let (x, y) = match direction {
            Direction::TopLeft => (position.x - 1, position.y - 1),
            Direction::Top => (position.x, position.y - 1),
            Direction::TopRight => (position.x + 1, position.y - 1),
            Direction::Right => (position.x + 1, position.y),
            Direction::BottomRight => (position.x + 1, position.y + 1),
            Direction::Bottom => (position.x, position.y + 1),
            Direction::BottomLeft => (position.x - 1, position.y + 1),
            Direction::Left => (position.x - 1, position.y),
        };
        Ok(Point { x, y })
    }

    fn get_neighbor_states(&self, position: &Point) -> Vec<CellState> {
        let mut states: Vec<CellState> = vec![];

        for direction in [
            Direction::TopLeft,
            Direction::Top,
            Direction::TopRight,
            Direction::Right,
            Direction::BottomRight,
            Direction::Bottom,
            Direction::BottomLeft,
            Direction::Left,
        ]
        .iter()
        {
            match self.get_neighbor_state(position, direction) {
                Ok(e) => states.push(e),
                _ => (),
            }
        }
        states
    }

    fn get_alive_neighbor_count(&self, position: &Point) -> u8 {
        let mut alive_count = 0;
        for state in self.get_neighbor_states(position).iter() {
            match state {
                CellState::Alive => alive_count += 1,
                _ => (),
            }
        }
        alive_count
    }
}
