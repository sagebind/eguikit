use egui::{vec2, Response, Sense, Ui, Widget};
use keyframe::{keyframes, AnimationSequence};

/// An animated spinner with indeterminate progress used to indicate
/// asynchronous loading.
///
/// By default spinners are drawn using the current text color.
pub struct Spinner {
    size: f32,
    style: Style,
}

/// Possible spinner styles.
///
/// All spinners are dynamically drawn and be scaled to any size.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Style {
    /// Dots lazily dance around.
    Dots,
}

impl Spinner {
    /// Set the size of the spinner in pixels.
    ///
    /// This sets the max width and height. Depending on the style, the spinner
    /// may be shorter or narrower.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spinner style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: 40.0,
            style: Style::Dots,
        }
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        match self.style {
            Style::Dots => {
                let (rect, response) =
                    ui.allocate_at_least(vec2(self.size, self.size / 2.0), Sense::hover());

                let big_radius = self.size / 6.0;
                let small_radius = big_radius / 2.0;
                let center = rect.center();
                let left = center - vec2(big_radius * 2.0, 0.0);
                let right = center + vec2(big_radius * 2.0, 0.0);

                let mut radius1 = keyframes![
                    (small_radius, 0.0),
                    (big_radius, 0.2),
                    (small_radius, 0.4),
                    (small_radius, 1.0)
                ];
                radius1.advance_by(ui.input().time % radius1.duration());

                let mut radius2 = keyframes![
                    (small_radius, 0.2),
                    (big_radius, 0.4),
                    (small_radius, 0.6),
                    (small_radius, 1.0)
                ];
                radius2.advance_by(ui.input().time % radius1.duration());

                let mut radius3 = keyframes![
                    (small_radius, 0.4),
                    (big_radius, 0.6),
                    (small_radius, 0.8),
                    (small_radius, 1.0)
                ];
                radius3.advance_by(ui.input().time % radius1.duration());

                ui.painter()
                    .circle_filled(left, radius1.now(), ui.visuals().text_color());
                ui.painter()
                    .circle_filled(center, radius2.now(), ui.visuals().text_color());
                ui.painter()
                    .circle_filled(right, radius3.now(), ui.visuals().text_color());

                ui.ctx().request_repaint();

                response
            }
        }
    }
}
