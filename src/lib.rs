enum Direction {
    North,
    South,
    East,
    West
}
struct Cell {
    north_cell: Option<Box<Cell>>,
    south_cell: Option<Box<Cell>>,
    east_cell: Option<Box<Cell>>,
    west_cell: Option<Box<Cell>>
}
impl Cell {
    fn get_neighbor(&self, direction: Direction) -> Option<&Cell> {
        match direction {
            Direction::North => self.north_cell.as_deref(),
            Direction::South => self.south_cell.as_deref(),
            Direction::East => self.east_cell.as_deref(),
            Direction::West => self.west_cell.as_deref(),
        }
    }
}
struct Arena {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Arena {
    fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(Cell {
                north_cell: None,
                south_cell: None,
                east_cell: None,
                west_cell: None,
            });
        }

        // Link cells together for a 2D grid structure
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let cell = &mut cells[index];

                if y > 0 {
                    cell.north_cell = Some(cells[index - width]);
                }
                if y < height - 1 {
                    cell.south_cell = Some(cells[index + width]);
                }
                if x > 0 {
                    cell.west_cell = Some(cells[index - 1]);
                }
                if x < width - 1 {
                    cell.east_cell = Some(cells[index + 1]);
                }
            }
        }

        Self { cells, width, height }
    }

    // Access a cell at a given (x, y) coordinate
    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }
}

struct Bug {
    direction: Direction,
    position: Cell,
    is_alive: bool
}