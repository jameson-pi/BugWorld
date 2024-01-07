use std::rc::Rc;
use std::cell::RefCell;

enum Direction {
    North,
    South,
    East,
    West,
}

struct Cell {
    north_cell: Option<Rc<RefCell<Cell>>>,
    south_cell: Option<Rc<RefCell<Cell>>>,
    east_cell: Option<Rc<RefCell<Cell>>>,
    west_cell: Option<Rc<RefCell<Cell>>>,
    bug: bool,
}

impl Cell {
    fn get_neighbor(&self, direction: Direction) -> Option<Rc<RefCell<Cell>>> {
        match direction {
            Direction::North => self.north_cell.clone(),
            Direction::South => self.south_cell.clone(),
            Direction::East => self.east_cell.clone(),
            Direction::West => self.west_cell.clone(),
        }
    }
}

struct Arena {
    cells: Vec<Rc<RefCell<Cell>>>,
    width: usize,
    height: usize,
}

impl Arena {
    fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(Rc::new(RefCell::new(Cell {
                north_cell: None,
                south_cell: None,
                east_cell: None,
                west_cell: None,
                bug: false,
            })));
        }

        // Link cells together for a 2D grid structure
        for y in 0..height {
            for x in 0..width {
                let index: usize = y * width + x;
                let cell = cells[index].clone();
                if y > 0 {
                    let north_index = index - width;
                    let north_cell = cells[north_index].clone();
                    cell.borrow_mut().north_cell = Some(north_cell);
                }
                if y < height - 1 {
                    let south_index = index + width;
                    let south_cell = cells[south_index].clone();
                    cell.borrow_mut().south_cell = Some(south_cell);
                }
                if x > 0 {
                    let west_index = index - 1;
                    let west_cell = cells[west_index].clone();
                    cell.borrow_mut().west_cell = Some(west_cell);
                }
                if x < width - 1 {
                    let east_index = index + 1;
                    let east_cell = cells[east_index].clone();
                    cell.borrow_mut().east_cell = Some(east_cell);
                }
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    // Access a cell at a given (x, y) coordinate
    fn get_cell(&self, x: usize, y: usize) -> Option<Rc<RefCell<Cell>>> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x].clone())
        } else {
            None
        }
    }
}

struct Bug {
    direction: Direction,
    position: Rc<RefCell<Cell>>,
    is_alive: bool,
}