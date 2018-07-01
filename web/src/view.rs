use stdweb::{
    traits::{IEvent, IKeyboardEvent, IMouseEvent},
    unstable::TryInto,
    web::{
        document,
        event::{ClickEvent, KeyDownEvent, ResizeEvent},
        html_element::*,
        window, CanvasRenderingContext2d, IEventTarget, INonElementParentNode, TextAlign,
        TextBaseline,
    },
};

use Context;

use sudoku::{ui::model::Game, Difficulty, Element, Point};

use std::{
    cell::RefCell,
    rc::Rc,
};

#[cfg(not(feature = "light_ui"))]
const TEXT: &'static str = "#fff";
#[cfg(not(feature = "light_ui"))]
const BG: &'static str = "#001d29";
#[cfg(not(feature = "light_ui"))]
const GRID: &'static str = "rgba(240, 240, 240, 0.3)";
#[cfg(not(feature = "light_ui"))]
const HIGHLIGHT: &'static str = "rgba(240, 240, 240, 0.2)";
#[cfg(not(feature = "light_ui"))]
const SUB_HIGHLIGHT: &'static str = "rgba(240, 240, 240, 0.1)";

#[cfg(feature = "light_ui")]
const TEXT: &'static str = "#555";
#[cfg(feature = "light_ui")]
const BG: &'static str = "hsl(0, 0%, 99%)";
#[cfg(feature = "light_ui")]
const GRID: &'static str = "rgba(15, 15, 15, 0.3)";
#[cfg(feature = "light_ui")]
const HIGHLIGHT: &'static str = "rgba(15, 15, 15, 0.1)";
#[cfg(feature = "light_ui")]
const SUB_HIGHLIGHT: &'static str = "rgba(15, 15, 15, 0.05)";

const COLORIZE_ON_HIGHLIGHT: bool = true;

// partial_min
#[cfg_attr(rustfmt, rustfmt_skip)]
fn min(l: f64, r: f64) -> f64 {
    if l > r { r } else { l }
}

fn get_order(context: &Option<&Context>) -> u8 {
    context.map(|c| c.game.current.order).unwrap_or(3)
}

fn grid_length() -> f64 {
    let (width, height) = (
        window().inner_width() as f64,
        window().inner_height() as f64,
    );
    min(0.9 * width, 0.9 * height)
}

fn get_canvas() -> CanvasElement {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .try_into()
        .unwrap()
}

fn grid_origin(context: &Option<&Context>) -> (f64, f64) {
    let axis = get_order(&context).pow(2);

    let (width, height) = (
        window().inner_width() as f64,
        window().inner_height() as f64,
    );

    let center = (width / 2.0, height / 2.0);
    let length = grid_length();
    let spacing = length / (axis as f64);

    let left = center.0 - (axis as f64) * spacing / 2.0;
    let top = center.1 - (axis as f64) * spacing / 2.0;

    (left, top)
}

fn point_for_click(context: &Context, click: &ClickEvent) -> Option<Point> {
    let origin = grid_origin(&Some(context));
    let length = grid_length();
    let max = (origin.0 + length, origin.1 + length);
    let locus = (click.client_x() as f64, click.client_y() as f64);
    if locus.0 < origin.0 || locus.0 > max.0 || locus.1 < origin.1 || locus.1 > max.1 {
        None
    } else {
        let order = context.game.current.order as f64;
        let axis = order.powf(2.0);
        let specific = length / axis;
        let x = ((locus.0 - origin.0) / specific).floor() as u8;
        let y = ((locus.1 - origin.1) / specific).floor() as u8;
        Some(Point([x, y]))
    }
}

