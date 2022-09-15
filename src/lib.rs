mod random;
mod minesweeper;

use minesweeper::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getGame)]
pub fn get_game() -> String {
    let ms = MineSweeper::new(10, 10, 10);

    ms.to_string()
}