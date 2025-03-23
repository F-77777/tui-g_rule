use colorgrad::Gradient;
use ratatui::{
    prelude::{Alignment, Buffer, Color, Margin, Rect, Span, Style},
    text::Line,
    widgets::Widget,
};
/// the Rule widget
/// # Allows:
///,ignore     - Vertical alignment
///     - Horizontal alignment
///     - Horizontal and vertical margins
///     - Center symbols
///     - Horizontal and vertical orientation
///     - Colorgrad gradients
///     - Start and end symbols
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
/// controls rule orientation
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Orientation {
    Vertical,
    Horizontal,
}
/// vertical version of the Alignment enum
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
/// macro for generating gradient text that returns a `Line` with the inputted gradient.
/// # Parameters
/// 1. any type that can be converted to Line (String, Line, &str)
/// 2. a colorgrad gradient (can be either Box<dyn Gradient> or an owned type)
/// ```rust,ignore
///     let gradient_text = generate_gradient_text!("Rainbow Text", colorgrad::preset::rainbow());
///     // displays "Rainbow Text" with a rainbow gradient
///     buf.set_line(1, 1, &gradient_text, gradient_text.width())
/// ```
#[macro_export]
macro_rules! generate_gradient_text {
    ($ln:expr, $gr:expr) => {{
        let ln: Line = $ln.into();
        let mut new_text = Vec::new();
        for (s, c) in ln.spans.clone().into_iter().zip($gr.colors(ln.width())) {
            new_text.push(s.style(Style::new().fg(Color::Rgb(
                (c.r * 255.0) as u8,
                (c.g * 255.0) as u8,
                (c.b * 255.0) as u8,
            ))));
        }
        Line::from(new_text)
    }};
}
impl Default for Rule {
    fn default() -> Self {
        Self::new().start('+').end('+').main_symbol('=').center('+')
    }
}
impl Rule {
    /// generates a new rule that looks like `━━━━━━━━━━━━━` with no gradient and no gradient
    /// centered horizontally and vertically by default
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
            horizontal_alignment: Alignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    }
    /// the new function and the with_gradient function combined
    /// ```rust,ignore
    ///     // displays a new rule with a rainbow gradient
    ///     Rule::new_with_gradient(colorgrad::preset::rainbow())
    /// ```
    pub fn new_with_gradient<G: Gradient + 'static>(gradient: G) -> Self {
        Self::new().with_gradient(gradient)
    }
    /// sets gradient for rule. uses colorgrad gradients
    /// ```rust,ignore
    ///     // displays `+=====+=====+` with a rainbow gradient
    ///     Rule::default().with_gradient(colorgrad::preset::rainbow())
    /// ```
    pub fn with_gradient<G: Gradient + 'static>(mut self, gradient: G) -> Self {
        self.gradient = Some(Box::<G>::new(gradient));
        self
    }
    /// sets the horizontal margin
    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        self.margin.horizontal = margin;
        self
    }
    /// sets the vertical margin
    pub fn vertical_margin(mut self, margin: u16) -> Self {
        self.margin.vertical = margin;
        self
    }
    /// makes the rule horizontal instead of vertical. Horizontal by default
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }
    /// makes the rule a vertical rule instead of horizontal
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }
    /// repeated symbol for right side
    /// ```rust,ignore
    ///     Rule::default().right_symbol('-')
    /// ```
    /// `+=====+-----+`
    pub fn right_symbol(mut self, symb: char) -> Self {
        self.right_symbol = symb;
        self
    }
    /// repeated symbol for left side
    /// ```rust,ignore
    ///     Rule::default().left_symbol('-')
    /// ```
    /// `+-----+=====+`
    pub fn left_symbol(mut self, symb: char) -> Self {
        self.left_symbol = symb;
        self
    }
    /// first symbol
    /// ```rust,ignore
    ///     Rule::default().start('%')
    /// ```
    /// `%=====+=====+`
    pub fn start(mut self, symb: char) -> Self {
        self.start = symb;
        self
    }
    /// last symbol
    ///```rust,ignore
    ///     Rule::default().end('%');
    ///```   
    /// `+=====+=====%`
    pub fn end(mut self, symb: char) -> Self {
        self.end = symb;
        self
    }
    /// center symbol  
    ///```rust,ignore
    ///     Rule::default().center('%')
    ///```
    /// `+=====%=====+`
    pub fn center(mut self, symb: char) -> Self {
        self.center = symb;
        self
    }
    /// the left_symbol and the right_symbol functions in one
    pub fn main_symbol(mut self, symb: char) -> Self {
        self = self.left_symbol(symb).right_symbol(symb);
        self
    }
    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }
    /// sets rule orientation
    /// ```rust,ignore
    ///     // creates a vertical rule
    ///     Rule::default().orientation(Orientation::Vertical)
    /// ```
    /// /// using the `.horizontal()` and `.vertical()` methods instead is recommended for simplicity
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    /// sets vertical alignment
    /// centered by default
    /// ```rust,ignore
    ///     // creates a rule thats vertically aligned to the top
    ///     Rule::default().vertical_alignment(VerticalAlignment::Top)
    /// ```
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
    // sets horizontal alignment
    pub fn horizontal_alignment(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }
}
impl Widget for Rule {
    fn render(self, mut area_old: Rect, buf: &mut Buffer) {
        if self.orientation == Orientation::Horizontal {
            area_old.y = match self.vertical_alignment {
                VerticalAlignment::Top => area_old.y + self.margin.vertical,
                VerticalAlignment::Center => {
                    (area_old.bottom() / 2).saturating_sub(1) + self.margin.vertical
                }
                VerticalAlignment::Bottom => {
                    area_old.bottom().saturating_sub(3 + self.margin.vertical)
                }
            }
        };
        if self.orientation == Orientation::Vertical {
            area_old.x = match self.horizontal_alignment {
                Alignment::Left => area_old.x + self.margin.horizontal,
                Alignment::Center => {
                    (area_old.right() / 2).saturating_sub(1) + self.margin.horizontal
                }
                Alignment::Right => area_old.right().saturating_sub(3 + self.margin.horizontal),
            }
        };
        let area = area_old.inner(Margin::new(1, 1));
        let rep_count: f32 = (match self.orientation {
            Orientation::Horizontal => area.width as f32,
            Orientation::Vertical => area.height as f32,
        } / 2.0)
            - 1.0;
        let margin = match self.orientation {
            Orientation::Vertical => self.margin.vertical,
            Orientation::Horizontal => self.margin.horizontal,
        } as usize;
        let left = self
            .left_symbol
            .to_string()
            .repeat((rep_count.floor() as usize).saturating_sub(margin));
        let right = self
            .right_symbol
            .to_string()
            .repeat((rep_count.round() as usize).saturating_sub(margin + 1));

        let mut ln = String::with_capacity(margin + 1 + left.len() + 1 + right.len() + 1 + margin);
        ln.push_str(&String::from(" ").repeat(match self.orientation {
            Orientation::Horizontal => {
                (match self.horizontal_alignment {
                    Alignment::Left => 0,
                    Alignment::Center => self.margin.horizontal,
                    Alignment::Right => self.margin.horizontal * 2,
                } as usize)
            }
            Orientation::Vertical => {
                (match self.vertical_alignment {
                    VerticalAlignment::Top => 0,
                    VerticalAlignment::Center => self.margin.vertical,
                    VerticalAlignment::Bottom => self.margin.vertical * 2,
                } as usize)
            }
        }));
        ln.push(self.start);
        ln.push_str(left.as_str());
        ln.push(self.center);
        ln.push_str(right.as_str());
        ln.push(self.end);
        ln.push_str(
            String::from(" ")
                .repeat(match self.orientation {
                    Orientation::Horizontal => {
                        (match self.horizontal_alignment {
                            Alignment::Left => self.margin.horizontal * 2,
                            Alignment::Center => self.margin.horizontal,
                            Alignment::Right => 0,
                        } as usize)
                    }
                    Orientation::Vertical => {
                        (match self.vertical_alignment {
                            VerticalAlignment::Top => self.margin.vertical * 2,
                            VerticalAlignment::Center => self.margin.vertical,
                            VerticalAlignment::Bottom => 0,
                        } as usize)
                    }
                })
                .as_str(),
        );
        macro_rules! create_raw_spans {
            ($string:expr) => {
                $string
                    .chars()
                    .map(String::from)
                    .map(Span::from)
                    .collect::<Vec<Span>>()
            };
        }
        let ln = if let Some(boxed) = &self.gradient {
            generate_gradient_text!(Line::from(create_raw_spans!(ln)), boxed)
        } else {
            Line::from(create_raw_spans!(ln))
        };
        match self.orientation {
            Orientation::Horizontal => {
                buf.set_line(area.x, area.y, &ln, ln.spans.len() as u16 + 1);
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
        let mut buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Horizontal Rule").centered())
            .title_bottom(Line::raw(" Vertical Alignment: Center ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .horizontal_margin(1)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│ +=====================+=====================+ │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "└───────── Vertical Alignment: Center ──────────┘",
            ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Horizontal Rule").centered())
            .title_bottom(Line::raw(" Vertical Alignment: Top ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .horizontal_margin(1)
            .vertical_alignment(VerticalAlignment::Top)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│ +=====================+=====================+ │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "└─────────── Vertical Alignment: Top ───────────┘",
            ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Horizontal Rule").centered())
            .title_bottom(Line::raw(" Vertical Alignment: Bottom ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .horizontal_margin(1)
            .vertical_alignment(VerticalAlignment::Bottom)
            .horizontal()
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
                "┌────────────────Horizontal Rule────────────────┐",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│                                               │",
                "│ +=====================+=====================+ │",
                "└───────── Vertical Alignment: Bottom ──────────┘",
            ]);
        assert_eq!(buffer, expected);
    }
    #[test]
    pub fn test_vr() {
        use super::*;
        let mut buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Vertical Rule").centered())
            .title_bottom(Line::raw(" Horizontal Alignment: Center ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .vertical()
            .vertical_margin(1)
            .main_symbol('│')
            .horizontal_alignment(Alignment::Center)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│                       +                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       +                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       │                       │",
            "│                       +                       │",
            "│                                               │",
            "└──────── Horizontal Alignment: Center ─────────┘",
        ]);
        assert_eq!(buffer, expected);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Vertical Rule").centered())
            .title_bottom(Line::raw(" Horizontal Alignment: Left ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .vertical()
            .vertical_margin(1)
            .main_symbol('│')
            .horizontal_alignment(Alignment::Left)
            .render(buffer.area, &mut buffer);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│+                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "│+                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "││                                              │",
            "│+                                              │",
            "│                                               │",
            "└───────── Horizontal Alignment: Left ──────────┘",
        ]);
        assert_eq!(buffer, expected);
        #[rustfmt::skip]
        let expected = Buffer::with_lines([
            "┌─────────────────Vertical Rule─────────────────┐",
            "│                                               │",
            "│                                              +│",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              +│",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              ││",
            "│                                              +│",
            "│                                               │",
            "└───────── Horizontal Alignment: Right ─────────┘",
        ]);
        buffer = Buffer::empty(Rect::new(0, 0, 49, 19));
        Block::bordered()
            .title_top(Line::raw("Vertical Rule").centered())
            .title_bottom(Line::raw(" Horizontal Alignment: Right ").centered())
            .render(buffer.area, &mut buffer);
        Rule::default()
            .vertical()
            .vertical_margin(1)
            .main_symbol('│')
            .horizontal_alignment(Alignment::Right)
            .render(buffer.area, &mut buffer);
        assert_eq!(buffer, expected);
    }
}
pub mod macros {
    #[macro_export]
    macro_rules! gen_main {
        () => {
            fn main() -> io::Result<()> {
                let mut terminal = ratatui::init();
                let app_result = run(&mut terminal);
                ratatui::restore();
                app_result
            }
        };
    }
    #[macro_export]
    macro_rules! gen_example_code {
        ($fun:item) => {
            tui_rule::gen_use!();
            tui_rule::gen_other_functions!();
            tui_rule::gen_run!($fun);
            tui_rule::gen_main!();
        };
    }
    #[macro_export]
    macro_rules! gen_run {
        ($fun:item) => {
            $fun
        };
    }
    #[macro_export]
    macro_rules! gen_other_functions {
        () => {
            fn handle_events() -> io::Result<()> {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        handle_key_event(key_event)
                    }
                    _ => Ok(()),
                }
            }

            fn handle_key_event(key_event: KeyEvent) -> io::Result<()> {
                match key_event.code {
                    KeyCode::Char('q') => {
                        let _ = ratatui::restore();
                        std::process::exit(0);
                    }
                    _ => Ok(()),
                }
            }
        };
    }
    #[macro_export]
    macro_rules! gen_use {
        () => {
            use colorgrad::Gradient;
            use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
            use ratatui::{
                buffer::Buffer,
                layout::Rect,
                prelude::{Alignment, Color, Style},
                text::Line,
                widgets::{Block, Widget},
                DefaultTerminal, Frame,
            };
            use std::{io, rc::Rc};
            use tui_rule::{Rule, VerticalAlignment};
        };
    }
}
