tui_rule::gen_example_code!(
    fn run(
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered();
                let rule_vert = Rule::from_set(
                    presets::test_sets::VERTICAL,
                )
                .horizontal_alignment(Alignment::Left)
                .vertical()
                .horizontal_padding(4)
                .vertical_padding(1);
                let rule_hor = Rule::from_set(
                    presets::test_sets::HORIZONTAL,
                )
                .horizontal_padding(6);
                f.render_widget(rule_hor, f.area());
                f.render_widget(rule_vert, f.area());
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
