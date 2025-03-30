tui_rule::gen_example_code!(
    fn run(
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered().title_top(
                    Line::from(
                        crate::generate_gradient_text!(
                            "Vertical",
                            colorgrad::preset::warm()
                        ),
                    )
                    .centered(),
                );
                let rule_hor = Rule::from_set(
                    presets::test_sets::VERTICAL,
                ).vertical()
                .vertical_padding(1)
                .with_gradient(colorgrad::preset::warm());
                f.render_widget(rule_hor, f.area());
                f.render_widget(block, f.area());
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
