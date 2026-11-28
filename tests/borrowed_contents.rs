use std::hash::{DefaultHasher, Hash, Hasher};

use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, StatefulWidget, Widget};
use ratatui_table::{Cell, Row, Table, TableState};
use rstest::rstest;

const WIDTHS: [Constraint; 3] = [
    Constraint::Length(6),
    Constraint::Min(8),
    Constraint::Fill(1),
];

fn people() -> Vec<Row<'static>> {
    vec![
        Row::new([Cell::new("Alice").column_span(2), Cell::new("Admin")]),
        Row::new(["Bob\nB.", "Designer", "Active"]).height(2),
        Row::new(["Carol", "Engineer", "Away"])
            .top_margin(1)
            .bottom_margin(1),
        Row::new(["Dave", "Support", "Active"]).style(Style::new().fg(Color::Green)),
        Row::new(["Erin", "Sales", "Idle"]),
    ]
}

fn decorate(table: Table<'_>) -> Table<'_> {
    table
        .header(Row::new(["Name", "Role", "Status"]).style(Style::new().fg(Color::Yellow)))
        .footer(Row::new([Cell::new("5 people").column_span(3)]))
        .block(Block::bordered().title("People"))
        .highlight_symbol(">>")
        .row_highlight_style(Style::new().bg(Color::Blue))
        .column_highlight_style(Style::new().fg(Color::Red))
        .cell_highlight_style(Style::new().bg(Color::Magenta))
}

fn render(table: Table<'_>, area: Rect, mut state: TableState) -> (Buffer, TableState) {
    let mut buf = Buffer::empty(area);
    StatefulWidget::render(table, area, &mut buf, &mut state);
    (buf, state)
}

fn hash(table: &Table<'_>) -> u64 {
    let mut hasher = DefaultHasher::new();
    table.hash(&mut hasher);
    hasher.finish()
}

#[rstest]
#[case::nothing_selected(0, None, None)]
#[case::last_row_and_a_column(0, Some(4), Some(1))]
#[case::offset_past_the_selection(3, Some(1), None)]
#[case::column_only(3, None, Some(2))]
fn borrowed_rows_render_like_owned_rows(
    #[case] offset: usize,
    #[case] selected: Option<usize>,
    #[case] selected_column: Option<usize>,
) {
    let rows = people();
    let area = Rect::new(0, 0, 32, 9);
    let state = TableState::default()
        .with_offset(offset)
        .with_selected(selected)
        .with_selected_column(selected_column);

    let owned = render(decorate(Table::new(rows.clone(), WIDTHS)), area, state);
    let borrowed = render(decorate(Table::from(&rows).widths(WIDTHS)), area, state);

    assert_eq!(borrowed, owned);
}

#[test]
fn borrowed_rows_compare_and_hash_like_owned_rows() {
    let rows = people();
    let borrowed = Table::from(&rows).widths(WIDTHS);
    let owned = Table::new(rows.clone(), WIDTHS);

    assert_eq!(borrowed, owned);
    assert_eq!(hash(&borrowed), hash(&owned));
    assert_eq!(format!("{borrowed:?}"), format!("{owned:?}"));
}

#[test]
fn lazy_rows_borrow_cached_cells() {
    let cache: Vec<Vec<Cell<'static>>> = (0..50)
        .map(|index| vec![Cell::from(format!("row {index}")), Cell::from("cached")])
        .collect();
    let widths = [Constraint::Length(8), Constraint::Fill(1)];
    let area = Rect::new(0, 0, 20, 6);
    let state = TableState::default().with_selected(30);

    let lazy = Table::lazy_rows(cache.len(), widths, |index| Row::from(&cache[index]))
        .highlight_symbol(">");
    let owned = Table::new(cache.iter().cloned().map(Row::new), widths).highlight_symbol(">");

    assert_eq!(render(lazy, area, state), render(owned, area, state));
}

struct App {
    rows: Vec<Row<'static>>,
    header: Row<'static>,
}

// Fails to compile if `Row` stores its cells in `Cow<'a, [Cell<'a>]>`, which makes `Row` invariant
// over `'a`.
fn table<'s>(app: &'s App, title: &'s str) -> Table<'s> {
    Table::from(&app.rows)
        .widths([Constraint::Length(5); 2])
        .header(Row::from(&app.header))
        .block(Block::bordered().title(title))
}

#[test]
fn rows_cached_with_a_static_lifetime_borrow_into_a_shorter_table() {
    let app = App {
        rows: vec![Row::new(["Cell1", "Cell2"]), Row::new(["Cell3", "Cell4"])],
        header: Row::new(["Col1", "Col2"]),
    };
    let title = String::from("Users");
    let area = Rect::new(0, 0, 15, 5);
    let mut buf = Buffer::empty(area);

    Widget::render(table(&app, &title), area, &mut buf);

    assert_eq!(
        buf,
        Buffer::with_lines([
            "┌Users────────┐",
            "│Col1  Col2   │",
            "│Cell1 Cell2  │",
            "│Cell3 Cell4  │",
            "└─────────────┘",
        ])
    );
}
