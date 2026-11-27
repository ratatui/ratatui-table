use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::line;
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};
use ratatui_table::{Cell, HighlightPlacement, Row, Table, TableBorders, TableState};
use rstest::rstest;

fn render(table: &Table, width: u16, height: u16, state: &mut TableState) -> Buffer {
    let mut buf = Buffer::empty(Rect::new(0, 0, width, height));
    StatefulWidget::render(table, buf.area, &mut buf, state);
    buf
}

#[track_caller]
fn assert_renders(table: &Table, state: &mut TableState, expected: &[&str]) {
    let expected = Buffer::with_lines(expected.iter().copied());
    let buf = render(table, expected.area.width, expected.area.height, state);
    assert_eq!(buf, expected);
}

fn lines(buf: &Buffer) -> Vec<String> {
    (0..buf.area.height)
        .map(|y| (0..buf.area.width).map(|x| buf[(x, y)].symbol()).collect())
        .collect()
}

/// A 2x2 table with a header `A B`, the shape issue #2 uses for its targets.
fn ab() -> Table<'static> {
    Table::new(
        [Row::new(["1", "2"]), Row::new(["3", "4"])],
        [Constraint::Length(1); 2],
    )
    .header(Row::new(["A", "B"]))
}

fn people() -> Table<'static> {
    Table::new(
        [
            Row::new(["Alice", "Admin", "Berlin"]),
            Row::new(["Bob", "Dev", "Paris"]),
        ],
        [Constraint::Length(5); 3],
    )
    .header(Row::new(["Name", "Role", "City"]))
    .borders(TableBorders::INNER_VERTICAL)
}

fn ids(count: usize) -> Table<'static> {
    Table::new(
        (0..count).map(|i| Row::new([format!("r{i}"), format!("v{i}")])),
        [Constraint::Length(2); 2],
    )
    .header(Row::new(["id", "v"]))
    .borders(TableBorders::ALL)
}

#[rstest]
#[case::all(TableBorders::ALL, &[
    "┌─┬─┐",
    "│A│B│",
    "├─┼─┤",
    "│1│2│",
    "├─┼─┤",
    "│3│4│",
    "└─┴─┘",
])]
#[case::columns_and_header(TableBorders::INNER_VERTICAL | TableBorders::HEADER_SEPARATOR, &[
    "A│B",
    "─┼─",
    "1│2",
    "3│4",
])]
#[case::header_only(TableBorders::HEADER_SEPARATOR, &[
    "A B",
    "───",
    "1 2",
    "3 4",
])]
#[case::columns_only(TableBorders::INNER_VERTICAL, &[
    "A│B",
    "1│2",
    "3│4",
])]
fn borders_draw_the_issue_targets(#[case] borders: TableBorders, #[case] expected: &[&str]) {
    assert_renders(&ab().borders(borders), &mut TableState::default(), expected);
}

#[test]
fn outer_borders_match_a_bordered_block_from_two_cells_up() {
    for width in 2..=12 {
        for height in 2..=6 {
            let area = Rect::new(0, 0, width, height);
            let mut block = Buffer::empty(area);
            let mut borders = Buffer::empty(area);
            Widget::render(ab().block(Block::bordered()), area, &mut block);
            Widget::render(ab().borders(TableBorders::OUTER), area, &mut borders);
            assert_eq!(borders, block, "{width}x{height}");
        }
    }
}

#[rstest]
fn outer_flags_match_a_block_with_the_same_borders_bits_from_two_cells_up(
    #[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)] bits: u8,
) {
    let sides = Borders::from_bits(bits).unwrap();
    let borders = TableBorders::from_bits(bits).unwrap();
    for width in 2..=5 {
        for height in 2..=5 {
            let area = Rect::new(0, 0, width, height);
            let mut expected = Buffer::empty(area);
            let mut buf = Buffer::empty(area);
            Widget::render(ab().block(Block::new().borders(sides)), area, &mut expected);
            Widget::render(ab().borders(borders), area, &mut buf);
            assert_eq!(buf, expected, "{borders:?} {width}x{height}");
        }
    }
}

#[test]
fn borders_draw_the_frame_when_no_cell_fits() {
    assert_renders(
        &ab().borders(TableBorders::ALL),
        &mut TableState::default(),
        &["┌┐", "└┘"],
    );
}

#[test]
fn separator_without_a_top_line_joins_the_side_lines() {
    let borders = TableBorders::LEFT
        | TableBorders::RIGHT
        | TableBorders::BOTTOM
        | TableBorders::INNER_VERTICAL
        | TableBorders::HEADER_SEPARATOR;
    #[rustfmt::skip]
    let expected = [
        "│A│B│",
        "├─┼─┤",
        "│1│2│",
        "│3│4│",
        "└─┴─┘",
    ];
    assert_renders(
        &ab().borders(borders),
        &mut TableState::default(),
        &expected,
    );
}

