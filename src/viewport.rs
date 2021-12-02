use crate::toggles::{HandleStyle, PointLabels, PreviewMode};
use skulpin::skia_safe::{Canvas, Matrix};

/// This structure represents the current viewport without requiring a reference to the Skia
/// canvas.
#[derive(Clone, Debug)]
pub struct Viewport {
    pub winsize: (f32, f32),
    pub factor: f32,
    pub offset: (f32, f32),
    //pub dpi: f64,
    pub matrix: Matrix,
    pub point_labels: PointLabels,
    pub handle_style: HandleStyle,
    pub preview_mode: PreviewMode,
    /// We need a recalculated offset due to an unavoidable OS WM event that made our state
    /// untrustworthy. (Maximize, resize, etc.)
    /// You should check this flag in your render loop and clear it if set. If set, call rebuild()
    /// before relying on this Viewport.
    broken: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            winsize: (0., 0.),
            factor: 1.,
            offset: (0., 0.),
            //dpi: 1.,
            broken: false,
            matrix: Matrix::new_identity(),
            point_labels: PointLabels::None,
            preview_mode: PreviewMode::None,
            handle_style: HandleStyle::Handlebars,
        }
    }
}

// this impl is for flags & boilerplate
impl Viewport {
    pub fn with_winsize(mut self, winsize: (f32, f32)) -> Self {
        self.winsize = winsize;
        self
    }
    pub fn with_factor(mut self, factor: f32) -> Self {
        self.factor = factor;
        self
    }
    pub fn with_offset(mut self, offset: (f32, f32)) -> Self {
        self.offset = offset;
        self
    }
    /*pub fn with_dpi(mut self, dpi: f64) -> Self {
        self.dpi = dpi;
        self
    }*/
    pub fn with_point_labels(mut self, point_labels: PointLabels) -> Self {
        self.point_labels = point_labels;
        self
    }
    pub fn with_handle_style(mut self, handle_style: HandleStyle) -> Self {
        self.handle_style = handle_style;
        self
    }
    pub fn with_preview_mode(mut self, preview_mode: PreviewMode) -> Self {
        self.preview_mode = preview_mode;
        self
    }
    pub fn is_broken(&self) -> bool {
        self.broken
    }
}

// this impl contains the important stuff
impl Viewport {
    /// It may be useful to tag this viewport for rebuilding from the OS, just based on what Skia
    /// knows about the canvas and its matrix, but sometimes you may not have the canvas in scope—
    /// especially during window resize events. This allows you to tag the canvas as needed to be
    /// rebuilt next frame / next use if not used in all frames. Use sparingly, for API logic, not
    /// unsettable once set w/o going through ``Viewport::redraw``.
    pub fn set_broken_flag(&mut self) {
        self.broken = true;
    }
    pub fn refresh_from_backing_canvas(&mut self, canvas: &Canvas) -> Result<Matrix, ()> {
        let matrix = canvas.local_to_device_as_3x3(); // used to be total_matrix()
        debug_assert!(!matrix.has_perspective());
        debug_assert!(matrix.rect_stays_rect());
        debug_assert!(matrix.is_finite());
        let offset = matrix.map_xy(self.offset.0, self.offset.1);
        self.offset = (offset.x, offset.y);
        log::trace!("Matrix before refresh was {:?}…", &matrix);
        self.rebuild(None);
        log::trace!("And after ’twas {:?}.", &self.matrix);
        let new_matrix = canvas.local_to_device_as_3x3(); // used to be total_matrix()
        new_matrix
            .invert()
            .map(|inm| Ok(Matrix::concat(&matrix, &inm)))
            .unwrap_or(Err(()))
    }
    pub fn as_device_matrix(&self) -> Matrix {
        *(Matrix::default().set_scale_translate(
            (self.factor, -self.factor),
            (self.offset.0, self.winsize.1 - -self.offset.1),
        ))
    }
    fn rebuild(&mut self, matrix: Option<Matrix>) {
        let dmatrix = self.as_device_matrix();
        let matrix = Matrix::concat(
            &if let Some(m) = matrix {
                m
            } else {
                Matrix::new_identity()
            },
            &dmatrix,
        );
        self.matrix = matrix;
        self.broken = false;
    }
    pub fn redraw(&mut self, canvas: &mut Canvas) {
        if self.broken {
            let diff = self.refresh_from_backing_canvas(canvas);
            log::debug!(
                "Refreshed canvas’s matrix from backing store in redraw(…), diff was {:?}",
                diff
            );
            match diff {
                Ok(diff) => debug_assert_eq!(&diff, Matrix::i()),
                Err(()) => log::error!("Failed to refresh viewport from backing canvas, viewport will reset!"),
            }
        }
        self.rebuild(None);
        canvas.set_matrix(&self.matrix.into());
    }
}
