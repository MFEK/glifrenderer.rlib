use MFEKmath::rect::FlipIfRequired as _;
use glifparser::{IntegerOrFloat::Float, Guideline};
use skulpin::skia_safe as skia;
use skia::Canvas;

use crate::constants::GRID_STROKE;
use crate::guidelines::draw_guideline;
use crate::viewport::Viewport;

#[derive(Copy, Clone, Debug)]
pub struct Grid {
    pub offset: f32,
    pub spacing: f32,
    pub slope: Option<f32>,
}

impl Grid {
    pub fn slope_degrees(&self) -> Option<f32> {
        if let Some(slope) = self.slope {
            Some( (f32::to_degrees(f32::atan(slope)) * 10000.).round() / 10000. )
        } else {
            None
        }
    }
}

pub fn draw(canvas: &mut Canvas, grid: &Grid, viewport: &Viewport) {
    // This works by making fake guidelines.
    let dmatrix = viewport.as_device_matrix();
    let mut sk_c_bounds = canvas.local_clip_bounds().unwrap();
    sk_c_bounds.flip_if_required();
    let origin = skia::Point::new(sk_c_bounds.left, sk_c_bounds.top);
    let factor = dmatrix.scale_x();
    let winsize = viewport.winsize;
    let winsize = (winsize.0 * factor.recip(), winsize.1 * factor.recip());

    let whole_left_offset = grid.offset + -origin.x / grid.spacing;
    let fractional_left_offset = whole_left_offset - whole_left_offset.floor();
    let units_from_left = fractional_left_offset * grid.spacing;

    let whole_top_offset = grid.offset + -origin.y / grid.spacing;
    let fractional_top_offset = whole_top_offset - whole_top_offset.floor();
    let units_from_top = fractional_top_offset * grid.spacing;

    let left = (units_from_left + origin.x.floor()) as isize;
    for i in (left .. left + winsize.0 as isize).step_by(grid.spacing as usize) {
        let guideline = Guideline::<()>::from_x_y_angle((i as f32).floor() + grid.offset, grid.offset, Float(90.));
        draw_guideline(viewport, canvas, &guideline, Some(GRID_STROKE));
    }

    let top = (units_from_top + origin.y.floor()) as isize;
    for i in (top .. top + winsize.1 as isize).step_by(grid.spacing as usize) {
        let guideline = Guideline::<()>::from_x_y_angle(grid.offset, (i as f32).floor() + grid.offset, Float(0.));
        draw_guideline(viewport, canvas, &guideline, Some(GRID_STROKE));
    }

    let offset = dmatrix.map_origin();
    let offset = (offset.x, offset.y);
    let total_horizontal = f32::floor(viewport.winsize.1 as f32 / viewport.factor / grid.spacing) as i32;

    let winsize = (winsize.0 * 2., winsize.1 * 2.);
    if let Some(slope) = grid.slope_degrees() {
        let gslope = grid.slope.unwrap();
        let viewx = viewport.winsize.0 as f32 / viewport.factor;
        let slope_max: f32 = if gslope < 0. { f32::min(-1., gslope) } else { f32::max(1., gslope) };
        let spacing = grid.spacing * slope_max;
        let goffset = ((grid.offset + offset.0 + offset.1 * gslope.abs()) / spacing)
            .fract()
            * spacing;
        for i in -total_horizontal..total_horizontal {
            let guideline = Guideline::<()>::from_x_y_angle(viewx + grid.offset, gslope * viewx + spacing * (i as f32).floor() + grid.offset, Float(slope));
            draw_guideline(viewport, canvas, &guideline, Some(GRID_STROKE));
        }
    }
}