pub fn play(context: Rc<RefCell<Context>>) {
    render(Some(&context.borrow()));
    let resize_context = context.clone();
    let click_context = context.clone();
    let key_context = context.clone();
    window().add_event_listener(move |_: ResizeEvent| {
        let context = &resize_context;
        render(Some(&context.borrow()));
    });
    let canvas = get_canvas();
    document().add_event_listener(move |event: KeyDownEvent| {
        if let Ok(mut context) = key_context.try_borrow_mut() {
            if let Some(point) = context.focused {
                match event.key().as_str() {
                    "Backspace" | "Delete" => {
                        event.prevent_default();
                        if context.game.is_mutable(point) {
                            let _old = context.game.remove(point);
                            render(Some(&context));
                        }
                    }
                    "Escape" => {
                        context.focused = None;
                        render(Some(&context));
                    }
                    key => {
                        if let Ok(value) = key.parse::<u8>() {
                            let order = get_order(&Some(&context));
                            if value > 0 && value <= order.pow(2) {
                                let element = Element(value);
                                if context.game.insertion_is_correct(point, element) {
                                    context.game.insert(point, element);
                                    render(Some(&context));
                                    // This will need to change to is_solved if the behvaior of insertion
                                    // changes to allow incorrect insertions.
                                    if context.game.current.is_complete() {
                                        let congrats =
                                            format!("Sudoku solved in {} moves!", context.game.moves);
                                        js! { alert(@{congrats}); }
                                        context.game =
                                            Game::new(context.game.current.order, Difficulty::Advanced);
                                        context.focused = None;
                                        render(Some(&context));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    canvas.add_event_listener(move |event: ClickEvent| {
        if let Ok(mut context) = click_context.try_borrow_mut() {
            context.focused = point_for_click(&context, &event);
            render(Some(&context));
        }
    });
}

pub fn fill_box(
    ctx: &CanvasRenderingContext2d,
    context: &Context,
    point: Point,
    color: &'static str,
) {
    let (left, top) = grid_origin(&Some(context));
    let axis = get_order(&Some(context)).pow(2);
    let length = grid_length();
    let spacing = length / (axis as f64);
    ctx.set_fill_style_color(color);
    ctx.fill_rect(
        left + point[0] as f64 * spacing,
        top + point[1] as f64 * spacing,
        spacing,
        spacing,
    );
    ctx.set_fill_style_color(BG);
}

pub fn render(context: Option<&Context>) {
    let canvas: CanvasElement = get_canvas();
    canvas.set_width(window().inner_width() as u32);
    canvas.set_height(window().inner_height() as u32);
    let ctx = canvas.get_context::<CanvasRenderingContext2d>().unwrap();
    ctx.set_fill_style_color(BG);
    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    ctx.fill_rect(0.0, 0.0, width, height);
    ctx.set_stroke_style_color(GRID);

    let (left, top) = grid_origin(&context);
    let order = get_order(&context);
    let axis = order.pow(2);
    let length = grid_length();
    let spacing = length / (axis as f64);

    for i in 0..=axis {
        ctx.set_line_width(if i % order == 0 { 4.0 } else { 2.0 });
        ctx.begin_path();
        ctx.move_to(left + (i as f64) * spacing, top);
        ctx.line_to(left + (i as f64) * spacing, top + length);
        ctx.move_to(left, top + (i as f64) * spacing);
        ctx.line_to(left + length, top + (i as f64) * spacing);
        ctx.stroke();
    }

    let font_size = length / 14.0;
    ctx.set_font(&format!("{}px sans-serif", font_size));
    ctx.set_text_baseline(TextBaseline::Middle);
    ctx.set_text_align(TextAlign::Center);
    if let Some(context) = context {
        let highlighted: Option<Vec<Point>> = context.focused.map(|f| {
            let mut group = context.game.current.group_indices(f);
            group.sort();
            group.dedup();
            group
        });
        if let Some(ref group) = &highlighted {
            for point in group {
                fill_box(&ctx, &context, *point, SUB_HIGHLIGHT);
            }
        }
        if let Some(focused) = context.focused {
            fill_box(&ctx, &context, focused, HIGHLIGHT);
        }
        let focused_value = context.focused.and_then(|p| context.game.current[p]);
        let angles = [0, 15, 40, 60, 100, 160, 230, 275, 315];
        let colors = angles
            .into_iter()
            .map(|angle| format!("hsl({}, 70%, 50%)", angle))
            .collect::<Vec<_>>();
        let highlighted = highlighted.unwrap_or_default();
        for point in context.game.points() {
            if let Some(Element(value)) = context.game.current[point] {
                let x = point[0];
                let y = point[1];
                let color = if COLORIZE_ON_HIGHLIGHT && !highlighted.contains(&point) && Some(Element(value)) != focused_value {
                    TEXT
                } else {
                    &colors[(value - 1) as usize]
                };
                ctx.set_fill_style_color(color);
                ctx.fill_text(
                    &format!("{}", value),
                    left + spacing * (x as f64 + 0.5),
                    top + spacing * (y as f64 + 0.5),
                    None,
                );
            }
        }
    }
}
