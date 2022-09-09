use std::ops::Index;
use std::fmt::{Display, Write};

use crate::random::random_number;

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

impl Display for MineSweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_cells.contains(&pos) {
                    if self.flagged_cells.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } else if self.mines.contains(&pos) {
                f.write_str("💣 ")?;
                } else {
                let mine_count = self.neighbor_mines(pos);

                if mine_count > 0 {
                    write!(f, " {} ", mine_count)?;
                } else {
                    f.write_str("⬜ ")?;
                }
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
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
    use crate::MineSweeper::MineSweeper;

    #[test]
    fn test_new_game() {
        let mut ms = MineSweeper::new(10, 10, 5);
        ms.open_cell((5, 5));
        ms.toggle_flag((6, 6));
        ms.open_cell((6, 6));

        println!("{}", ms);
    }
}
