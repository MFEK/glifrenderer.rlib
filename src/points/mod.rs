use glifparser::glif::Layer;
use glifparser::glif::contour::MFEKContourCommon;
use glifparser::glif::point::MFEKPointCommon;
use glifparser::outline::skia::ToSkiaPath;
use MFEKmath::mfek::ResolveCubic as _;

use skia_safe::{
    Canvas, ContourMeasureIter, Matrix, Paint, PaintStyle, Path as SkPath, Point as SkPoint,
    Rect as SkRect, Vector,
};
use std::collections::HashSet;

pub mod names;

use super::constants::*;
use crate::toggles::{HandleStyle, PointLabels};
use crate::viewport::Viewport;

use glifparser::{
    Handle as GPHandle, MFEKGlif, Point as GPPoint, PointData as GPPointData, WhichHandle,
};

type Color = u32;

#[derive(Clone, Copy, PartialEq)]
pub enum UIPointType {
    Point((GPHandle, GPHandle)),
    GPHandle,
    Direction,
}

trait SkiaFromGlyph<PD: GPPointData> {
    fn from_glif(p: &GPPoint<PD>) -> SkPoint;
}

impl<PD: GPPointData> SkiaFromGlyph<PD> for SkPoint {
    fn from_glif(p: &GPPoint<PD>) -> Self {
        SkPoint::from((p.x, p.y))
    }
}


pub fn draw_directions<PD: GPPointData>(
    viewport: &Viewport,
    layer: &Layer<PD>,
    canvas: &Canvas,
    selected: &HashSet<(usize, usize)>,
    only_selected: bool,
) {
    let selected: HashSet<usize> = selected.into_iter().map(|(ci, _pi)| *ci).collect();
    for (ci, c) in layer.outline.iter().map(|c|c.to_cubic()).enumerate() {
        if c.is_empty() {
            return
        }
        let piter = ContourMeasureIter::from_path(&c.inner().cubic().unwrap().to_skia_path(None).unwrap(), false, None);
        for cm in piter {
            // Get vector and tangent -4 Skia units along the contur
            let (vec, tan) = cm.pos_tan(-4.).unwrap();
            if !only_selected || (only_selected && selected.contains(&ci)) {
                draw_triangle_point(viewport, vec, tan, false, canvas);
            }
        }
    }
}

// For direction markers, not a "real" point So, we make three paths. `path` we return; `path2` is
// 90 deg offset from `path1`, both of which are lines created by elongating Skia vectors. `path2`
// is rotated at its center, such that they form an X. We elongate `path1` a bit so the final
// triangle is not isoceles. We then move to the "point" (path2[1]), make a line to the second
// point (on the base), finish that segment, and close the path.
fn draw_triangle_point(
    viewport: &Viewport,
    at: SkPoint,
    along: Vector,
    selected: bool,
    canvas: &Canvas,
) {
    let (fill, stroke) = get_fill_and_stroke(UIPointType::Direction, selected);
    let factor = viewport.factor;
    let mut paint = Paint::default();
    paint.set_stroke_width(DIRECTION_STROKE_THICKNESS * (1. / factor));
    paint.set_anti_alias(true);

    let mut path = SkPath::new();
    let mut path1 = SkPath::new();

    let mut vec = along.clone();
    vec.set_length(TRIANGLE_POINT_AREA * (1. / factor));

    let mut matrix = Matrix::new_identity();
    matrix.set_rotate(90., at + vec);

    vec.set_length(TRIANGLE_POINT_AREA * 2.5 * (1. / factor));

    path1.move_to(at + vec);
    path1.line_to(at);
    let mut path2 = SkPath::new();
    //vec.set_length(10.);
    vec.set_length(TRIANGLE_POINT_AREA * 2. * (1. / factor));
    path2.move_to(at + vec);
    path2.line_to(at);
    path2.transform(&matrix);

    let points1_count = path1.count_points();
    let mut points1 = vec![SkPoint::default(); points1_count];
    path1.get_points(&mut points1);

    let points2_count = path2.count_points();
    let mut points2 = vec![SkPoint::default(); points2_count];
    path2.get_points(&mut points2);

    path.move_to(points2[1]);
    path.line_to(points2[0]);
    path.line_to(points1[0]);
    path.close();

    paint.set_style(PaintStyle::StrokeAndFill);
    paint.set_color(fill);
    canvas.draw_path(&path, &paint);
    paint.set_style(PaintStyle::Stroke);
    paint.set_color(stroke);
    canvas.draw_path(&path, &paint);
}

pub fn draw_round_point(
    at: (f32, f32),
    radius: f32,
    stroke: Color,
    alpha: f32,
    canvas: &Canvas,
    factor: f32,
) {
    let mut paint = Paint::default();
    paint.set_stroke_width(DIRECTION_STROKE_THICKNESS * (1. / factor));
    paint.set_anti_alias(true);

    paint.set_style(PaintStyle::Stroke);
    paint.set_color(stroke);
    paint.set_alpha_f(alpha);
    canvas.draw_circle((at.0, at.1), radius, &paint);
}

