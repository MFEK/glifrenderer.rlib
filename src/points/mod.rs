use glifparser::glif::Layer;
use glifparser::outline::skia::ToSkiaPath;
use skulpin::skia_safe::{
    Canvas, ContourMeasureIter, Matrix, Paint, PaintStyle, Path as SkPath, Point as SkPoint,
    Rect as SkRect, Vector,
};
use std::collections::HashSet;

pub mod calc;
pub mod names;

use self::calc::*;
use super::constants::*;
use crate::toggles::{HandleStyle, PointLabels};
use crate::viewport::Viewport;
use crate::SKIA_POINT_TRANSFORMS;

use glifparser::{
    Handle as GPHandle, MFEKGlif, Point as GPPoint, PointData as GPPointData,
    PointType as GPPointType,
};

type Color = u32;

#[derive(Clone, Copy, PartialEq)]
pub enum UIPointType {
    Point((GPHandle, GPHandle)),
    GPHandle,
    #[allow(unused)]
    Anchor,
    Direction,
}

trait SkiaFromGlyph<PD: GPPointData> {
    fn from_glif(p: &GPPoint<PD>) -> SkPoint;
}

impl<PD: GPPointData> SkiaFromGlyph<PD> for SkPoint {
    fn from_glif(p: &GPPoint<PD>) -> Self {
        SkPoint::from((calc_x(p.x), calc_y(p.y)))
    }
}

