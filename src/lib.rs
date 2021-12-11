//! Skia renderer.
#[macro_use] extern crate lazy_static;

pub mod constants;
pub mod guidelines;
pub mod points; // point drawing functions
pub mod toggles;
pub mod viewport;
// This imports calc_x, etc. which transforms coordinates between .glif and Skia
pub use points::calc::{calc_x, calc_y};
pub mod anchors;
pub mod glyph;
pub mod grid;
pub mod string;

use glifparser::outline::skia::SkiaPointTransforms;
pub static SKIA_POINT_TRANSFORMS: Option<SkiaPointTransforms> =
    Some(SkiaPointTransforms { calc_x, calc_y });
