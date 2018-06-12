use stdweb::{
    unstable::TryInto,
    web::{
        document, event::ResizeEvent, html_element::*, window, CanvasRenderingContext2d,
        IEventTarget, INonElementParentNode, TextAlign, TextBaseline,
    },
};

use sudoku::ui::model::Game;
use sudoku::{Element, Sudoku};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

const DARK_BG: &'static str = "#001d29";
const DARK_GRID: &'static str = "rgba(240, 240, 240, 0.3)";

const LIGHT_BG: &'static str = "hsl(0, 0%, 99%)";
const LIGHT_GRID: &'static str = "rgba(15, 15, 15, 0.3)";

// partial_min
#[cfg_attr(rustfmt, rustfmt_skip)]
fn min(l: f64, r: f64) -> f64 {
    if l > r { r } else { l }
}

pub fn play(game: Rc<RefCell<Game>>) {
    render(Some(&game.borrow()));
    // Downgrading the reference here will do nicely
    let weak = Rc::downgrade(&game);
    window().add_event_listener(move |_: ResizeEvent| {
        if let Some(game) = weak.upgrade() {
            render(Some(&game.borrow()));
        } else {
            render(None);
        }
    });
    /*while !game.borrow().is_finished() {

    }*/
}

pub fn render(game: Option<&Game>) {
    let canvas: CanvasElement = document()
        .get_element_by_id("canvas")
        .unwrap()
        .try_into()
        .unwrap();
    canvas.set_width(window().inner_width() as u32);
    canvas.set_height(window().inner_height() as u32);
    let context = canvas.get_context::<CanvasRenderingContext2d>().unwrap();
    context.set_fill_style_color(DARK_BG);
    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    context.fill_rect(0.0, 0.0, width, height);
    context.set_stroke_style_color(DARK_GRID);

    let order: u8 = 3;
    let axis = order.pow(2);

    let center = (width / 2.0, height / 2.0);
    let length = min(0.9 * width, 0.9 * height);
    let spacing = length / (axis as f64);

    let left = center.0 - (axis as f64) * spacing / 2.0;
    let top = center.1 - (axis as f64) * spacing / 2.0;

    for i in 0..=axis {
        context.set_line_width(if i % order == 0 { 4.0 } else { 2.0 });
        context.begin_path();
        context.move_to(left + (i as f64) * spacing, top);
        context.line_to(left + (i as f64) * spacing, top + length);
        context.move_to(left, top + (i as f64) * spacing);
        context.line_to(left + length, top + (i as f64) * spacing);
        context.stroke();
    }

    let font_size = length / 14.0;
    context.set_font(&format!("{}px sans-serif", font_size));
    context.set_text_baseline(TextBaseline::Middle);
    context.set_text_align(TextAlign::Center);
    if let Some(game) = game {
        let angles = [0, 15, 40, 60, 100, 150, 230, 275, 315];
        let colors = angles
            .into_iter()
            .map(|angle| format!("hsl({}, 70%, 50%)", angle))
            .collect::<Vec<_>>();
        for point in game.points() {
            if let Some(Element(value)) = game.current[point] {
                let x = point[0];
                let y = point[1];
                let color = &colors[(value - 1) as usize];
                context.set_fill_style_color(color);
                context.fill_text(
                    &format!("{}", value),
                    left + spacing * (x as f64 + 0.5),
                    top + spacing * (y as f64 + 0.5),
                    None,
                );
            }
        }
    }
}
