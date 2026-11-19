use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Running,
    Paused,
}

pub struct Stopwatch {
    state: State,
    accumulated: Duration,
    start_time: Option<Instant>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            accumulated: Duration::ZERO,
            start_time: None,
        }
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn start(&mut self, now: Instant) {
        if self.state != State::Running {
            self.state = State::Running;
            self.start_time = Some(now);
        }
    }
    pub fn pause(&mut self, now: Instant) {
        if self.state == State::Running {
            self.accumulated = self.elapsed_at(now);
            self.start_time = None;
            self.state = State::Paused;
        }
    }

    pub fn toggle(&mut self, now: Instant) {
        match self.state {
            State::Running => self.pause(now),
            State::Paused | State::Idle => self.start(now),
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Idle;
        self.accumulated = Duration::ZERO;
        self.start_time = None;
    }
    pub fn elapsed_at(&self, now: Instant) -> Duration {
        match self.start_time {
            Some(start_time) => self.accumulated + now.saturating_duration_since(start_time),
            None => self.accumulated,
        }
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}