#[rstest]
#[case::no_spacing(0, &[
    "Name │Role │City     ",
    "Alice│Admin│Berli    ",
    "Bob  │Dev  │Paris    ",
])]
#[case::one_cell(1, &[
    "Name │Role │City     ",
    "Alice│Admin│Berli    ",
    "Bob  │Dev  │Paris    ",
])]
#[case::even_gap_rounds_left(2, &[
    "Name │ Role │ City   ",
    "Alice│ Admin│ Berli  ",
    "Bob  │ Dev  │ Paris  ",
])]
#[case::odd_gap_centers(3, &[
    "Name  │ Role  │ City ",
    "Alice │ Admin │ Berli",
    "Bob   │ Dev   │ Paris",
])]
fn inner_vertical_line_sits_in_the_middle_of_the_column_gap(
    #[case] spacing: u16,
    #[case] expected: &[&str],
) {
    assert_renders(
        &people().column_spacing(spacing),
        &mut TableState::default(),
        expected,
    );
}

#[rstest]
#[case::space_between(Flex::SpaceBetween, &[
    "Name   │  Role   │  City ",
    "Alice  │  Admin  │  Berli",
    "Bob    │  Dev    │  Paris",
])]
#[case::space_around(Flex::SpaceAround, &[
    "  Name  │ Role  │ City   ",
    "  Alice │ Admin │ Berli  ",
    "  Bob   │ Dev   │ Paris  ",
])]
fn inner_vertical_line_follows_the_flexed_gap(#[case] flex: Flex, #[case] expected: &[&str]) {
    assert_renders(&people().flex(flex), &mut TableState::default(), expected);
}

#[test]
fn horizontal_lines_join_the_lines_of_flexed_columns() {
    assert_renders(
        &people().flex(Flex::SpaceBetween).borders(TableBorders::ALL),
        &mut TableState::default(),
        &[
            "┌──────┬────────┬───────┐",
            "│Name  │  Role  │  City │",
            "├──────┼────────┼───────┤",
            "│Alice │  Admin │  Berli│",
            "├──────┼────────┼───────┤",
            "│Bob   │  Dev   │  Paris│",
            "└──────┴────────┴───────┘",
        ],
    );
}

fn spans() -> Table<'static> {
    Table::new(
        [
            Row::new(["1", "2", "3"]),
            Row::new([Cell::new("wide").column_span(2), Cell::new("x")]),
            Row::new(["4", "5", "6"]),
            Row::new([Cell::new("all three").column_span(3)]),
        ],
        [Constraint::Length(3); 3],
    )
    .header(Row::new(["A", "B", "C"]))
}

#[rstest]
#[case::all(TableBorders::ALL, &[
    "┌───┬───┬───┐",
    "│A  │B  │C  │",
    "├───┼───┼───┤",
    "│1  │2  │3  │",
    "├───┴───┼───┤",
    "│wide   │x  │",
    "├───┬───┼───┤",
    "│4  │5  │6  │",
    "├───┴───┴───┤",
    "│all three  │",
    "└───────────┘",
])]
#[case::inner(TableBorders::INNER, &[
    "A  │B  │C  ",
    "───┼───┼───",
    "1  │2  │3  ",
    "───┴───┼───",
    "wide   │x  ",
    "───┬───┼───",
    "4  │5  │6  ",
    "───┴───┴───",
    "all three  ",
])]
fn spanned_cells_hide_the_lines_they_cover(
    #[case] borders: TableBorders,
    #[case] expected: &[&str],
) {
    assert_renders(
        &spans().borders(borders),
        &mut TableState::default(),
        expected,
    );
}

#[test]
fn spanned_header_turns_the_separator_junction_down() {
    let table = Table::new(
        [Row::new(["1", "2"]), Row::new(["3", "4"])],
        [Constraint::Length(1); 2],
    )
    .header(Row::new([Cell::new("AB").column_span(2)]))
    .borders(TableBorders::ALL);
    assert_renders(
        &table,
        &mut TableState::default(),
        &[
            "┌───┐",
            "│AB │",
            "├─┬─┤",
            "│1│2│",
            "├─┼─┤",
            "│3│4│",
            "└─┴─┘",
        ],
    );
}

#[test]
fn spanned_rows_hide_lines_through_their_margins() {
    let table = Table::new(
        [
            Row::new([Cell::new("span").column_span(2)]).top_margin(1),
            Row::new(["1", "2"]),
        ],
        [Constraint::Length(3); 2],
    )
    .header(Row::new([Cell::new("Head").column_span(2)]).bottom_margin(1))
    .borders(TableBorders::ALL);
    assert_renders(
        &table,
        &mut TableState::default(),
        &[
            "┌───────┐",
            "│Head   │",
            "│       │",
            "├───────┤",
            "│       │",
            "│span   │",
            "├───┬───┤",
            "│1  │2  │",
            "└───┴───┘",
        ],
    );
}

