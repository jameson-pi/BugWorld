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
                let cell: Rc<RefCell<Cell>> = cells[index].clone();
                if y > 0 {
                    let north_index: usize = index - width;
                    let north_cell: Rc<RefCell<Cell>> = cells[north_index].clone();
                    cell.borrow_mut().north_cell = Some(north_cell);
                }
                if y < height - 1 {
                    let south_index: usize = index + width;
                    let south_cell: Rc<RefCell<Cell>> = cells[south_index].clone();
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






















#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::{Arena, Cell, Direction}; 
    #[test]
    fn test_get_neighbor() {
        let cell1 = Rc::new(RefCell::new(Cell {
            north_cell: None,
            south_cell: None,
            east_cell: None,
            west_cell: None,
            bug: false,
        }));

        let cell2 = Rc::new(RefCell::new(Cell {
            north_cell: Some(cell1.clone()),
            south_cell: None,
            east_cell: None,
            west_cell: None,
            bug: false,
        }));

        cell1.borrow_mut().north_cell = Some(cell2.clone());

        assert_eq!(cell1.borrow().get_neighbor(Direction::North), Some(cell2.clone()));
        assert_eq!(cell2.borrow().get_neighbor(Direction::South), Some(cell1.clone()));
        assert_eq!(cell1.borrow().get_neighbor(Direction::East), None);
        assert_eq!(cell1.borrow().get_neighbor(Direction::West), None);
    }

    #[test]
    fn test_get_cell() {
        let arena = Arena::new(3, 3);

        assert_eq!(arena.get_cell(0, 0), Some(arena.cells[0].clone()));
        assert_eq!(arena.get_cell(2, 2), Some(arena.cells[8].clone()));
        assert_eq!(arena.get_cell(3, 3), None);
        assert_eq!(arena.get_cell(1, 4), None);
    }
}