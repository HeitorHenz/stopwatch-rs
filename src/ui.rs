use std::time::{Duration, Instant};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;
use crate::cli::Precision;
use crate::stopwatch::State;

pub fn draw(frame: &mut Frame, app: &App, now: Instant) {
    let area = frame.area();

    const HELP: &str = "space: pause  l: lap  r: reset  q: quit";

    let [main_area, help_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(2)]).areas(area);

    if main_area.width < 20 || main_area.height < 9 {
        frame.render_widget(
            Paragraph::new("Terminal size is too small. Please resize.")
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }),
            main_area,
        );
        return;
    }

    frame.render_widget(
        Paragraph::new(HELP)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::DarkGray)),
        help_area,
    );

    let [stopwatch_area, laps_area] =
        Layout::vertical([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)]).areas(main_area);

    let [status_area, timer_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(3)]).areas(stopwatch_area);

    let (status, color) = match app.state() {
        State::Running => ("RUNNING", Color::Green),
        State::Paused => ("PAUSED", Color::Yellow),
        State::Idle => ("IDLE", Color::DarkGray),
    };

    frame.render_widget(
        Paragraph::new(status)
            .alignment(Alignment::Center)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        status_area,
    );

    let time = format_duration(app.elapsed(now), app.precision());

    let block = Block::bordered().title("Stopwatch");
    let inner = block.inner(timer_area);
    frame.render_widget(block, timer_area);

    let [clock_area] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::Center)
        .areas(inner);

    frame.render_widget(
        Paragraph::new(time)
            .alignment(Alignment::Center)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        clock_area,
    );

    let visible_rows = laps_area.height.saturating_sub(2) as usize;

    let items: Vec<ListItem> = app
        .laps()
        .iter()
        .enumerate()
        .rev()
        .take(visible_rows)
        .map(|(index, lap)| {
            let split = format_duration(lap.split, app.precision());
            let total = format_duration(lap.total, app.precision());
            ListItem::new(format!(
                "Lap {:>2}   split {split:<10}   total {total:<10}",
                index + 1
            ))
        })
        .collect();

    let laps_block = Block::bordered().title(" Laps ");

    if app.laps().is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "Press l to record a lap",
                Style::default().fg(Color::DarkGray),
            )))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(laps_block),
            laps_area,
        );
    } else {
        frame.render_widget(List::new(items).block(laps_block), laps_area);
    }
}

fn format_duration(elapsed: Duration, precision: Precision) -> String {
    let total_secs = elapsed.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = elapsed.subsec_millis();

    let clock = if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    };

    match precision {
        Precision::Seconds => clock,
        Precision::Centis => {
            let centis = millis / 10;
            format!("{clock}.{centis:02}")
        }
        Precision::Millis => format!("{clock}.{millis:03}"),
    }
}