fn selectable() -> Table<'static> {
    Table::new(
        [
            Row::new(["Alice", "Admin"]),
            Row::new(["Bob", "Dev"]),
            Row::new(["Carol", "Ops"]),
        ],
        [Constraint::Length(5); 2],
    )
    .highlight_symbol(">>")
    .borders(TableBorders::OUTER | TableBorders::INNER_VERTICAL)
}

#[rstest]
#[case::first_column(HighlightPlacement::FirstColumn, &[
    "┌───────┬──────┐",
    "│  Alice│Admin │",
    "│>>Bob  │Dev   │",
    "│  Carol│Ops   │",
    "└───────┴──────┘",
])]
#[case::all_columns(HighlightPlacement::AllColumns, &[
    "┌───────┬────────┐",
    "│  Alice│  Admin │",
    "│>>Bob  │>>Dev   │",
    "│  Carol│  Ops   │",
    "└───────┴────────┘",
])]
fn highlight_symbol_sits_inside_the_box_of_its_column(
    #[case] placement: HighlightPlacement,
    #[case] expected: &[&str],
) {
    assert_renders(
        &selectable().highlight_placement(placement),
        &mut TableState::default().with_selected(1),
        expected,
    );
}

#[rstest]
#[case::middle(5, 3, &[
    "┌──┬──┐",
    "│id│v │",
    "├──┼──┤",
    "│r3│v3│",
    "├──┼──┤",
    "│r4│v4│",
    "├──┼──┤",
    "│r5│v5│",
    "└──┴──┘",
])]
#[case::last(7, 5, &[
    "┌──┬──┐",
    "│id│v │",
    "├──┼──┤",
    "│r5│v5│",
    "├──┼──┤",
    "│r6│v6│",
    "├──┼──┤",
    "│r7│v7│",
    "└──┴──┘",
])]
fn row_separators_scroll_to_the_selected_row(
    #[case] selected: usize,
    #[case] offset: usize,
    #[case] expected: &[&str],
) {
    let mut state = TableState::default().with_selected(selected);
    assert_renders(&ids(8), &mut state, expected);
    assert_eq!(state.offset(), offset);
}

#[test]
fn row_separators_scroll_one_row_per_step() {
    let table = ids(8);
    let mut state = TableState::default();
    let mut offset = 0;
    for selected in (0..8).chain((0..8).rev()) {
        state.select(Some(selected));
        let buf = render(&table, 7, 9, &mut state);
        assert!(
            state.offset().abs_diff(offset) <= 1,
            "selecting {selected} moved the offset from {offset} to {}",
            state.offset()
        );
        offset = state.offset();
        let label = format!("│r{selected}│");
        assert!(
            lines(&buf).iter().any(|line| line.contains(&label)),
            "row {selected} is not visible"
        );
    }
}

#[test]
fn row_separator_stays_blank_when_the_next_row_does_not_fit() {
    let mut state = TableState::default().with_selected(5);
    assert_renders(
        &ids(8),
        &mut state,
        &[
            "┌──┬──┐",
            "│id│v │",
            "├──┼──┤",
            "│r3│v3│",
            "├──┼──┤",
            "│r4│v4│",
            "├──┼──┤",
            "│r5│v5│",
            "│  │  │",
            "└──┴──┘",
        ],
    );
    assert_eq!(state.offset(), 3);
}

#[test]
fn column_lines_run_through_the_empty_rows_area_to_the_footer() {
    assert_renders(
        &ab().footer(Row::new(["F", "G"])).borders(TableBorders::ALL),
        &mut TableState::default(),
        &[
            "┌─┬─┐",
            "│A│B│",
            "├─┼─┤",
            "│1│2│",
            "├─┼─┤",
            "│3│4│",
            "│ │ │",
            "├─┼─┤",
            "│F│G│",
            "└─┴─┘",
        ],
    );
}

const ASCII: line::Set = line::Set {
    vertical: "|",
    horizontal: "-",
    top_right: "+",
    top_left: "+",
    bottom_right: "+",
    bottom_left: "+",
    vertical_left: "+",
    vertical_right: "+",
    horizontal_down: "+",
    horizontal_up: "+",
    cross: "+",
};

