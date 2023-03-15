//! Constants. This file should eventually become a config dotfile loaded & reloaded dynamically.
//! See issue #7 (GitHub).

use std::f32::consts::PI;

/* General purpose multiplier */
pub const PEN_SIZE: f32 = 1.0;

/* Sizes */
pub static OUTLINE_STROKE_THICKNESS: f32 = 1. * PEN_SIZE;
pub static GUIDELINE_THICKNESS: f32 = 2. * PEN_SIZE;
pub static GRID_THICKNESS: f32 = 2. * PEN_SIZE;
pub static POINT_STROKE_THICKNESS: f32 = 0. * PEN_SIZE;
/// TODO:  DIRECTION_STROKE_THICKNESS seems to overwrite POINT_STROKE_THICKNESS? - ELIH
pub static DIRECTION_STROKE_THICKNESS: f32 = 2. * PEN_SIZE;
pub static HANDLE_STROKE_THICKNESS: f32 = 2. * PEN_SIZE;
pub static POINT_RADIUS: f32 = 4. * PEN_SIZE;
/// Triangles aren't really points, but we internally treat them as such. They represent directions.
/// Also it's a factor, the area isn't *literally* six pixels even on DPI 1.0 :-)
pub static TRIANGLE_POINT_AREA: f32 = ((POINT_RADIUS + 1.) * 2.) * PEN_SIZE;
pub static HANDLE_RADIUS: f32 = 4. * PEN_SIZE;
pub static HANDLEBAR_THICKNESS: f32 = 2. * PEN_SIZE;
pub static ANCHOR_RADIUS: f32 = 12. * PEN_SIZE;
pub static ANCHOR_STROKE_THICKNESS: f32 = 2. * PEN_SIZE;
pub static POINT_LABEL_PADDING: f32 = 3. * PEN_SIZE;

/* Colors */
pub static OUTLINE_FILL: u32 = 0xff_606060;
pub static OUTLINE_STROKE: u32 = 0xff_101010;
pub static POINT_SQUARE_FILL: u32 = 0xff_00E040;
pub static POINT_SQUARE_STROKE: u32 = 0xff_00B050;
pub static DIRECTION_FILL: u32 = 0xff_00E040;
pub static DIRECTION_STROKE: u32 = 0xff_00B050;
// One and two refers to number of Bezier handles.
pub static POINT_ONE_FILL: u32 = 0xff_0000A0;
pub static POINT_ONE_STROKE: u32 = 0xff_2040FF;
pub static POINT_TWO_FILL: u32 = 0xff_579aff;
pub static POINT_TWO_STROKE: u32 = 0xff_2040FF;
pub static HANDLE_FILL: u32 = 0xff_FF10EE;
pub static HANDLE_STROKE: u32 = 0xff_7010FF;
pub static HANDLEBAR_STROKE: u32 = 0xff_707070;
pub static RIB_STROKE: u32 = 0xaa_ff7e7e;

pub struct Style {
     
}
// Can be reused below at your option.
pub static SELECTED_FILL: u32 = 0xff_FF8030;
pub static SELECTED_STROKE: u32 = 0xff_FF6000;
pub static SELECTED_TERTIARY: u32 = 0xff_FF8080;

pub static SELECTED_POINT_SQUARE_FILL: u32 = SELECTED_FILL;
pub static SELECTED_POINT_SQUARE_STROKE: u32 = SELECTED_TERTIARY;
pub static SELECTED_DIRECTION_FILL: u32 = SELECTED_FILL;
pub static SELECTED_DIRECTION_STROKE: u32 = SELECTED_TERTIARY;
// One and two refers to number of Bezier handles.
pub static SELECTED_POINT_ONE_FILL: u32 = SELECTED_FILL;
pub static SELECTED_POINT_ONE_STROKE: u32 = SELECTED_STROKE;
pub static SELECTED_POINT_TWO_FILL: u32 = SELECTED_STROKE;
pub static SELECTED_POINT_TWO_STROKE: u32 = SELECTED_TERTIARY;
pub static SELECTED_HANDLE_FILL: u32 = 0xff_FFD040;
pub static SELECTED_HANDLE_STROKE: u32 = 0xff_FFC040;
pub static SELECTED_HANDLEBAR_STROKE: u32 = SELECTED_HANDLE_STROKE;

pub static MEASURE_STROKE: u32 = 0xff_FF5050;

pub static GUIDELINE_STROKE: u32 = 0xff_7d7d7d;
pub static LBEARING_STROKE: u32 = 0xff_000000;
pub static RBEARING_STROKE: u32 = LBEARING_STROKE;
pub static BASELINE_STROKE: u32 = LBEARING_STROKE;
pub static GRID_STROKE: u32 = 0x99_007d7d;

pub static UFO_GUIDELINE_STROKE: u32 = 0xaa_7d0000;

pub static BACKGROUND_COLOR: u32 = 0xff_D0D0D0;
// "Paper" is the preview mode.
pub static PAPER_BGCOLOR: u32 = 0xff_F0F0F0;
// This is the automatic fill. Color (emoji) .glif's, when implemented, will ignore it, and can't
// be set here.
pub static PAPER_FILL: u32 = 0xff_101010;

pub static ANCHOR_FILL: u32 = 0xff_4080FF;
pub static ANCHOR_STROKE: u32 = 0xff_2040FF;

pub static POINT_NAME_DARKEN: u8 = 0x02;

// On-screen strings
pub static COMPONENT_NAME_COLOR: u32 = 0xff_444444;
pub static COMPONENT_NAME_BGCOLOR: u32 = 0x00_ffffff;
pub static DEFAULT_STRING_COLOR: u32 = 0xff_ff0000;
pub static DEFAULT_STRING_BGCOLOR: u32 = 0xaa_ffffff;
pub static ANCHOR_NAME_COLOR: u32 = 0xff_0000BB;
pub static ANCHOR_NAME_BGCOLOR: u32 = 0x00_ffffff;
pub static SELECTED_ANCHOR_COLOR: u32 = 0xff_00ffff;

/* Math */
pub const DEGREES_IN_RADIANS: f32 = PI / 180.0;
