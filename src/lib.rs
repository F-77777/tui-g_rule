use colorgrad::Gradient;
use ratatui::{
    prelude::{Alignment, Margin, Buffer, Color, Rect, Span, Style},
    text::Line,
    widgets::Widget,
};
pub struct Rule {
    pub gradient: Option<Box<dyn Gradient>>,
    pub start: char,
    pub end: char,
    pub center: char,
    pub right_symbol: char,
    pub left_symbol: char,
    pub orientation: Orientation,
    pub margin: Margin,
    pub vertical_alignment: VerticalAlignment,
    pub horizontal_alignment: Alignment,
}
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Orientation {
    Vertical,
    Horizontal,
}
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
#[macro_export]
macro_rules! generate_gradient_text {
    ($ln:expr, $gr:expr) => {{
        let mut new_text = Vec::new();
        for (s, c) in $ln.spans.into_iter().zip($gr.colors($ln.width())) {
            new_text.push(s.style(Style::new().fg(Color::Rgb(
                (c.r * 255.0) as u8,
                (c.g * 255.0) as u8,
                (c.b * 255.0) as u8,
            ))));
        }
        new_text
    }};
}
impl Default for Rule {
    fn default() -> Self {
        Self::new().start('+').end('+').main_symbol('=').center('+')
    }
}
impl Rule {
    pub fn new() -> Self {
        Self {
            gradient: None,
            start: '━',
            end: '━',
            center: '━',
            left_symbol: '━',
            right_symbol: '━',
            margin: Margin::new(0, 0),
            orientation: Orientation::Horizontal,
            horizontal_alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
    pub fn new_with_gradient<G: Gradient + 'static>(gradient: G) -> Self {
        Self::new().with_gradient(gradient)
    }
    pub fn with_gradient<G: Gradient + 'static>(mut self, gradient: G) -> Self {
        self.gradient = Some(Box::<G>::new(gradient));
        self
    }
    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        self.margin.horizontal = margin;
        self
    }
    pub fn vertical_margin(mut self, margin: u16) -> Self {
        self.margin.vertical = margin;
        self
    }
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
    pub fn right_symbol(mut self, symb: char) -> Self {
        self.right_symbol = symb;
        self
    }
    pub fn left_symbol(mut self, symb: char) -> Self {
        self.left_symbol = symb;
        self
    }
    /// first symbol
    pub fn start(mut self, symb: char) -> Self {
        self.start = symb;
        self
    }
    /// last symbol
    ///```rust
    ///     Rule::default().end('%');
    /// ```
    /// `+=====+=====%`

    pub fn end(mut self, symb: char) -> Self {
        self.end = symb;
        self
    }
    /// center symbol  
    /// if it was set to ' ': `+===== =====+`
    pub fn center(mut self, symb: char) -> Self {
        self.center = symb;
        self
    }
    pub fn main_symbol(mut self, symb: char) -> Self {
        self = self
            .left_symbol(symb)
            .right_symbol(symb);
        self
    }
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
    pub fn horizontal_alignment(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }
}
impl Widget for Rule {
    fn render(self, mut area_old: Rect, buf: &mut Buffer) {
        area_old.y = match self.vertical_alignment {
            VerticalAlignment::Top => area_old.y,
            VerticalAlignment::Center => area_old.bottom() / 2,
            VerticalAlignment::Bottom => area_old.bottom().saturating_sub(3),
        };
        let area = area_old.inner(Margin::new(1, 1));
        let rep_count: f32 = (match self.orientation {
            Orientation::Horizontal => area.width as f32,
            Orientation::Vertical => area.height as f32,
        } / 2.0) - 2.0;
        let space_rep_count = match self.orientation {
            Orientation::Horizontal => self.margin.horizontal,
            Orientation::Vertical => self.margin.vertical,
        } as usize;
        let ln = format!(
            "{}{}{}{}{}{}{}",
            String::from(" ").repeat(match self.orientation {
                Orientation::Horizontal => (
                    match self.horizontal_alignment {
                        Alignment::Left => 0,
                        Alignment::Center => self.margin.horizontal,
                        Alignment::Right => self.margin.horizontal * 2,
                    } as usize
                ),
                Orientation::Vertical => (
                    match self.vertical_alignment {
                        VerticalAlignment::Top => self.margin.vertical * 2,
                        VerticalAlignment::Center => self.margin.vertical,
                        VerticalAlignment::Bottom => 0,
                    } as usize
                ),
            }),
            self.start,
            self.left_symbol
                .to_string()
                .repeat((rep_count.floor() as usize).saturating_sub(space_rep_count / 2)),
            self.center,
            self.right_symbol
                .to_string()
                .repeat(((rep_count.round() as usize).saturating_sub(1)).saturating_sub(space_rep_count / 2)),
            self.end,
            String::from(" ").repeat(match self.orientation {
                Orientation::Horizontal => (
                    match self.horizontal_alignment {
                        Alignment::Left => self.margin.horizontal * 2,
                        Alignment::Center => self.margin.horizontal,
                        Alignment::Right => 0,
                    } as usize
                ),
                Orientation::Vertical => (
                    match self.vertical_alignment {
                        VerticalAlignment::Top => 0,
                        VerticalAlignment::Center => self.margin.vertical,
                        VerticalAlignment::Bottom => self.margin.vertical * 2,
                    } as usize
                ),
            }),
        );
        macro_rules! create_raw_spans {
            ($string:expr) => {
                $string
                    .chars().map(String::from)
                    .map(Span::from)
                    .collect::<Vec<Span>>()
            }
        }
        let ln = if let Some(boxed) = &self.gradient {
            generate_gradient_text!(Line::from(create_raw_spans!(ln)), boxed)
        } else {
            create_raw_spans!(ln)
        };
        match self.orientation {
            Orientation::Horizontal => {
                buf.set_line(
                    area.x,
                    area.y,
                    &Line::from(ln.clone()),
                    ln.len() as u16 + 1,
                );
            }
            Orientation::Vertical => {
                for (y_n, s) in ln.iter().enumerate() {
                    buf.set_span(area.x, area.y + y_n as u16, s, 1);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use ratatui::widgets::Block;
    #[test]
    pub fn test_hr() {
        use super::*;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 48, 19));
        Block::bordered()
            .title_top(Line::raw("Horizontal").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .horizontal_margin(1)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(Alignment::Right)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Horizontal──────────────────┐",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│+=====================+=====================+│",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "│                                             │",
            "└─────────────────────────────────────────────┘",
            ]);
        assert_eq!(buffer, expected);
        buffer.reset();
    }
    #[test]
    pub fn test_vr() {
        use super::*;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 48, 19));
        Block::bordered()
            .title_top(Line::raw("T").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .orientation(Orientation::Vertical)
            .main_symbol('│')
            .horizontal_alignment(Alignment::Center)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌────T────┐",
            "│    +    │",
            "│    │    │",
            "│    │    │",
            "│    +    │",
            "│    │    │",
            "│    │    │",
            "│    +    │",
            "└─────────┘"]);
        assert_eq!(buffer, expected);
    }
}
