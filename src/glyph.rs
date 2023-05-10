use super::constants::*;

use crate::viewport::Viewport;
use crate::{string::UiString, toggles::PreviewMode};

use glifparser::outline::skia::ToSkiaPaths;
use glifparser::{glif::LayerOperation, MFEKGlif, PointData};
use skia_safe::{Path, Canvas, Color4f, Paint, PaintStyle, PathOp, Rect, Color};

pub struct Style {
    outline_fill: Color,
    paper_fill: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self { 
            outline_fill: OUTLINE_FILL.into(),
            paper_fill: PAPER_FILL.into(),
        }
    }
}

impl Style {
    pub fn new(outline_fill: Color, paper_fill: Color) -> Self {
        Self {
            outline_fill,
            paper_fill
        }
    }
}

pub fn draw_components<PD: PointData>(
    glyph: &MFEKGlif<PD>,
    viewport: &Viewport,
    canvas: &mut Canvas,
) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(OUTLINE_STROKE);
    paint.set_style(PaintStyle::Stroke);
    let mut path = Path::new();
    for rect in glyph.component_rects.as_ref().unwrap() {
        let skrect = Rect::new(rect.minx, rect.miny, rect.maxx, rect.maxy);
        let uis = UiString::with_colors(
            &rect.name,
            COMPONENT_NAME_COLOR,
            Some(COMPONENT_NAME_BGCOLOR),
        );
        uis.draw(viewport, (rect.minx, rect.maxy), canvas);
        path.add_rect(skrect, None);
    }
    let skpaths = glyph.flattened.as_ref().map(|f| f.to_skia_paths(None));
    if let Some(skp) = skpaths.map(|skp| skp.combined()) {
        canvas.draw_path(&skp, &paint);
    } else {
        log::error!("Failed to draw components!");
    }
}

pub fn draw_layer_group(
    viewport: &Viewport,
    canvas: &mut Canvas,
    root_color: Option<Color4f>,
    open_path: &Path,
    closed_path: &Path,
    outline_path: &Path,
    style: &Style,
) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);

    if viewport.preview_mode == PreviewMode::Paper {
        paint.set_style(PaintStyle::Fill);
    } else if viewport.preview_mode == PreviewMode::None {
        paint.set_style(PaintStyle::Stroke);
    } else {
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_color(style.outline_fill);
        paint.set_stroke_width(OUTLINE_STROKE_THICKNESS * (1. / viewport.factor));
    }

    if let Some(color) = root_color {
        paint.set_color4f(color, None);
    } else if viewport.preview_mode == PreviewMode::Paper {
        paint.set_color(style.paper_fill);
    }

    canvas.draw_path(&closed_path, &paint);

    paint.set_style(PaintStyle::Stroke);
    canvas.draw_path(&open_path, &paint);

    if viewport.preview_mode != PreviewMode::Paper {
        paint.set_style(PaintStyle::Stroke);

        if root_color.is_none() {
            paint.set_color(style.outline_fill);
        }
        canvas.draw_path(&closed_path, &paint);
        canvas.draw_path(&outline_path, &paint);
    }
}

//TODO: pub use crate::events::vws;
// Before we draw we've got to build a flattened path out of the glyph by resolving
// each layer operation in turn.
pub fn draw<PD: PointData>(canvas: &mut Canvas, glyph: &MFEKGlif<PD>, viewport: &Viewport, style: Option<Style>) {
    let style = if style.is_none() { Style::default() } else { style.unwrap() };
    let mut total_open_path = Path::new();
    let mut total_closed_path = Path::new();
    let mut total_outline_path = Path::new();
    let mut root_color = if let Some(color) = glyph.layers[0].color {
        Some(Color4f::new(
            color.r.into(),
            color.g.into(),
            color.b.into(),
            color.a.into(),
        ))
    } else {
        None
    };

    for (layer_idx, layer) in glyph.layers.iter().enumerate() {
        if !layer.visible {
            continue;
        }

        if layer.operation.is_none() && layer_idx != 0 {
            draw_layer_group(
                viewport,
                canvas,
                root_color,
                &total_open_path,
                &total_closed_path,
                &total_outline_path,
                &style
            );

            total_open_path = Path::new();
            total_closed_path = Path::new();
            total_outline_path = Path::new();

            root_color = if let Some(color) = layer.color {
                Some(Color4f::new(
                    color.r.into(),
                    color.g.into(),
                    color.b.into(),
                    color.a.into(),
                ))
            } else {
                None
            };
        }

        let skpaths = layer.outline.to_skia_paths(None);

        if let Some(op) = &layer.operation {
            let pathop = match op {
                LayerOperation::Difference => PathOp::Difference,
                LayerOperation::Union => PathOp::Union,
                LayerOperation::Intersect => PathOp::Intersect,
                LayerOperation::XOR => PathOp::XOR,
            };

            if let Some(open) = skpaths.open {
                total_open_path.add_path(
                    &open,
                    (0., 0.),
                    skia_safe::path::AddPathMode::Append,
                );
            }

            if let Some(closed) = skpaths.closed {
                total_outline_path.add_path(
                    &closed,
                    (0., 0.),
                    skia_safe::path::AddPathMode::Append,
                );
                if let Some(result) = total_closed_path.op(&closed, pathop) {
                    total_closed_path = Path::new();
                    total_closed_path.add_path(
                        &result.as_winding().unwrap(),
                        (0., 0.),
                        skia_safe::path::AddPathMode::Append,
                    );
                } else {
                    total_closed_path.add_path(
                        &closed,
                        (0., 0.),
                        skia_safe::path::AddPathMode::Append,
                    );
                }
            }
        } else {
            if let Some(open) = skpaths.open {
                total_open_path.add_path(
                    &open,
                    (0., 0.),
                    skia_safe::path::AddPathMode::Append,
                );
            }

            if let Some(closed) = skpaths.closed {
                total_outline_path.add_path(
                    &closed,
                    (0., 0.),
                    skia_safe::path::AddPathMode::Append,
                );
                total_closed_path.add_path(
                    &closed,
                    (0., 0.),
                    skia_safe::path::AddPathMode::Append,
                );
            }
        }
    }

    draw_layer_group(
        viewport,
        canvas,
        root_color,
        &total_open_path,
        &total_closed_path,
        &total_outline_path,
        &style
    );
}
