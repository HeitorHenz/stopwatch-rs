mod app;
mod cli;
mod stopwatch;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

use app::App;
use cli::Cli;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let tick = Duration::from_millis(cli.tick_ms);
    let mut app = App::new(cli);

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut app, tick);
    let _ = terminal.show_cursor();
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App, tick: Duration) -> io::Result<()> {
    while !app.should_quit() {
        let now = Instant::now();
        terminal.draw(|frame| ui::draw(frame, app, now))?;
        if event::poll(tick)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key, Instant::now())
                }
            }
        }
    }
    Ok(())
}
