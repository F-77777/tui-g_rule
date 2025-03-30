# tui-rule, 
A pretty rule widget using colorgrad
## Info: 
A terminal representation of html's `<hr>` tag in the form of a widget.  
Uses [colorgrad](https://crates.io/crates/colorgrad) for gradients.
## Note:
- Rules are automatically fitted inside the provided area to avoid being rendered over blocks
#### See CHANGELOG.md for more info

### Horizontal rule with margin + gradient
```rust
    let block = Block::bordered();
    let rule = Rule::from_set(
            presets::test_sets::HORIZONTAL
        )
        .with_gradient(colorgrad::preset::warm())
        .horizontal_padding(4);
    f.render_widget(rule, f.area());
    f.render_widget(block, f.area());
```
![Horizontal Gradient](https://vhs.charm.sh/vhs-62MZQbra2ktFXds4rIK1wt.gif)
### Vertical rule with margin + gradient
```rust
    let block = Block::bordered();
    let rule = Rule::from_set(
            presets::test_sets::VERTICAL,
        )
        .with_gradient(colorgrad::preset::warm())
        .vertical()
        .vertical_padding(1);
        f.render_widget(rule, f.area());
        f.render_widget(block, f.area());
```
![Vertical Gradient](https://vhs.charm.sh/vhs-6AcaSwrZXBs0aBSSAxq0Qs.gif)

### Rules without gradients are also supported
```rust
    let block = Block::bordered();
    
    let rule_vert = Rule::from_set(
            presets::test_sets::VERTICAL
        )
        .horizontal_alignment(Alignment::Left)
        .vertical()
        .horizontal_padding(4)
        .vertical_padding(1);
    let rule_hor = Rule::from_set(
            presets::test_sets::HORIZONTAL
        )
        .horizontal_padding(6);
    f.render_widget(rule_hor, f.area());
    f.render_widget(rule_vert, f.area());
    f.render_widget(block, f.area());
```
![No gradient](https://vhs.charm.sh/vhs-7dfINmVfh15uF9JDQZcmTy.gif)
## See PRESET_EXAMPLES.md for more examples