fn get_fill_and_stroke(kind: UIPointType, selected: bool) -> (Color, Color) {
    let (fill, stroke) = if selected {
        match kind {
            UIPointType::GPHandle => (SELECTED_OFFCURVE, SELECTED_OFFCURVE_STROKE),
            UIPointType::Point((GPHandle::At(_, _), GPHandle::Colocated))
            | UIPointType::Point((GPHandle::Colocated, GPHandle::At(_, _))) => {
                (SELECTED_STROKE, SELECTED_TERTIARY)
            }
            UIPointType::Point((GPHandle::Colocated, GPHandle::Colocated))
            | UIPointType::Direction => (SELECTED_STROKE, SELECTED_TERTIARY),
            _ => (SELECTED_FILL, SELECTED_STROKE),
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

pub fn draw_directions<PD: GPPointData>(viewport: &Viewport, layer: &Layer<PD>, canvas: &mut Canvas) {
    for c in &layer.outline {
        drop(
            c.inner
                .to_skia_path(SKIA_POINT_TRANSFORMS)
                .as_ref()
                .map(|p| {
                    let piter = ContourMeasureIter::from_path(p, false, None);
                    for cm in piter {
                        // Get vector and tangent -4 Skia units along the contur
                        let (vec, tan) = cm.pos_tan(-4.).unwrap();
                        draw_triangle_point(viewport, vec, tan, false, canvas);
                    }
                }),
        );
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
    canvas: &mut Canvas,
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
    kind: UIPointType,
    selected: bool,
    canvas: &mut Canvas,
    paint: &mut Paint,
    factor: f32,
) {
    let (fill, stroke) = get_fill_and_stroke(kind, selected);
    let factor = factor;
    let radius = POINT_RADIUS
        * (1. / factor)
        * if kind != UIPointType::GPHandle && selected {
            1.75
        } else {
            1.
        };
    paint.set_color(fill);
    canvas.draw_circle((at.0, at.1), radius, &paint);
    paint.set_style(PaintStyle::Stroke);
    paint.set_color(stroke);
    canvas.draw_circle((at.0, at.1), radius, &paint);
}

pub fn draw_square_point(
    at: (f32, f32),
    kind: UIPointType,
    selected: bool,
    canvas: &mut Canvas,
    paint: &mut Paint,
    factor: f32,
) {
    let (fill, stroke) = get_fill_and_stroke(kind, selected);
    let radius = (POINT_RADIUS * (1. / factor)) * 2. * if selected { 1.75 } else { 1. };

    let mut path = SkPath::new();
    paint.set_color(fill);
    path.add_rect(
        SkRect::from_point_and_size((at.0 - radius / 2., at.1 - radius / 2.), (radius, radius)),
        None,
    );
    path.close();
    canvas.draw_path(&path, &paint);
    paint.set_color(stroke);
    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&path, &paint);
}

pub fn draw_point(
    viewport: &Viewport,
    at: (f32, f32),
    original: (f32, f32),
    number: Option<isize>,
    kind: UIPointType,
    selected: bool,
    canvas: &mut Canvas,
) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::StrokeAndFill);
    let thiccness = if kind == UIPointType::GPHandle {
        HANDLE_STROKE_THICKNESS
    } else {
        POINT_STROKE_THICKNESS
    };
    paint.set_stroke_width(thiccness * (1. / viewport.factor));
    let _radius = if kind == UIPointType::GPHandle {
        HANDLE_RADIUS
    } else {
        POINT_RADIUS
    } * (1. / viewport.factor);

    match kind {
        UIPointType::GPHandle | UIPointType::Point((GPHandle::At(_, _), GPHandle::At(_, _))) => {
            draw_round_point(at, kind, selected, canvas, &mut paint, viewport.factor);
        }
        UIPointType::Point(_) => {
            draw_square_point(at, kind, selected, canvas, &mut paint, viewport.factor);
        }
        _ => {}
    }

    match number {
        None => {}
        Some(i) => match viewport.point_labels {
            PointLabels::None => {}
            PointLabels::Numbered => names::draw_point_number(viewport, at, i, canvas),
            PointLabels::Locations => names::draw_point_location(viewport, at, original, canvas),
        },
    }

    if let UIPointType::Point((a, b)) = kind {
        if viewport.handle_style != HandleStyle::None {
            draw_handle(viewport, a, selected, canvas);
            draw_handle(viewport, b, selected, canvas);
        }
    }
}

fn draw_handle(viewport: &Viewport, h: GPHandle, selected: bool, canvas: &mut Canvas) {
    match h {
        GPHandle::Colocated => {}
        GPHandle::At(x, y) => {
            draw_point(
                viewport,
                (calc_x(x), calc_y(y)),
                (x, y),
                None,
                UIPointType::GPHandle,
                selected,
                canvas,
            );
        }
    }
}

pub fn draw_handlebars<PD: GPPointData>(
    viewport: &Viewport,
    prevpoint: Option<&GPPoint<PD>>, // None in cubic mode when selecting as no access to prevpoints
    point: &GPPoint<PD>,
    selected: bool,
    canvas: &mut Canvas,
) {
    let mut path = SkPath::new();
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_color(if selected {
        SELECTED_FILL
    } else {
        HANDLEBAR_STROKE
    });
    paint.set_stroke_width(HANDLEBAR_THICKNESS * (1. / viewport.factor));
    paint.set_style(PaintStyle::Stroke);

    match point.a {
        GPHandle::At(x, y) => {
            path.move_to((calc_x(x), calc_y(y)));
            path.line_to((calc_x(point.x), calc_y(point.y)));
        }
        _ => {
            path.move_to((calc_x(point.x), calc_y(point.y)));
        }
    }
    match point.b {
        GPHandle::At(x, y) => {
            path.line_to((calc_x(x), calc_y(y)));
        }
        _ => {}
    }
    if point.ptype == GPPointType::QCurve || point.ptype == GPPointType::QClose {
        if let Some(pp) = prevpoint {
            match pp.a {
                GPHandle::At(x, y) => {
                    path.line_to((calc_x(x), calc_y(y)));
                }
                _ => {}
            }
        }
    }
    canvas.draw_path(&path, &paint);
}

pub fn draw_complete_point<PD: GPPointData>(
    viewport: &Viewport,
    point: &GPPoint<PD>,
    number: Option<isize>,
    selected: bool,
    canvas: &mut Canvas,
) {
    if point.ptype == GPPointType::QClose {
        return;
    }

    draw_point(
        viewport,
        (calc_x(point.x), calc_y(point.y)),
        (point.x, point.y),
        number,
        UIPointType::Point((point.a, point.b)),
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
    canvas: &mut Canvas,
) {
    let mut i: isize = -1;
    let handle_style = viewport.handle_style;

    for (lidx, layer) in glyph.layers.iter().enumerate() {
        if lidx != active_layer {
            continue;
        };
        if handle_style == HandleStyle::Handlebars {
            for (cidx, contour) in layer.outline.iter().enumerate() {
                let mut prevpoint = contour.inner.first().unwrap();
                for (pidx, point) in contour.inner.iter().enumerate() {
                    let selected = if (lidx == active_layer && selected.contains(&(cidx, pidx)))
                        || (lidx == active_layer && vcidx == Some(cidx) && vpidx == Some(pidx))
                    {
                        true
                    } else {
                        false
                    };
                    draw_handlebars(viewport, Some(prevpoint), &point, selected, canvas);
                    prevpoint = &point;
                }
            }
        }

        for (cidx, contour) in layer.outline.iter().enumerate() {
            for (pidx, point) in contour.inner.iter().enumerate() {
                if point.b != GPHandle::Colocated {
                    i += 1;
                }
                let selected = if (lidx == active_layer && selected.contains(&(cidx, pidx)))
                    || (lidx == active_layer && vcidx == Some(cidx) && vpidx == Some(pidx))
                {
                    true
                } else {
                    false
                };

                draw_complete_point(viewport, &point, Some(i), selected, canvas);
                if point.a != GPHandle::Colocated {
                    i += 1;
                }
                i += 1;
            }
        }
    }
}
