//! Row, column, and cell selection.
//!
//! ```shell
//! cargo run -p selection
//! ```
//!
//! The arrow keys move the selection on both axes. The axis keys pick which of the three
//! [`TableSelection`] variants is active, and the placement key decides which columns make room
//! for the highlight symbol.

use std::io::Result;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};
use ratatui_table::{HighlightPlacement, HighlightSpacing, Row, Table, TableSelection, TableState};

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

const HELP: &str =
    " ↑/↓ row  ←/→ column │ r row  c column  x cell  n none │ p placement  s spacing │ q quit ";

fn main() -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

struct App {
    items: Vec<Vec<&'static str>>,
    state: TableState,
    placement: HighlightPlacement,
    spacing: HighlightSpacing,
}

impl App {
    fn new() -> Self {
        Self {
            items: ITEMS.iter().map(|item| item.to_vec()).collect(),
            state: TableState::new().with_selected_cell(Some((0, 1))),
            placement: HighlightPlacement::FirstColumn,
            spacing: HighlightSpacing::Always,
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
                    KeyCode::Down | KeyCode::Char('j') => self.state.select_next_row(),
                    KeyCode::Up | KeyCode::Char('k') => self.state.select_previous_row(),
                    KeyCode::Right | KeyCode::Char('l') => self.state.select_next_column(),
                    KeyCode::Left | KeyCode::Char('h') => self.state.select_previous_column(),
                    KeyCode::Char('r') => self.state.set_selection(TableSelection::Row(self.row())),
                    KeyCode::Char('c') => {
                        self.state
                            .set_selection(TableSelection::Column(self.column()));
                    }
                    KeyCode::Char('x') => self.state.set_selection(TableSelection::Cell {
                        row: self.row(),
                        column: self.column(),
                    }),
                    KeyCode::Char('n') => self.state.set_selection(None),
                    KeyCode::Char('p') => self.placement = next_placement(&self.placement),
                    KeyCode::Char('s') => self.spacing = next_spacing(&self.spacing),
                    _ => {}
                }
                self.clamp_selection();
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

        // The configuration is rendered around the table rather than inside it, so the table shows
        // nothing but the effect of the current settings.
        frame.render_widget(self.config_line(), config_area);
        let table = self.table();
        frame.render_stateful_widget(table, table_area, &mut self.state);
        frame.render_widget(Line::from(HELP).dim(), help_area);
    }

    fn table(&self) -> Table<'static> {
        let rows = self.items.iter().map(|item| Row::new(item.iter().copied()));
        let widths = [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(6),
        ];
        Table::new(rows, widths)
            .header(Row::new(HEADER).style(Style::new().bold()))
            .block(Block::bordered().title(" ratatui-table "))
            .row_highlight_style(Style::new().bg(Color::Indexed(17)))
            .column_highlight_style(Style::new().bg(Color::Indexed(22)))
            .cell_highlight_style(Style::new().bg(Color::Indexed(130)))
            .highlight_symbol(">>")
            .highlight_spacing(self.spacing.clone())
            .highlight_placement(self.placement.clone())
    }

    fn config_line(&self) -> Line<'static> {
        Line::from(format!(
            " selection: {}   placement: {}   spacing: {} ",
            selection_label(self.state.selection()),
            placement_label(&self.placement),
            self.spacing,
        ))
        .bold()
    }

    /// The row the axis keys switch to, which is the selected one when there is one.
    fn row(&self) -> usize {
        self.state.selected_row().unwrap_or(0)
    }

    /// The column the axis keys switch to, which is the selected one when there is one.
    fn column(&self) -> usize {
        self.state.selected_column().unwrap_or(0)
    }

    /// Keeps the selection inside the table.
    ///
    /// The navigation methods deliberately do not know the table's size — an out of range index is
    /// clamped on render — so a demo that displays the indexes has to clamp them itself.
    fn clamp_selection(&mut self) {
        let last_row = self.items.len().saturating_sub(1);
        let last_column = HEADER.len().saturating_sub(1);
        if let Some(row) = self.state.selected_row() {
            self.state.select_row(Some(row.min(last_row)));
        }
        if let Some(column) = self.state.selected_column() {
            self.state.select_column(Some(column.min(last_column)));
        }
    }
}

fn next_placement(placement: &HighlightPlacement) -> HighlightPlacement {
    match placement {
        HighlightPlacement::FirstColumn => HighlightPlacement::SelectedColumn,
        HighlightPlacement::SelectedColumn => HighlightPlacement::Columns(vec![0, 2]),
        HighlightPlacement::Columns(_) => HighlightPlacement::AllColumns,
        HighlightPlacement::AllColumns => HighlightPlacement::FirstColumn,
    }
}

const fn next_spacing(spacing: &HighlightSpacing) -> HighlightSpacing {
    match spacing {
        HighlightSpacing::Always => HighlightSpacing::WhenSelected,
        HighlightSpacing::WhenSelected => HighlightSpacing::Never,
        HighlightSpacing::Never => HighlightSpacing::Always,
    }
}

fn placement_label(placement: &HighlightPlacement) -> String {
    match placement {
        HighlightPlacement::Columns(columns) => format!("Columns({columns:?})"),
        other => other.to_string(),
    }
}

fn selection_label(selection: Option<TableSelection>) -> String {
    match selection {
        Some(TableSelection::Row(row)) => format!("Row({row})"),
        Some(TableSelection::Column(column)) => format!("Column({column})"),
        Some(TableSelection::Cell { row, column }) => format!("Cell({row}, {column})"),
        None => "None".to_string(),
    }
}
