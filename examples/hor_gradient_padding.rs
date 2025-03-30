tui_rule::gen_example_code!(
    fn run(
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered();
                let rule = Rule::from_set(
                    presets::test_sets::HORIZONTAL,
                )
                .with_gradient(colorgrad::preset::warm())
                .horizontal_padding(4);
                f.render_widget(rule, f.area());
                f.render_widget(block, f.area());
            })?;
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {
                    if let KeyCode::Char('q') =
                        key_event.code
                    {
                        break Ok(());
                    }
                }
            }
        }
    }
);
