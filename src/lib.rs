mod random;

use random::random_number;

type Position = (usize, usize);

struct MineSweeper {
    width: usize,
    height: usize,
    open_cells: Vec<Position>,
    mines: Vec<Position>,
    flagged_cells: Vec<Position>,
}

impl MineSweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> MineSweeper {
        MineSweeper {
            width,
            height,
            open_cells: Vec::new(),
            mines: {
                let mut mines:Vec<Position> = Vec::new();

                while mines.len() < mine_count {
                    mines.push((random_number(width), random_number(height)))
                }

                mines
            },
            flagged_cells: Vec::new(),
        }
    }
}