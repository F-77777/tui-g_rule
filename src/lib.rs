use colorgrad::Gradient;
use VerticalAlignent as VAlignment;
use ratatui::{
    prelude::{Alignment, Buffer, Color, Rect, Span, Style},
    text::Line,
    widgets::Widget,
};
pub struct Rule {
    pub gradient: Option<Box<dyn Gradient>>,
    pub start: char,
    pub end: char,
    pub center: char,
    pub left_center: char,
    pub right_center: char,
    pub right_symbol: char,
    pub left_symbol: char,
    pub orientation: Orientation,
    pub left_margin: u16,
    pub right_margin: u16,
    pub top_margin: u16,
    pub bottom_margin: u16,
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
    ($ln:expr, $gr:expr) => {
        $ln.spans
            .iter()
            .zip($gr.colors($ln.width()))
            .map(|(span, color)| {
                span.clone().style(Style::new().fg(Color::Rgb(
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                )))
            })
            .collect()
    };
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
            right_center: '━',
            left_center: '━',
            left_symbol: '━',
            right_symbol: '━',
            top_margin: 0,
            left_margin: 0,
            bottom_margin: 0,
            right_margin: 0,
            orientation: Orientation::Horizontal,
            horizontal_alignment: Alignment::Left,
            vertical_alignment: VAlignment::Top,
        }
    }
    pub fn new_with_gradient<G: Gradient + 'static>(gradient: G) -> Self {
        Self::new().with_gradient(gradient)
    }
    pub fn with_gradient<G: Gradient + 'static>(mut self, gradient: G) -> Self {
        self.gradient = Some(Box::<G>::new(gradient));
        self
    }
    pub fn horizontal(mut self)
 -> Self {
     self.orientation = Orientation::Horizontal;
     self
 }
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
    pub fn main_symbol(mut self, symb: char) -> Self {
        self = self
            .left_symbol(symb)
            .right_symbol(symb)
            .left_center(symb)
            .right_center(symb);
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
    ```rust
        Rule::default().end('%');
    ```
    `+=====+=====%`
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
    /// center symbol for right side
    /// ```rust
    /// Rule::default().right_center(' ')
    /// ``` `+=====+== ==+`
    pub fn right_center(mut self, symb: char) -> Self {
        self.right_center = symb;
        self
    }

    pub fn left_center(mut self, symb: char) -> Self {
        self.left_center = symb;
        self
    }
    /// the right_center, center, and left_center functions all in one
    pub fn main_center(mut self, symb: char) -> Self {
        self = self.right_center(symb).left_center(symb).center(symb);
        self
    }
    pub fn left_margin(mut self, margin: u16) -> Self {
        self.left_margin = margin;
        self
    }
    pub fn right_margin(mut self, margin: u16) -> Self {
        self.right_margin = margin;
        self
    }
    pub fn top_margin(mut self, margin: u16) -> Self {
        self.top_margin = margin;
        self
    }
    pub fn bottom_margin(mut self, margin: u16) -> Self {
        self.bottom_margin = margin;
        self
    }
    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        self = self.left_margin(margin).right_margin(margin);
        self
    }
    pub fn vertical_margin(mut self, margin: u16) -> Self {
        self = self.top_margin(margin).bottom_margin(margin);
        self
    }
    pub fn margin(mut self, margin: u16) -> Self {
        self = self
            .left_margin(margin)
            .right_margin(margin)
            .top_margin(margin)
            .bottom_margin(margin);
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect {
            width: area
                .width
                .saturating_sub(self.left_margin + self.right_margin + 2),
            height: area
                .height
                .saturating_sub(self.top_margin + self.bottom_margin + 2),
            x: match self.horizontal_alignment {
                Alignment::Left => area.x,
                Alignment::Center => area.right() / 2,
                Alignment::Right => area.right().saturating_sub(1)
            } + self.left_margin.saturating_sub(self.right_margin()) + 1,
            y: match self.vertical_alignment {
                VAlignment::Top => area.y
                VAlignment::Center => area.bottom() / 2,
                VAlignment::Bottom => area.bottom().saturating_sub(1)
            } + self.top_margin.saturating_sub(self.bottom_margin()) + 1,
        };

        let rep_count = (match self.orientation {
            Orientation::Horizontal => area.width,
            Orientation::Vertical => area.height,
        } / 4)
            .saturating_sub(1) as usize;
        let left = self.left_symbol.to_string().repeat(rep_count);
        let right = self.right_symbol.to_string().repeat(rep_count);

        let mut ln = String::with_capacity(
            1 + left.len() + 1 + left.len() + 1 + right.len() + 1 + right.len() + 1,
        );

        ln.push(self.start);
        ln.push_str(&left);
        ln.push(self.left_center);
        ln.push_str(&left);
        ln.push(self.center);
        ln.push_str(&right);
        ln.push(self.right_center);
        ln.push_str(&right);
        ln.push(self.end);
        let ln = if let Some(boxed) = &self.gradient {
            generate_gradient_text!(Line::from(ln.clone()), boxed)
        } else {
            ln.chars()
                .map(|s| Span::from(String::from(s)))
                .collect::<Vec<Span>>()
        };
        match self.orientation {
            Orientation::Horizontal => {
                buf.set_line(area.x, area.y, &Line::from(ln.clone()), ln.len() as u16 + 1);
            }
            Orientation::Vertical => {
                for (y, s) in ln.iter().enumerate() {
                    buf.set_span(area.x, area.y + y as u16, s, 1);
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
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 41));
        Block::bordered()
            .title_top(Line::raw("T").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .vertical_alignment(VerticalAlignment::Bottom)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────T────────────────┐",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│                                  │",
            "│+================+===============+│",
            "│                                  │",
            "└──────────────────────────────────┘"]);
        assert_eq!(buffer, expected);
    }
    #[test]
    pub fn test_vr() {
        use super::*;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 20));
        Block::bordered()
            .title_top(Line::raw("T").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .orientation(Orientation::Vertical)
            .main_symbol('│')
            .top_margin(1)
            .horizontal_margin(1)
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
