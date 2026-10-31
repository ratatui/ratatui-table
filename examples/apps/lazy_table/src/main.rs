//! A log viewer over ten million entries.
//!
//! ```shell
//! cargo run -p lazy_table
//! ```
//!
//! Two things to look at:
//!
//! - **Rows are built on demand.** There is no `Vec` of entries. [`Table::lazy_rows`] calls the
//!   factory only for what is on screen, and the factory bumps the counter in the status line, so
//!   it shows exactly how many entries the last frame cost. Press `G` to jump to entry 9,999,999
//!   and the count stays at a screenful.
//! - **Heights are declared apart from the rows.** `ERROR` entries are three lines tall. Scrolling
//!   needs the height of entries it will never draw, and asking a lazy row for its height would
//!   mean building it — so [`Table::lazy_row_height_with`] answers from the index alone.
//!
//! The log itself is fake but fixed: every entry is worked out from its index, never stored and
//! never random, because the factory runs again on every frame an entry is visible.

use std::io::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};
use ratatui_table::{Cell, Row, Table, TableState};

/// Enough entries that collecting them up front would be the whole cost of the program.
const ENTRY_COUNT: usize = 10_000_000;

/// Entries are a quarter of a second apart, which puts the tail of the log about a month in.
const MILLIS_PER_ENTRY: usize = 250;

// Every seventeenth entry failed, every eleventh is a warning, every seventh is debug chatter.
// The three are coprime, so they never line up into a repeating block.
const ERROR_EVERY: usize = 17;
const WARN_EVERY: usize = 11;
const DEBUG_EVERY: usize = 7;

const SERVICES: [&str; 8] = [
    "kernel",
    "systemd",
    "sshd",
    "nginx",
    "postgres",
    "containerd",
    "kubelet",
    "chronyd",
];

const MESSAGES: [&str; 7] = [
    "accepted connection, {} open",
    "request completed in {}ms",
    "flushed {} pages to disk",
    "renewed lease, {}s remaining",
    "reaped {} idle workers",
    "checkpoint complete, {} segments recycled",
    "rotated log, {} MiB archived",
];

const FAILURES: [&str; 5] = [
    "connection reset by peer",
    "timed out waiting for lock after {}ms",
    "checksum mismatch on segment {}",
    "out of memory, killed worker {}",
    "write barrier failed on device {}",
];

const HEADER: [&str; 4] = ["Timestamp", "Level", "Service", "Message"];

const HELP: &str = " ↑/↓ move  PgUp/PgDn page  g/G oldest/newest  q quit ";

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    state: TableState,
    /// Bumped once per entry the factory actually builds, and reset before every frame.
    built: Arc<AtomicUsize>,
}

