
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use BugWorld::{Arena, Cell, Direction};

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