tui_rule::gen_example_code!(
    fn run(
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered().title_top(
                    Line::from(generate_gradient_text!(
                        "ASCII",
                        colorgrad::preset::warm()
                    ))
                    .centered(),
                );
                use presets::borders::plain::*;
                let marg =
                    ratatui::layout::Margin::new(0, 0);
                let right = Rule::from_set(RIGHT)
                    .area_margin(marg)
                    .horizontal_alignment(Alignment::Right)
                    .vertical();
                let left = Rule::from_set(LEFT)
                    .area_margin(marg)
                    .horizontal_alignment(Alignment::Left)
                    .vertical();
                let top = Rule::from_set(TOP).area_margin(marg).vertical_alignment(VerticalAlignment::Top);
                let bottom = Rule::from_set(BOTTOM).area_margin(marg).vertical_alignment(VerticalAlignment::Bottom);
                f.render_widget(right, f.area());
                f.render_widget(left, f.area());
                f.render_widget(top, f.area());
                f.render_widget(bottom, f.area());
            })?;
            let event = event::read()?;
            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            break Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
);
