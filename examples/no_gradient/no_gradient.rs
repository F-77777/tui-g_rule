tui_rule::gen_example_code!(
    fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered();
                let rule_vert = Rule::default().horizontal_alignment(Alignment::Left)
                    .vertical().horizontal_margin(4)
                    .vertical_margin(1);
		let rule_hor = Rule::default().horizontal_margin(6);
                f.render_widget(rule_hor, f.area());
		f.render_widget(rule_vert, f.area());
                f.render_widget(block, f.area());
            })?;
            if let Err(e) = handle_events() {
                eprintln!("Error handling events: {}", e);
            }
        }
    }
);
