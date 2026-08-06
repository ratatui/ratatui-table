use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};
#[cfg(feature = "serde")]
use ratatui_table::TableSelection;
use ratatui_table::{Cell, HighlightSpacing, Row, Table, TableState};
use ratatui_widgets::table as builtin;

#[test]
fn renders_static_layout_like_builtin_table() {
    let area = Rect::new(0, 0, 34, 9);
    let block = Block::new().borders(Borders::ALL).title("People");
    let widths = [
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Fill(1),
    ];

    let extracted = Table::new(
        [
            Row::new([Cell::new("Alice").column_span(2), Cell::new("Admin")]),
            Row::new(["Bob\nB.", "Designer", "Active"]).height(2),
        ],
        widths,
    )
    .header(Row::new(["Name", "Role", "Status"]).style(Style::new().bold()))
    .footer(Row::new([Cell::new("2 people").column_span(3)]))
    .block(block.clone())
    .column_spacing(1)
    .flex(Flex::SpaceBetween)
    .style(Style::new().fg(Color::White));

    let builtin = builtin::Table::new(
        [
            builtin::Row::new([
                builtin::Cell::new("Alice").column_span(2),
                builtin::Cell::new("Admin"),
            ]),
            builtin::Row::new(["Bob\nB.", "Designer", "Active"]).height(2),
        ],
        widths,
    )
    .header(builtin::Row::new(["Name", "Role", "Status"]).style(Style::new().bold()))
    .footer(builtin::Row::new([
        builtin::Cell::new("2 people").column_span(3)
    ]))
    .block(block)
    .column_spacing(1)
    .flex(Flex::SpaceBetween)
    .style(Style::new().fg(Color::White));

    let mut extracted_buffer = Buffer::empty(area);
    let mut builtin_buffer = Buffer::empty(area);
    Widget::render(extracted, area, &mut extracted_buffer);
    Widget::render(builtin, area, &mut builtin_buffer);

    assert_eq!(extracted_buffer, builtin_buffer);
}

#[test]
fn renders_selection_and_scrolling_like_builtin_table() {
    let area = Rect::new(0, 0, 24, 5);
    let widths = [Constraint::Length(7), Constraint::Length(7)];
    let extracted_rows = (0..8).map(|index| Row::new([format!("row {index}"), "value".to_owned()]));
    let builtin_rows =
        (0..8).map(|index| builtin::Row::new([format!("row {index}"), "value".to_owned()]));

    let extracted = Table::new(extracted_rows, widths)
        .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .column_highlight_style(Style::new().fg(Color::Yellow))
        .cell_highlight_style(Style::new().bg(Color::Blue))
        .highlight_symbol("> ")
        .highlight_spacing(HighlightSpacing::Always);
    let builtin = builtin::Table::new(builtin_rows, widths)
        .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .column_highlight_style(Style::new().fg(Color::Yellow))
        .cell_highlight_style(Style::new().bg(Color::Blue))
        .highlight_symbol("> ")
        .highlight_spacing(builtin::HighlightSpacing::Always);

    let mut extracted_state = TableState::new()
        .with_offset(1)
        .with_selected_cell(Some((6, 1)));
    let mut builtin_state = builtin::TableState::new()
        .with_offset(1)
        .with_selected_cell(Some((6, 1)));
    let mut extracted_buffer = Buffer::empty(area);
    let mut builtin_buffer = Buffer::empty(area);

    StatefulWidget::render(extracted, area, &mut extracted_buffer, &mut extracted_state);
    StatefulWidget::render(builtin, area, &mut builtin_buffer, &mut builtin_state);

    assert_eq!(extracted_buffer, builtin_buffer);
    assert_eq!(extracted_state.offset(), builtin_state.offset());
    assert_eq!(extracted_state.selected_row(), builtin_state.selected());
    assert_eq!(
        extracted_state.selected_column(),
        builtin_state.selected_column()
    );
}

#[test]
fn handles_small_areas_like_builtin_table() {
    for width in 0..=4 {
        for height in 0..=3 {
            let area = Rect::new(0, 0, width, height);
            let extracted = Table::new([Row::new(["one", "two"])], [5, 5])
                .block(Block::bordered())
                .highlight_symbol(">>");
            let builtin = builtin::Table::new([builtin::Row::new(["one", "two"])], [5, 5])
                .block(Block::bordered())
                .highlight_symbol(">>");
            let mut extracted_buffer = Buffer::empty(area);
            let mut builtin_buffer = Buffer::empty(area);

            Widget::render(extracted, area, &mut extracted_buffer);
            Widget::render(builtin, area, &mut builtin_buffer);

            assert_eq!(
                extracted_buffer, builtin_buffer,
                "rendering differed for {width}x{height}"
            );
        }
    }
}

/// The default [`HighlightPlacement`] must lay out exactly like the built-in table: reserve the
/// symbol column at the left edge and shift every column right by its width.
#[test]
fn default_highlight_placement_matches_builtin_table() {
    let area = Rect::new(0, 0, 24, 4);
    let widths = [Constraint::Length(6), Constraint::Length(6)];
    let rows = ["a", "b", "c"];

    let extracted = Table::new(rows.map(|r| Row::new([r, r])), widths)
        .highlight_symbol(">> ")
        .flex(Flex::Start);
    let builtin = builtin::Table::new(rows.map(|r| builtin::Row::new([r, r])), widths)
        .highlight_symbol(">> ")
        .flex(Flex::Start);

    let mut extracted_state = TableState::new().with_selected_row(1);
    let mut builtin_state = builtin::TableState::new().with_selected(1);
    let mut extracted_buffer = Buffer::empty(area);
    let mut builtin_buffer = Buffer::empty(area);

    StatefulWidget::render(extracted, area, &mut extracted_buffer, &mut extracted_state);
    StatefulWidget::render(builtin, area, &mut builtin_buffer, &mut builtin_state);

    assert_eq!(extracted_buffer, builtin_buffer);
}

#[cfg(feature = "serde")]
#[test]
fn serializes_state_with_serde_feature() {
    let state = TableState::new()
        .with_offset(3)
        .with_selected_cell(Some((5, 2)));
    let json = serde_json::to_string(&state).unwrap();
    let round_trip: TableState = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip, state);
}

#[cfg(feature = "serde")]
#[test]
fn serializes_every_selection_arm() {
    for selection in [
        TableSelection::Row(1),
        TableSelection::Column(2),
        TableSelection::Cell { row: 3, column: 4 },
    ] {
        let state = TableState::new().with_selection(selection);
        let json = serde_json::to_string(&state).unwrap();
        let round_trip: TableState = serde_json::from_str(&json).unwrap();
        assert_eq!(round_trip, state);
    }
}
