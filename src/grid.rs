use glifparser::{Guideline, IntegerOrFloat::Float};
use skia::{Canvas, Color, Paint, PaintStyle, Path};
use skulpin::skia_safe as skia;
use MFEKmath::rect::FlipIfRequired as _;

use crate::constants::{GRID_STROKE, GRID_THICKNESS};
use crate::guidelines::draw_guideline_impl;
use crate::viewport::Viewport;

#[derive(Copy, Clone, Debug)]
pub struct Grid {
    /// both X and Y
    pub offset: f32,
    /// space between grid lines
    pub spacing: f32,
    /// if Some(_) then there's an italic element to the grid
    pub slope: Option<f32>,
    /// whether to draw
    pub show: bool,
}

impl Default for Grid {
    fn default() -> Self {
        let offset = 0.;
        let spacing = 30.;
        let slope = None;
        Grid {
            offset,
            spacing,
            slope,
            show: false,
        }
    }
}

impl Grid {
    pub fn slope_degrees(&self) -> Option<f32> {
        if let Some(slope) = self.slope {
            Some((f32::to_degrees(f32::atan(slope)) * 10000.).round() / 10000.)
        } else {
            None
        }
    }
}

pub fn draw(canvas: &mut Canvas, grid: &Grid, viewport: &Viewport) {
    if !grid.show {
        return;
    }

    let mut path = Path::new();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let color = GRID_STROKE;
    let scolor = Color::from(color);
    paint.set_color(scolor);
    paint.set_stroke_width(GRID_THICKNESS * (1. / viewport.factor));
    paint.set_style(PaintStyle::Stroke);
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

    // Draw vertical guidelines (based on viewport location)
    let left = (units_from_left + origin.x.floor()) as isize;
    for i in (left..left + winsize.0 as isize).step_by(grid.spacing as usize) {
        let guideline = Guideline::<()>::from_x_y_angle(
            (i as f32).floor() + grid.offset,
            grid.offset,
            Float(90.),
        );
        draw_guideline_impl(viewport, canvas, &guideline, Some(GRID_STROKE), &mut path);
    }

    // Draw horizontal guidelines (based on viewport location)
    let top = (units_from_top + origin.y.floor()) as isize;
    for i in (top..top + winsize.1 as isize).step_by(grid.spacing as usize) {
        let guideline = Guideline::<()>::from_x_y_angle(
            grid.offset,
            (i as f32).floor() + grid.offset,
            Float(0.),
        );
        draw_guideline_impl(viewport, canvas, &guideline, Some(GRID_STROKE), &mut path);
    }

    let total_horizontal =
        f32::floor(viewport.winsize.1 as f32 / viewport.factor / grid.spacing) as i32;

    // Draw italic guidelines (based on viewport size, but always origin-relative, so possible to
    // escape)
    if let Some(angle) = grid.slope_degrees() {
        let gslope = grid.slope.unwrap();
        let viewx = viewport.winsize.0 as f32 / viewport.factor;
        let slope_max: f32 = if gslope < 0. {
            f32::min(-1., gslope)
        } else {
            f32::max(1., gslope)
        };
        let spacing = grid.spacing * slope_max;
        for i in -total_horizontal..total_horizontal {
            let guideline = Guideline::<()>::from_x_y_angle(
                viewx + grid.offset,
                gslope * viewx + spacing * (i as f32).floor() + grid.offset,
                Float(angle),
            );
            draw_guideline_impl(viewport, canvas, &guideline, Some(GRID_STROKE), &mut path);
        }
    }

    canvas.draw_path(&path, &paint);
}