pub fn draw_square_point(
    at: (f32, f32),
    radius: f32,
    stroke: Color,
    _fill: Color,
    canvas: &Canvas,
    factor: f32,
) {
    let mut paint = Paint::default();
    paint.set_stroke_width(DIRECTION_STROKE_THICKNESS * (1. / factor));
    paint.set_anti_alias(true);

    let mut path = SkPath::new();
    path.add_rect(
        SkRect::from_point_and_size((at.0 - radius / 2., at.1 - radius / 2.), (radius, radius)),
        None,
    );
    path.close();
    paint.set_color(stroke);
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path, &paint);
}


pub fn draw_cross_point(
    at: (f32, f32),
    radius: f32,
    stroke: Color,
    alpha: f32,
    canvas: &Canvas,
    factor: f32,
) {
    let mut paint = Paint::default();
    paint.set_stroke_width(DIRECTION_STROKE_THICKNESS * (1. / factor));
    paint.set_anti_alias(true);

    paint.set_style(PaintStyle::Stroke);
    paint.set_color(stroke);
    paint.set_alpha_f(alpha);
    canvas.draw_line((at.0 - radius, at.1 - radius), (at.0 + radius, at.1 + radius), &paint);
    canvas.draw_line((at.0 + radius, at.1 - radius), (at.0 - radius, at.1 + radius), &paint);    
}

fn get_fill_and_stroke(kind: UIPointType, selected: bool) -> (Color, Color) {
    let (fill, stroke) = if selected {
        match kind {
            UIPointType::GPHandle => (SELECTED_HANDLE_FILL, SELECTED_HANDLE_STROKE),
            UIPointType::Point((GPHandle::At(_, _), GPHandle::Colocated))
            | UIPointType::Point((GPHandle::Colocated, GPHandle::At(_, _))) => {
                (SELECTED_POINT_ONE_FILL, SELECTED_POINT_ONE_STROKE)
            }
            UIPointType::Direction => (SELECTED_DIRECTION_FILL, SELECTED_DIRECTION_STROKE),
            UIPointType::Point((GPHandle::Colocated, GPHandle::Colocated)) => {
                (SELECTED_POINT_SQUARE_FILL, SELECTED_POINT_SQUARE_STROKE)
            }
            _ => (SELECTED_POINT_TWO_FILL, SELECTED_POINT_TWO_STROKE),
        }
    } else {
        match kind {
            UIPointType::GPHandle => (HANDLE_FILL, HANDLE_STROKE),
            UIPointType::Point((GPHandle::At(_, _), GPHandle::Colocated))
            | UIPointType::Point((GPHandle::Colocated, GPHandle::At(_, _))) => {
                (POINT_ONE_FILL, POINT_ONE_STROKE)
            }
            UIPointType::Direction => (DIRECTION_FILL, DIRECTION_STROKE),
            UIPointType::Point((GPHandle::Colocated, GPHandle::Colocated)) => {
                (POINT_SQUARE_FILL, POINT_SQUARE_STROKE)
            }
            _ => (POINT_TWO_FILL, POINT_TWO_STROKE),
        }
    };
    (fill, stroke)
}

pub fn get_point_stroke_fill(round: bool, selected: bool) -> (Color, Color) {
    match round {
        true => match selected {
            true => (SELECTED_POINT_ONE_FILL, SELECTED_POINT_ONE_STROKE),
            false => (POINT_ONE_FILL, POINT_ONE_STROKE)
        },
        false => match selected {
            true => (SELECTED_POINT_SQUARE_FILL, SELECTED_POINT_SQUARE_STROKE),
            false => (POINT_SQUARE_FILL, POINT_SQUARE_STROKE),
        },
    }
}
pub fn draw_point<PD: GPPointData>(
    viewport: &Viewport,
    point: &dyn MFEKPointCommon<PD>,
    number: Option<isize>,
    selected: bool,
    canvas: &Canvas,
) {
    let factor = viewport.factor;
    let at = (point.x(), point.y());
    let original = at;
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::StrokeAndFill);
    let thiccness = POINT_STROKE_THICKNESS;
    paint.set_stroke_width(thiccness * (1. / factor));
    let radius = POINT_RADIUS * (1. / factor) * 1.75;

    let round = point.get_handle_position(WhichHandle::A).is_some() && point.get_handle_position(WhichHandle::B).is_some();
    let (stroke, fill) = get_point_stroke_fill(round, selected);
    if round {
        // Hack to differentiate between normal and smooth points. 
        // TODO: Change to an enum??
        if point.get_smooth().is_some_and(|s| s == true) {
            draw_round_point(at, radius, fill, 1., canvas, factor);
        } else {
            draw_cross_point(at, radius, fill, 1., canvas, factor);
        }
    } else {
        draw_square_point(at, radius * 1.25, fill, stroke, canvas, factor);
    }

    match number {
        None => {}
        Some(i) => match viewport.point_labels {
            PointLabels::None => {}
            PointLabels::Numbered => names::draw_point_number(viewport, at, i, canvas),
            PointLabels::Locations => names::draw_point_location(viewport, at, original, canvas),
        },
    }

    if let Some(name) = point.get_name().as_ref() {
        names::draw_point_name(viewport, at, stroke, name, canvas);
    }

    if viewport.handle_style != HandleStyle::None {
        if let Some(a) = point.get_handle(WhichHandle::A) {
            draw_handle::<PD>(viewport, a, selected, canvas);
        }
        if let Some(b) = point.get_handle(WhichHandle::B) {
            draw_handle::<PD>(viewport, b, selected, canvas);
        }
    }
}

