extern crate stdweb;
extern crate sudoku;

use sudoku::ui::model::Game;
use sudoku::{Difficulty, Sudoku};

use std::rc::Rc;
use std::cell::RefCell;

mod view;

use view::{play, render};

fn main() {
	render(None);
	let game = Game::new(3, Difficulty::Advanced);
	let game = Rc::new(RefCell::new(game));
	play(game);
}