impl App {
    fn new() -> Self {
        Self {
            state: TableState::new().with_selected(Some(0)),
            built: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => self.state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => self.state.select_previous(),
                    KeyCode::PageDown => self.jump(20),
                    KeyCode::PageUp => self.jump(-20),
                    KeyCode::Char('g') | KeyCode::Home => self.state.select_first(),
                    KeyCode::Char('G') | KeyCode::End => self.state.select(Some(ENTRY_COUNT - 1)),
                    _ => {}
                }
                // The navigation methods do not know how many entries there are.
                let selected = self.state.selected().unwrap_or(0).min(ENTRY_COUNT - 1);
                self.state.select(Some(selected));
            }
        }
    }

    fn jump(&mut self, amount: isize) {
        let selected = self.state.selected().unwrap_or(0);
        self.state
            .select(Some(selected.saturating_add_signed(amount)));
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [table_area, status_area, help_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        self.built.store(0, Ordering::Relaxed);
        let table = self.table();
        frame.render_stateful_widget(table, table_area, &mut self.state);

        // Rendered after the table so the count belongs to the frame the viewer is looking at.
        frame.render_widget(self.status_line(), status_area);
        frame.render_widget(Line::from(HELP).dim(), help_area);
    }

    fn table(&self) -> Table<'static> {
        let widths = [
            Constraint::Length(14),
            Constraint::Length(5),
            Constraint::Length(11),
            Constraint::Fill(1),
        ];
        let built = Arc::clone(&self.built);

        Table::lazy_rows(ENTRY_COUNT, widths, move |index| {
            built.fetch_add(1, Ordering::Relaxed);
            entry(index)
        })
        // Heights are declared separately because scrolling needs the geometry of entries that
        // are never drawn. Only the index is needed here, so no row gets built.
        .lazy_row_height_with(|index| if is_error(index) { 3 } else { 1 })
        .header(Row::new(HEADER).style(Style::new().bold()))
        .block(Block::bordered().title(" journal — 10,000,000 entries "))
        .row_highlight_style(Style::new().bg(Color::Indexed(17)))
        .highlight_symbol(">> ")
    }

    fn status_line(&self) -> Line<'static> {
        let selected = self.state.selected().unwrap_or(0);
        Line::from(vec![
            " entry ".into(),
            thousands(selected).bold(),
            " of ".into(),
            thousands(ENTRY_COUNT).bold(),
            "  │  offset ".into(),
            thousands(self.state.offset()).bold(),
            "  │  rows built this frame ".into(),
            thousands(self.built.load(Ordering::Relaxed))
                .bold()
                .fg(Color::Indexed(214)),
        ])
    }
}

/// Whether an entry failed. Kept separate from [`entry`] because the table asks for the height of
/// entries it never draws, and an `ERROR` is three lines tall.
const fn is_error(index: usize) -> bool {
    index.is_multiple_of(ERROR_EVERY)
}

/// The level column: a label and the colour to print it in.
const fn level(index: usize) -> (&'static str, Color) {
    if is_error(index) {
        ("ERROR", Color::Indexed(203))
    } else if index.is_multiple_of(WARN_EVERY) {
        ("WARN", Color::Indexed(214))
    } else if index.is_multiple_of(DEBUG_EVERY) {
        ("DEBUG", Color::DarkGray)
    } else {
        ("INFO", Color::Indexed(39))
    }
}

/// One log entry. This is the whole dataset: a function of the index, stored nowhere.
fn entry(index: usize) -> Row<'static> {
    let (label, colour) = level(index);

    // Seeded from the index, so an entry reads the same every time it is drawn. The factory runs
    // again on every frame the entry is on screen, so an unseeded generator would flicker.
    let mut rng = StdRng::seed_from_u64(index as u64);
    let service = SERVICES[rng.random_range(0..SERVICES.len())];
    let template = if is_error(index) {
        FAILURES[rng.random_range(0..FAILURES.len())]
    } else {
        MESSAGES[rng.random_range(0..MESSAGES.len())]
    };
    let message = template.replace("{}", &rng.random_range(12..912).to_string());

    let body = if is_error(index) {
        Text::from(vec![
            Line::from(message),
            Line::from(format!("    code E{:04}", rng.random_range(0..10_000))).dim(),
            Line::from(format!(
                "    at src/{service}/handler.rs:{}",
                rng.random_range(20..820)
            ))
            .dim(),
        ])
    } else {
        Text::from(message)
    };

    Row::new([
        Cell::from(timestamp(index).dim()),
        Cell::from(Span::from(label).fg(colour)),
        Cell::from(Span::from(service).fg(Color::Indexed(108))),
        Cell::from(body),
    ])
}

/// Monotonic since start of capture, the way `dmesg` prints it: `[  12345.750]`.
fn timestamp(index: usize) -> Span<'static> {
    let millis = index * MILLIS_PER_ENTRY;
    Span::from(format!("[{:>7}.{:03}]", millis / 1000, millis % 1000))
}

/// `1234567` renders as `1,234,567`, so the entry count reads as the point of the example.
fn thousands(value: usize) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (position, digit) in digits.chars().enumerate() {
        if position > 0 && (digits.len() - position).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}
