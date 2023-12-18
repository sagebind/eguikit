use egui::{vec2, Rect, Response, Sense, Ui, Widget};
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Style {
    /// Dots lazily dance around.
    Dots,

    /// Classic pulsing vertical bars.
    Bars,

    Squares,
}

impl Spinner {
    const BAR_COUNT: usize = 5;

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
        let color = ui.visuals().text_color();

        ui.ctx().request_repaint();

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
                radius1.advance_by(ui.input(|i| i.time) % radius1.duration());

                let mut radius2 = keyframes![
                    (small_radius, 0.2),
                    (big_radius, 0.4),
                    (small_radius, 0.6),
                    (small_radius, 1.0)
                ];
                radius2.advance_by(ui.input(|i| i.time) % radius1.duration());

                let mut radius3 = keyframes![
                    (small_radius, 0.4),
                    (big_radius, 0.6),
                    (small_radius, 0.8),
                    (small_radius, 1.0)
                ];
                radius3.advance_by(ui.input(|i| i.time) % radius1.duration());

                ui.painter().circle_filled(left, radius1.now(), color);
                ui.painter().circle_filled(center, radius2.now(), color);
                ui.painter().circle_filled(right, radius3.now(), color);

                response
            }

            Style::Bars => {
                let (rect, response) =
                    ui.allocate_at_least(vec2(self.size, self.size), Sense::hover());

                let bar_count = Self::BAR_COUNT;
                let bar_width = self.size / bar_count as f32;

                for i in 0..bar_count {
                    let mut animation = keyframes![
                        (self.size * 0.3, 0.0, keyframe::functions::EaseInOut),
                        (self.size * 0.3, 0.2, keyframe::functions::EaseInOut),
                        (self.size * 0.7, 0.5, keyframe::functions::EaseInOut),
                        (self.size * 0.3, 0.8, keyframe::functions::EaseInOut),
                        (self.size * 0.3, 1.0)
                    ];

                    animation.advance_and_maybe_wrap(0.1 * (bar_count - i) as f64);
                    animation.advance_and_maybe_wrap(ui.input(|i| i.time) % animation.duration());

                    let center =
                        rect.left_center() + vec2(bar_width * i as f32 + bar_width / 2.0, 0.0);

                    ui.painter().rect_filled(
                        Rect::from_center_size(center, vec2(bar_width * 0.5, animation.now())),
                        2.0,
                        color,
                    );
                }

                response
            }

            Style::Squares => {
                let (rect, response) =
                    ui.allocate_at_least(vec2(self.size, self.size), Sense::hover());

                let square_size = self.size / 3.0;

                let positions = vec![
                    rect.min,
                    rect.min + vec2(square_size, 0.0),
                    rect.min + vec2(square_size * 2.0, 0.0),
                    rect.min + vec2(square_size * 2.0, square_size),
                    rect.min + vec2(square_size * 2.0, square_size * 2.0),
                    rect.min + vec2(square_size, square_size * 2.0),
                    rect.min + vec2(0.0, square_size * 2.0),
                    rect.min + vec2(0.0, square_size),
                ];

                let idx = (ui.input(|i| i.time) * 15.0 % positions.len() as f64) as usize;

                for i in 0..5 {
                    let position = (idx + i) % positions.len();
                    let square =
                        Rect::from_min_size(positions[position], vec2(square_size, square_size));
                    let color = color.linear_multiply(i as f32 / 5.0);

                    ui.painter().rect_filled(square, 0.0, color);
                }

                response
            }
        }
    }
}
