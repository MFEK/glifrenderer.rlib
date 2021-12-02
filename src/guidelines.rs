use crate::string::{self, UiString};
use crate::viewport::Viewport;

use super::constants::*;
use super::points::calc::*;

use MFEKmath::rect::FlipIfRequired as _;

use flo_curves as flo;
use glifparser::{Guideline, GuidelinePoint, IntegerOrFloat};
use kurbo;
use skulpin::skia_safe::{self as skia, Canvas, Color, Paint, PaintStyle, Path};

// This works by making four infinitely long lines at all edges of the viewport, then considering a
// guideline, also of infinite length, finding where it intersects with all edges, and drawing it.
pub(crate) fn draw_guideline_impl<PD: glifparser::PointData>(
    viewport: &Viewport,
    canvas: &mut Canvas,
    guideline: &Guideline<PD>,
    color: Option<u32>,
) -> Path {
    let factor = viewport.factor;
    let mut sk_c_bounds = canvas.local_clip_bounds().unwrap();
    sk_c_bounds.flip_if_required();
    let origin = skia::Point::new(sk_c_bounds.left, sk_c_bounds.bottom);

    let angle = f64::from(guideline.angle).to_radians();
    let angle_vec = kurbo::Vec2::from_angle(angle);
    let mut window_rect: skia::Rect = skia::Rect::from(canvas.local_clip_bounds().unwrap());
    window_rect.flip_if_required();

    // flo_curves calls lines just tuples of its Coord2's. In certain functions these are
    // considered infinite lines, not line segments
    let top_line = (
        flo::geo::Coord2::from((window_rect.left(), window_rect.top())),
        flo::geo::Coord2::from((window_rect.right(), window_rect.top())),
    );
    let bottom_line = (
        flo::geo::Coord2::from((window_rect.left(), window_rect.bottom())),
        flo::geo::Coord2::from((window_rect.right(), window_rect.bottom())),
    );
    let left_line = (
        flo::geo::Coord2::from((window_rect.left(), window_rect.top())),
        flo::geo::Coord2::from((window_rect.left(), window_rect.bottom())),
    );
    let right_line = (
        flo::geo::Coord2::from((window_rect.right(), window_rect.top())),
        flo::geo::Coord2::from((window_rect.right(), window_rect.bottom())),
    );

    // flo_curves-style line for current guideline
    let guideline_at = flo::geo::Coord2::from((calc_x(guideline.at.x), calc_y(guideline.at.y)));
    let guideline_ext =
        flo::geo::Coord2::from((guideline_at.0 + angle_vec.x, guideline_at.1 + angle_vec.y));
    let guideline_as_line = (guideline_at, guideline_ext);

    // flo_curves somewhat bizarrely calls maths infinite lines `rays` and maths line segments
    // `lines`. perhaps 3d graphics influenced its naming conventions? so a ray is not a ray, nor
    // is a line a line.
    let intersect_top = flo::line::line_intersects_ray(&top_line, &guideline_as_line);
    let intersect_left = flo::line::line_intersects_ray(&left_line, &guideline_as_line);
    let intersect_bottom = flo::line::line_intersects_ray(&bottom_line, &guideline_as_line);
    let intersect_right = flo::line::line_intersects_ray(&right_line, &guideline_as_line);

    let mut intersections = vec![];
    for intersection in [
        intersect_bottom,
        intersect_top,
        intersect_right,
        intersect_left,
    ] {
        if let Some(rir) = intersection {
            intersections.push(rir);
        }
    }

    // when guideline is on screen
    let (at2, at3) = if intersections.len() >= 2 {
        (
            GuidelinePoint {
                x: intersections[0].0 as f32,
                y: intersections[0].1 as f32,
            },
            GuidelinePoint {
                x: intersections[1].0 as f32,
                y: intersections[1].1 as f32,
            },
        )
    } else {
        // when it's not
        return Path::new();
    };

    let mut path = Path::new();
    path.move_to(((at2.x), (at2.y)));
    path.line_to(((at3.x), (at3.y)));
    if let Some(ref name) = guideline.name {
        let mut at = at2;
        // Our bottom is their top because we're -1 y flipped compared (only matters for baselines)
        let vcenter = if intersect_bottom.is_some() {
            string::VerticalAlignment::Top
        } else {
            // we only want this to trigger if not at top on purpose
            string::VerticalAlignment::Bottom
        };
        // This implements the sliding guideline labels along 0° angled guidelines
        let alignment = if angle == 0. && -origin.x <= window_rect.width() {
            let origin_offset = 2.5; // offset from origin of label
            if origin.x >= 0. {
                at.x = origin.x + origin_offset;
                string::Alignment::Left
            } else {
                at.x = origin_offset;
                string::Alignment::Left
            }
        // Otherwise it uses some sane defaults
        } else {
            at.x -= 5. * (1. / factor);
            string::Alignment::Right
        };
        UiString::with_colors(name, color.unwrap_or(GUIDELINE_STROKE), None)
            .autosized(string::AutoSizeMode::OnlySmaller)
            .padding(1.)
            .alignment(alignment)
            .vcenter(vcenter)
            .draw(&viewport, at.into(), canvas);
    }
    path
}

pub fn draw_guideline<PD: glifparser::PointData>(
    viewport: &Viewport,
    canvas: &mut Canvas,
    guideline: &Guideline<PD>,
    color: Option<u32>,
) {
    let path = draw_guideline_impl(viewport, canvas, guideline, color);
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    let color = color.unwrap_or(GUIDELINE_STROKE);
    let scolor = Color::from(color);
    paint.set_color(scolor);
    paint.set_stroke_width(GUIDELINE_THICKNESS * (1. / viewport.factor));
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path, &paint);
}

/// Convenience function for drawing the baseline.
pub fn draw_baseline<PD: glifparser::PointData>(viewport: &Viewport, canvas: &mut Canvas) {
    draw_guideline::<PD>(
        viewport,
        canvas,
        &Guideline::from_x_y_angle(0., 0., IntegerOrFloat::Float(0.)),
        Some(BASELINE_STROKE),
    );
}
