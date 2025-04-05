# TUI-RULE CHANGELOG
## Release 0.1.0 (First release)
### Features:
- Custom start, end and center symbols.
- Colorgrad gradient support
- Horizontal and vertical alignment
- Horizontal and vertical rules
- Horizontal and vertical margins
#### Default values:

- `margin: Margin::new(0, 0)`
- `horizontal_alignment: Alignment::Center`
- `vertical_alignment: VerticalAlignment::Center`
- `gradient: None`
---
## Release 0.1.1
### New Features:
- `Padding` as a replacement to `Margin`
  - More versatile (left, right, top, bottom) instead of margin's fixed horizontal and vertical
- `Bg` enum for text background types (bg applies to padding too)
- `area_margin` value to control the margin to wrap the rule in
- Rendering by reference
- Multiple symbol presets using the `Set` struct
  - 11 horizontal presets (most are braille patterns)
  - 1 vertical preset
  - Both horizontal and vertical have at least one `ASCII` variation
  - The two test presets used in the unit tests (test_sets mod)
- 2 Features 
  - `utils `
    - keeps only macros that are essential to the core logic
       - `generate_gradient_text`
         - Takes any type that can be converted into `Line` 
         - Returns `Vec<Span>`
         - Optional bg using the `Bg` enum
        - `create_segment`
          - Creates line segments using the `Set` struct 
          - Returns `String`
        - `create_raw_spans`
          - Takes either `&str` or `String` 
          - Returns `Vec<Span>` (raw)
   - `serde`
     - implements `serde::Serialize` and `serde::Deserialize` for enums and the `Set` struct
#### New Default values:
- `padding: Padding::new(0, 0, 0, 0),`
- `bg: Bg::None`,
- `extra_rep_1: 0,`
- `extra_rep_2: 0`,
- `area_margin: Margin::new(0, 0)`

## Release 0.1.2
### New Features:
- 17 new presets
  - 16 of them are border presets
    - 4 variations (`thick`, `rounded`, `plain`, `double`)
    - 4 presets for each variation (top, bottom, right, left)
  - 1 is an empty rule (all the characters are ' ')
- fixed the `area_margin` (2 spaces around if it was `Margin(0, 0)`)
