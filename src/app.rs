use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::cli::{Cli, Precision};
use crate::stopwatch::{State, Stopwatch};

pub struct App {
    stopwatch: Stopwatch,
    precision: Precision,
    should_quit: bool,
    laps: Vec<Lap>,
}

#[derive(Debug, Clone, Copy)]
pub struct Lap {
    pub total: Duration,
    pub split: Duration,
}

impl App {
    pub fn new(cli: Cli) -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            precision: cli.precision,
            laps: Vec::new(),
            should_quit: false,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn state(&self) -> State {
        self.stopwatch.state()
    }

    pub fn elapsed(&self, now: Instant) -> Duration {
        self.stopwatch.elapsed_at(now)
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }

    pub fn lap(&mut self, now: Instant) {
        let total = self.elapsed(now);
        let previous_total = self.laps.last().map_or(Duration::ZERO, |lap| lap.total);
        let split = total.saturating_sub(previous_total);

        self.laps.push(Lap { total, split });
    }

    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.laps.clear()
    }

    pub fn laps(&self) -> &[Lap] {
        &self.laps
    }

    pub fn on_key(&mut self, key: KeyEvent, now: Instant) {
        match key.code {
            KeyCode::Char(' ') => self.stopwatch.toggle(now),
            KeyCode::Char('r') | KeyCode::Char('R') => self.reset(),
            KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
            KeyCode::Char('l') | KeyCode::Char('L') => self.lap(now),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
