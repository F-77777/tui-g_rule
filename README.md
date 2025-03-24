# tui-rule
#### A pretty rule widget using colorgrad
## Info: 
A terminal representation of html's `<hr>` tag in the form of a widget.  
Uses [colorgrad](https://crates.io/crates/colorgrad) for gradients.
## Features
- Custom start, end and center symbols.
- Colorgrad gradient support
- Horizontal and vertical alignment
- Horizontal and vertical rules
- Margins
#### Note:
- Rules are automatically fitted inside the provided area to avoid being rendered over blocks

#### Default values:

- `margin: Margin::new(0, 0)`
- `horizontal_alignment: Alignment::Center`
- `vertical_alignment: VerticalAlignment::Center`
- `gradient: None`


#### Horizontal rule with margin + gradient
```rust
    let block = Block::bordered();
    let rule = Rule::default()
        .with_gradient(colorgrad::preset::warm())
        .horizontal()
        .horizontal_margin(4);
    f.render_widget(rule, f.area());
    f.render_widget(block, f.area());
```
[![3I7QStt.md.png](https://iili.io/3I7QStt.md.png)](https://freeimage.host/i/3I7QStt)
#### Vertical rule with margin + gradient
```rust
    let block = Block::bordered();
    let rule = Rule::default()
        .with_gradient(colorgrad::preset::warm())
        .vertical()
        .vertical_margin(1);
    f.render_widget(rule, f.area());
    f.render_widget(block, f.area());
```
[![3IYDVLP.md.png](https://iili.io/3IYDVLP.md.png)](https://freeimage.host/i/3IYDVLP)

##### Rules without gradients are also supported
```rust
    let block = Block::bordered();
    let rule_vert = Rule::default()
        .horizontal_alignment(Alignment::Left)
        .vertical()
        .horizontal_margin(4)
        .vertical_margin(1);
    let rule_hor = Rule::default()
        .horizontal_margin(6);
    f.render_widget(rule_hor, f.area());
    f.render_widget(rule_vert, f.area());
    f.render_widget(block, f.area());
```
[![3IcrSUl.md.png](https://iili.io/3IcrSUl.md.png)](https://freeimage.host/i/3IcrSUl)

