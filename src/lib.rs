mod random;

use std::ops::Index;

use random::random_number;

type Position = (usize, usize);

enum OpenResult {
    Mine,
    NoMine(u8),
    Flagged,
}

#[derive(Debug)]
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
                    let new_mine = (random_number(width), random_number(height));
                    if !mines.contains(&new_mine) {
                        mines.push((random_number(width), random_number(height)))
                    }
                }

                mines
            },
            flagged_cells: Vec::new(),
        }
    }

    pub fn open_cell(&mut self, position: Position) -> OpenResult{
        if self.flagged_cells.contains(&position) {
            return OpenResult::Flagged;
        }
        self.open_cells.push(position);

        let is_mine = self.mines.contains(&position);
        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(8)
        }
    }

    pub fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.min(1) - 1..=(x + 1).max(width - 1)).flat_map(move |i| (
                y.min(1) - 1..=(y + 1).max(height - 1)).map(move |j| (i, j)
        )).filter(move |&pos| pos != (x, y))
    }

    pub fn neighbor_mines(&self, pos: Position) -> u8 {
        self
            .neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.open_cells.contains(&pos) {
            return;
        }
        if self.flagged_cells.contains(&pos) {
            let index = self.flagged_cells.iter().position(|&item| item == pos).unwrap();
            self.flagged_cells.remove(index);
        } else {
            self.flagged_cells.push(pos);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MineSweeper;

    #[test]
    fn test_new_game() {
        let mine_sweeper = MineSweeper::new(10, 10, 5);

        println!("{:?}", mine_sweeper);
    }
}
