//! A package index of 200,000 rows that the app builds once and borrows on every frame.
//!
//! ```shell
//! cargo run -p borrowed_rows
//! ```
//!
//! The rows and the header live in `App`. Each frame, [`Table::from`] borrows the rows and
//! [`Row::from`] borrows the header, so drawing copies nothing. Press `c` to clone the rows into
//! the table instead, the way `Table::new` needs them, and watch the draw time in the status line
//! go from microseconds to tens of milliseconds.

use std::io::Result;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};
use ratatui_table::{Row, Table, TableState};

const PACKAGE_COUNT: usize = 200_000;

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(8),
    Constraint::Fill(1),
    Constraint::Length(9),
    Constraint::Length(14),
];

const HELP: &str = " ↑/↓ move  PgUp/PgDn page  g/G first/last  c borrow or clone  q quit ";

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    rows: Vec<Row<'static>>,
    header: Row<'static>,
    state: TableState,
    clone_rows: bool,
    last_draw: Duration,
}

impl App {
    fn new() -> Self {
        Self {
            rows: (0..PACKAGE_COUNT).map(package).collect(),
            header: Row::new(["#", "Package", "Version", "Downloads"]).style(Style::new().bold()),
            state: TableState::new().with_selected(Some(0)),
            clone_rows: false,
            last_draw: Duration::ZERO,
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
                    KeyCode::PageDown => self.state.scroll_down_by(20),
                    KeyCode::PageUp => self.state.scroll_up_by(20),
                    KeyCode::Char('g') | KeyCode::Home => self.state.select_first(),
                    KeyCode::Char('G') | KeyCode::End => self.state.select_last(),
                    KeyCode::Char('c') => self.clone_rows = !self.clone_rows,
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [table_area, status_area, help_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let start = Instant::now();
        let rows = if self.clone_rows {
            Table::new(self.rows.clone(), WIDTHS)
        } else {
            Table::from(&self.rows).widths(WIDTHS)
        };
        let table = rows
            .header(Row::from(&self.header))
            .block(Block::bordered().title(" packages, 200,000 rows "))
            .row_highlight_style(Style::new().bg(Color::Indexed(17)))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(table, table_area, &mut self.state);
        self.last_draw = start.elapsed();

        frame.render_widget(self.status_line(), status_area);
        frame.render_widget(Line::from(HELP).dim(), help_area);
    }

    fn status_line(&self) -> Line<'static> {
        let (mode, colour) = if self.clone_rows {
            ("cloned into the table", Color::Indexed(203))
        } else {
            ("borrowed from app state", Color::Indexed(114))
        };
        Line::from(vec![
            " rows ".into(),
            mode.bold().fg(colour),
            "  │  last draw ".into(),
            format!("{:.1?}", self.last_draw)
                .bold()
                .fg(Color::Indexed(214)),
        ])
    }
}

/// One package, worked out from its index so the example needs no data file.
fn package(index: usize) -> Row<'static> {
    const SYLLABLES: [&str; 8] = ["ra", "ta", "tui", "core", "term", "grid", "span", "cell"];
    let name = format!(
        "{}{}-{}",
        SYLLABLES[index % 8],
        SYLLABLES[index / 8 % 8],
        index / 64
    );
    let version = format!("{}.{}.{}", index % 3, index % 17, index % 29);
    let downloads = (index * 7919) % 9_999_991;
    Row::new([index.to_string(), name, version, downloads.to_string()])
}
