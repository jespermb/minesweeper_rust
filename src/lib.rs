mod random;
mod minesweeper;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
    static GAME: RefCell<MineSweeper> = RefCell::new(MineSweeper::new(10, 10, 10));
}

#[wasm_bindgen(js_name = getGame)]
pub fn get_game() -> String {
    return GAME.with(|game| game.borrow().to_string());
}

#[wasm_bindgen(js_name = openCell)]
pub fn open_field(x: usize, y:usize) -> String {
    GAME.with(|game| {
        game.borrow_mut().open_cell((x, y));
        return game.borrow().to_string();
    })
}
