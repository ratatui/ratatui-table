//! Lines around and between the cells.
//!
//! ```shell
//! cargo run -p borders
//! ```
//!
//! `b` cycles through presets of [`TableBorders`], `h` toggles the row lines, and `s` cycles
//! through the glyph sets. `-` and `+` change the column spacing, which moves the inner vertical
//! lines within the gap.

use std::io::Result;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::line;
use ratatui::text::Line;
use ratatui::{DefaultTerminal, Frame};
use ratatui_table::{Cell, Row, Table, TableBorders, TableState};

const HEADER: [&str; 4] = ["Name", "Role", "City", "Joined"];

const ITEMS: [[&str; 4]; 8] = [
    ["Alice", "Engineer", "Berlin", "2019"],
    ["Bob", "Designer", "Lisbon", "2020"],
    ["Carol", "Engineer", "Toronto", "2020"],
    ["Dan", "Support", "Nairobi", "2021"],
    ["Erin", "Engineer", "Osaka", "2021"],
    ["Frank", "Product", "Bogota", "2022"],
    ["Grace", "Engineer", "Helsinki", "2023"],
    ["Heidi", "Research", "Dunedin", "2024"],
];

const PRESETS: [(&str, TableBorders); 6] = [
    ("ALL", TableBorders::ALL),
    (
        "OUTER | INNER_VERTICAL | HEADER_SEPARATOR",
        TableBorders::OUTER
            .union(TableBorders::INNER_VERTICAL)
            .union(TableBorders::HEADER_SEPARATOR),
    ),
    (
        "INNER_VERTICAL | HEADER_SEPARATOR",
        TableBorders::INNER_VERTICAL.union(TableBorders::HEADER_SEPARATOR),
    ),
    ("HEADER_SEPARATOR", TableBorders::HEADER_SEPARATOR),
    ("INNER_VERTICAL", TableBorders::INNER_VERTICAL),
    ("none", TableBorders::empty()),
];

const SETS: [(&str, line::Set); 4] = [
    ("NORMAL", line::NORMAL),
    ("ROUNDED", line::ROUNDED),
    ("DOUBLE", line::DOUBLE),
    ("THICK", line::THICK),
];

const HELP: &str = " ↑/↓ row │ b borders  h row lines  s set  -/+ spacing │ q quit ";

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    state: TableState,
    preset: usize,
    set: usize,
    toggled: TableBorders,
    column_spacing: u16,
}

impl App {
    fn new() -> Self {
        Self {
            state: TableState::new().with_selected(1),
            preset: 0,
            set: 0,
            toggled: TableBorders::empty(),
            column_spacing: 1,
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
                    KeyCode::Char('b') => self.preset = (self.preset + 1) % PRESETS.len(),
                    KeyCode::Char('s') => self.set = (self.set + 1) % SETS.len(),
                    KeyCode::Char('h') => self.toggled.toggle(TableBorders::INNER_HORIZONTAL),
                    KeyCode::Char('-') => {
                        self.column_spacing = self.column_spacing.saturating_sub(1);
                    }
                    KeyCode::Char('+' | '=') => {
                        self.column_spacing = (self.column_spacing + 1).min(5);
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [config_area, table_area, help_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        frame.render_widget(self.config_line(), config_area);
        let table = self.table();
        frame.render_stateful_widget(table, table_area, &mut self.state);
        frame.render_widget(Line::from(HELP).dim(), help_area);
    }

    const fn borders(&self) -> TableBorders {
        PRESETS[self.preset].1.symmetric_difference(self.toggled)
    }

    fn table(&self) -> Table<'static> {
        let rows = ITEMS.iter().map(|item| Row::new(item.iter().copied()));
        let widths = [
            Constraint::Fill(2),
            Constraint::Fill(2),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ];
        let footer = Cell::new(format!("{} people", ITEMS.len())).column_span(4);
        Table::new(rows, widths)
            .header(Row::new(HEADER).style(Style::new().bold()))
            .footer(Row::new([footer]).style(Style::new().italic()))
            .column_spacing(self.column_spacing)
            .borders(self.borders())
            .border_set(SETS[self.set].1)
            .border_style(Style::new().fg(Color::DarkGray))
            .row_highlight_style(Style::new().bg(Color::Indexed(17)))
            .highlight_symbol(">>")
    }

    fn config_line(&self) -> Line<'static> {
        let rows = if self.borders().contains(TableBorders::INNER_HORIZONTAL) {
            "on"
        } else {
            "off"
        };
        Line::from(format!(
            " {} │ {} │ rows {rows} │ spacing {} ",
            PRESETS[self.preset].0, SETS[self.set].0, self.column_spacing,
        ))
        .bold()
    }
}