#[rstest]
#[case::rounded(line::ROUNDED, &[
    "╭─┬─╮",
    "│A│B│",
    "├─┼─┤",
    "│1│2│",
    "├─┼─┤",
    "│3│4│",
    "╰─┴─╯",
])]
#[case::double(line::DOUBLE, &[
    "╔═╦═╗",
    "║A║B║",
    "╠═╬═╣",
    "║1║2║",
    "╠═╬═╣",
    "║3║4║",
    "╚═╩═╝",
])]
#[case::thick(line::THICK, &[
    "┏━┳━┓",
    "┃A┃B┃",
    "┣━╋━┫",
    "┃1┃2┃",
    "┣━╋━┫",
    "┃3┃4┃",
    "┗━┻━┛",
])]
#[case::ascii(ASCII, &[
    "+-+-+",
    "|A|B|",
    "+-+-+",
    "|1|2|",
    "+-+-+",
    "|3|4|",
    "+-+-+",
])]
fn border_set_supplies_every_glyph(#[case] set: line::Set<'static>, #[case] expected: &[&str]) {
    assert_renders(
        &ab().borders(TableBorders::ALL).border_set(set),
        &mut TableState::default(),
        expected,
    );
}

fn style_map(buf: &Buffer) -> Vec<String> {
    (0..buf.area.height)
        .map(|y| {
            (0..buf.area.width)
                .map(|x| {
                    let cell = &buf[(x, y)];
                    [
                        if cell.fg == Color::Yellow { 'Y' } else { '.' },
                        if cell.bg == Color::Blue { 'B' } else { '.' },
                        if cell.modifier.contains(Modifier::REVERSED) {
                            'R'
                        } else {
                            '.'
                        },
                    ]
                    .iter()
                    .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

#[test]
fn border_style_is_patched_over_highlights_that_never_reach_the_frame() {
    let table = Table::new(
        [
            Row::new(["a", "b"]),
            Row::new(["c", "d"]),
            Row::new(["e", "f"]),
        ],
        [Constraint::Length(1); 2],
    )
    .header(Row::new(["A", "B"]))
    .borders(TableBorders::ALL)
    .border_style(Style::new().fg(Color::Yellow))
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .column_highlight_style(Style::new().bg(Color::Blue));
    let mut state = TableState::default()
        .with_selected(1)
        .with_selected_column(0);
    let buf = render(&table, 5, 9, &mut state);
    assert_eq!(
        lines(&buf),
        [
            "┌─┬─┐",
            "│A│B│",
            "├─┼─┤",
            "│a│b│",
            "├─┼─┤",
            "│c│d│",
            "├─┼─┤",
            "│e│f│",
            "└─┴─┘",
        ]
    );
    assert_eq!(
        style_map(&buf),
        [
            "Y.. Y.. Y.. Y.. Y..",
            "Y.. ... Y.. ... Y..",
            "Y.. Y.. Y.. Y.. Y..",
            "Y.. .B. Y.. ... Y..",
            "Y.. YB. Y.. Y.. Y..",
            "Y.. .BR Y.R ..R Y..",
            "Y.. YB. Y.. Y.. Y..",
            "Y.. .B. Y.. ... Y..",
            "Y.. Y.. Y.. Y.. Y..",
        ]
    );
}

#[rstest]
fn tiny_areas_never_panic(
    #[values(
        TableBorders::ALL,
        TableBorders::TOP,
        TableBorders::RIGHT,
        TableBorders::BOTTOM,
        TableBorders::LEFT,
        TableBorders::INNER_VERTICAL,
        TableBorders::INNER_HORIZONTAL,
        TableBorders::HEADER_SEPARATOR,
        TableBorders::FOOTER_SEPARATOR
    )]
    borders: TableBorders,
) {
    let eager = spans()
        .footer(Row::new([Cell::new("F").column_span(2)]).top_margin(1))
        .highlight_symbol(">>")
        .borders(borders);
    let lazy = Table::lazy_rows(5, [Constraint::Length(1); 3], |i| {
        Row::new([Cell::new(format!("{i}")).column_span(2)])
    })
    .header(Row::new(["A", "B", "C"]))
    .highlight_symbol(">>")
    .borders(borders);
    for width in 0..=4 {
        for height in 0..=4 {
            for table in [&eager, &lazy] {
                let mut state = TableState::default()
                    .with_selected(2)
                    .with_selected_column(1);
                render(table, width, height, &mut state);
            }
        }
    }
}

#[cfg(feature = "serde")]
#[test]
fn borders_round_trip_through_serde() {
    let borders = TableBorders::OUTER | TableBorders::INNER_VERTICAL;
    let json = serde_json::to_string(&borders).unwrap();
    assert_eq!(json, r#""TOP | RIGHT | BOTTOM | LEFT | INNER_VERTICAL""#);
    assert_eq!(
        serde_json::from_str::<TableBorders>(&json).unwrap(),
        borders
    );
}
