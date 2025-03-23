tui_rule::gen_example_code!(
    fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let block = Block::bordered();
                let rule = Rule::default()
                    .with_gradient(colorgrad::preset::warm())
		    .vertical()
                    .vertical_margin(1);
                f.render_widget(rule, f.area());
                f.render_widget(block, f.area());
            })?;
            if let Err(e) = handle_events() {
                eprintln!("Error handling events: {}", e);
            }
        }
    }
);