pub fn get_handle_stroke_fill(selected: bool) -> (Color, Color) {
    match selected {
        true => (SELECTED_HANDLE_FILL, SELECTED_HANDLE_STROKE),
        false => (HANDLE_FILL, HANDLE_STROKE),
    }
}
fn draw_handle<PD: GPPointData>(
    viewport: &Viewport,
    h: GPHandle,
    selected: bool,
    canvas: &Canvas,
) {
    // if the handle is colocated there is nothing to draw
    if let GPHandle::At(x, y) = h {
        let at = (x, y);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_stroke_width(HANDLE_STROKE_THICKNESS * (1. / viewport.factor));
        let radius = POINT_RADIUS * (1. / viewport.factor);
        let (_fill, stroke) = get_handle_stroke_fill(selected);
        draw_round_point(at, radius, stroke, 0.5, canvas, viewport.factor);
    }
}

pub fn draw_handlebars<PD: GPPointData>(
    viewport: &Viewport,
    point: &dyn MFEKPointCommon<PD>,
    selected: bool,
    canvas: &Canvas,
) {
    let mut path = SkPath::new();
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_color(if selected {
        SELECTED_HANDLEBAR_STROKE
    } else {
        HANDLEBAR_STROKE
    });
    paint.set_stroke_width(HANDLEBAR_THICKNESS * (1. / viewport.factor));
    paint.set_style(PaintStyle::Stroke);

    paint.set_alpha_f(0.5);
    if let Some((x, y)) = point.get_handle_position(WhichHandle::A) {
        path.move_to((x, y));
        path.line_to((point.x(), point.y()));
    } else {
        path.move_to((point.x(), point.y()));
    }

    if let Some((x, y)) = point.get_handle_position(WhichHandle::B) {
        path.line_to((x, y));
    }

    canvas.draw_path(&path, &paint);
}

pub fn draw_complete_point<PD: GPPointData>(
    viewport: &Viewport,
    point: &dyn MFEKPointCommon<PD>,
    number: Option<isize>,
    selected: bool,
    canvas: &Canvas,
) {
    draw_point(
        viewport,
        point,
        number,
        selected,
        canvas,
    );
}

pub fn draw_all<PD: GPPointData>(
    glyph: &MFEKGlif<PD>,
    viewport: &Viewport,
    active_layer: usize,
    vcidx: Option<usize>,
    vpidx: Option<usize>,
    selected: &HashSet<(usize, usize)>,
    canvas: &Canvas,
    only_selected: bool,
) {
    let mut i: isize = -1;
    let handle_style = viewport.handle_style;
    let should_draw = |draw: bool| !only_selected || (only_selected && draw);

    for (lidx, layer) in glyph.layers.iter().enumerate() {
        if lidx != active_layer {
            continue;
        };
        if handle_style == HandleStyle::Handlebars {
            for (cidx, contour) in layer.outline.iter().enumerate() {

                for (pidx, point) in contour.iter().enumerate() {
                    let selected = if (lidx == active_layer && selected.contains(&(cidx, pidx)))
                        || (lidx == active_layer && vcidx == Some(cidx) && vpidx == Some(pidx))
                    {
                        true
                    } else {
                        false
                    };
                    if should_draw(selected) {
                        draw_handlebars(viewport, point, selected, canvas);
                    }
                }
            }
        }

        for (cidx, contour) in layer.outline.iter().enumerate() {
            if contour.is_open() {
                i += 1;
            }

            for (pidx, point) in contour.iter().enumerate() {
                if let Some(handle_b) = point.get_handle(WhichHandle::B) {
                    if handle_b != GPHandle::Colocated {
                        i += 1;
                    }
                }

                let selected = if (lidx == active_layer && selected.contains(&(cidx, pidx)))
                    || (lidx == active_layer && vcidx == Some(cidx) && vpidx == Some(pidx))
                {
                    true
                } else {
                    false
                };

                if should_draw(selected) {
                    draw_complete_point(viewport, point, Some(i), selected, canvas);
                }

                if let Some(handle_b) = point.get_handle(WhichHandle::B) {
                    if handle_b != GPHandle::Colocated {
                        i += 1;
                    }
                }
                i += 1;
            }
        }
    }
}
