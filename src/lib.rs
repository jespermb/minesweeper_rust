
type Position = (usize, usize);

struct MineSweeper {
    width: usize,
    height: usize,
    open_cells: Vec<Position>,
    mines: Vec<Position>,
    flagged_cells: Vec<Position>,
}

impl MineSweeper {
    pub fn new(width: usize, height: usize) -> MineSweeper {
        MineSweeper {
            width,
            height,
            open_cells: Vec::new(),
            mines: Vec::new(),
            flagged_cells: Vec::new(),
        }
    }
}