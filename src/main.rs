use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols::DOT,
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

struct Term<'term, B: tui::backend::Backend> {
    terminal: &'term mut Terminal<B>,
    size: tui::layout::Rect,
}

impl<'term, B: tui::backend::Backend> Term<'term, B> {
    fn new(terminal: &'term mut Terminal<B>) -> Self {
        let size = terminal.size().unwrap();
        Self { terminal, size }
    }

    fn render_app<F>(&mut self, f: F) -> Result<(), io::Error>
    where
        F: Fn(&mut Frame<B>)
    {
        let terminal = &mut self.terminal;
        loop {
            terminal.draw(&f)?;

            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn multiplexer<B: tui::backend::Backend>(frame: &mut Frame<B>) {
    let rect_area = frame.size();
    let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT);
    frame.render_widget(tabs, rect_area);
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut term = Term::new(&mut terminal);

    term.render_app(multiplexer)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
