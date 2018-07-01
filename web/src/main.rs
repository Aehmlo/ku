#[macro_use]
extern crate stdweb;
extern crate sudoku;

use sudoku::{ui::model::Game, Difficulty, Point};

use std::{cell::RefCell, rc::Rc};

/// Represents the greater context of the current view state.
// Because this will contain references that are platform-specific, this lives here, not in ku::ui.
pub struct Context {
    game: Game,
    focused: Option<Point>,
}

impl Context {
    /// Constructs a context with a new game of the specified order and difficulty.
    pub fn new(order: u8, difficulty: Difficulty) -> Self {
        Self {
            game: Game::new(order, difficulty),
            focused: None,
        }
    }
}

mod view;

use view::{play, render};

fn main() {
    render(None);
    let context = Context::new(3, Difficulty::Advanced);
    let context = Rc::new(RefCell::new(context));
    play(context);
}
