#[path = "content_integration_core/grouped.rs"]
mod grouped;
#[path = "content_integration_core/individual.rs"]
mod individual;

pub(super) use grouped::grouped_treemap;
pub(super) use individual::individual_treemap;

use crate::treemap::Rect;
use crate::ui::theme::wrap_canvas;
use windows_reactor::*;

const CANVAS_WIDTH: f64 = 1000.0;
const CANVAS_HEIGHT: f64 = 540.0;

fn bounds() -> Rect {
    Rect::new(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT)
}

fn wrap_treemap_canvas(elements: Vec<View>) -> View {
    wrap_canvas(
        Canvas::new()
            .width(CANVAS_WIDTH)
            .height(CANVAS_HEIGHT)
            .children(elements)
            .into(),
    )
}

#[allow(clippy::too_many_arguments)]
fn treemap_block<F: Fn() + 'static>(
    rect: Rect,
    background: Color,
    border: Color,
    corner_radius: f64,
    label: String,
    font_size: f64,
    margin: Thickness,
    on_press: F,
) -> View {
    Border::new()
        .width(rect.width().max(2.0))
        .height(rect.height().max(2.0))
        .background(Brush::Solid(background))
        .border_brush(Brush::Solid(border))
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(corner_radius))
        .canvas_left(rect.x())
        .canvas_top(rect.y())
        .on_pointer_pressed(move |_| on_press())
        .content(
            TextBlock::new()
                .text(label)
                .font_size(font_size)
                .margin(margin),
        )
        .into()
}
