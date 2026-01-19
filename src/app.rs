use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

pub fn main_rat(output: String) -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app(terminal, output);
    ratatui::restore();
    Ok(result?)
}

fn app(mut terminal: DefaultTerminal, output: String) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &output))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, output: &str) {
    frame.render_widget(Paragraph::new(output), frame.area());
}
