//! Constants. This file should eventually become a config dotfile loaded & reloaded dynamically.
//! See issue #7 (GitHub).

use std::f32::consts::PI;

/* Sizes */
pub static OUTLINE_STROKE_THICKNESS: f32 = 1.5 * PEN_SIZE;
pub static GUIDELINE_THICKNESS: f32 = OUTLINE_STROKE_THICKNESS;
pub static GRID_THICKNESS: f32 = GUIDELINE_THICKNESS / 2. * PEN_SIZE;
pub static POINT_STROKE_THICKNESS: f32 = 3. * PEN_SIZE;
pub static DIRECTION_STROKE_THICKNESS: f32 = 2. * PEN_SIZE;
pub static HANDLE_STROKE_THICKNESS: f32 = 2.5 * PEN_SIZE;
pub static POINT_RADIUS: f32 = 5. * PEN_SIZE;
/// Triangles aren't really points, but we internally treat them as such. They represent directions.
/// Also it's a factor, the area isn't *literally* six pixels even on DPI 1.0 :-)
pub static TRIANGLE_POINT_AREA: f32 = ((POINT_RADIUS + 1.) * 0.8) * PEN_SIZE;
pub static HANDLE_RADIUS: f32 = 2.5 * PEN_SIZE;
pub static HANDLEBAR_THICKNESS: f32 = 3. * PEN_SIZE;
pub static ANCHOR_RADIUS: f32 = 15.;
pub static ANCHOR_STROKE_THICKNESS: f32 = 3. * PEN_SIZE;

/* Colors */
pub static OUTLINE_FILL: u32 = 0xff_666666;
pub static OUTLINE_STROKE: u32 = 0xff_000000;
pub static POINT_SQUARE_FILL: u32 = 0xff_6ae755;
pub static POINT_SQUARE_STROKE: u32 = 0xff_208e53;
pub static DIRECTION_FILL: u32 = 0xff_a2e758;
pub static DIRECTION_STROKE: u32 = 0xff_5c8e23;
// One and two refers to number of Bezier handles.
pub static POINT_ONE_FILL: u32 = 0xff_44cf8c;
pub static POINT_ONE_STROKE: u32 = 0xff_1d8a84;
pub static POINT_TWO_FILL: u32 = 0xff_579aff;
pub static POINT_TWO_STROKE: u32 = 0xff_4428ec;
pub static HANDLE_FILL: u32 = 0xff_ff57ee;
pub static HANDLE_STROKE: u32 = 0xff_b928ec;
pub static HANDLEBAR_STROKE: u32 = 0xff_7e28ec;
pub static RIB_STROKE: u32 = 0xaa_ff7e7e;

pub static SELECTED_FILL: u32 = 0xff_ffed50;
pub static SELECTED_STROKE: u32 = 0xff_ffa115;
pub static SELECTED_TERTIARY: u32 = 0xff_e6830f;
pub static SELECTED_OFFCURVE: u32 = 0xff_e6e6e6;
pub static SELECTED_OFFCURVE_STROKE: u32 = 0xff_ffc829;

pub static MEASURE_STROKE: u32 = 0xff_ff5050;

pub static GUIDELINE_STROKE: u32 = 0xff_7d7d7d;
pub static LBEARING_STROKE: u32 = 0xff_000000;
pub static RBEARING_STROKE: u32 = LBEARING_STROKE;
pub static BASELINE_STROKE: u32 = LBEARING_STROKE;
pub static GRID_STROKE: u32 = 0x99_007d7d;

pub static UFO_GUIDELINE_STROKE: u32 = 0xaa_7d0000;

pub static BACKGROUND_COLOR: u32 = 0xff_c4c4c4;
// "Paper" is the preview mode.
pub static PAPER_BGCOLOR: u32 = 0xff_ffffff;
// This is the automatic fill. Color (emoji) .glif's, when implemented, will ignore it, and can't
// be set here.
pub static PAPER_FILL: u32 = 0xff_000000;

pub static ANCHOR_FILL: u32 = 0xff_0000ff;
pub static ANCHOR_STROKE: u32 = 0xff_000099;

// On-screen strings
pub static COMPONENT_NAME_COLOR: u32 = 0xff_444444;
pub static COMPONENT_NAME_BGCOLOR: u32 = 0x00_ffffff;
pub static DEFAULT_STRING_COLOR: u32 = 0xff_ff0000;
pub static DEFAULT_STRING_BGCOLOR: u32 = 0xaa_ffffff;
pub static ANCHOR_NAME_COLOR: u32 = 0xff_000099;
pub static ANCHOR_NAME_BGCOLOR: u32 = 0x00_ffffff;
pub static SELECTED_ANCHOR_COLOR: u32 = 0xff_00ffff;

/* Math */
pub const DEGREES_IN_RADIANS: f32 = PI / 180.0;

/* Misc. */
pub const PEN_SIZE: f32 = 1.0;
