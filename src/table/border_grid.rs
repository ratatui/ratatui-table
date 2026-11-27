use alloc::vec::Vec;
use core::ops::Range;

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::Style;
use ratatui_core::symbols::line;

use super::{Row, TableBorders};

#[derive(Debug)]
pub struct BorderGrid {
    borders: TableBorders,
    frame: Rect,
    verticals: Vec<u16>,
    horizontals: Vec<u16>,
    columns: Vec<Rect>,
    spans: Vec<Rect>,
}

impl BorderGrid {
    pub fn new(borders: TableBorders, frame: Rect) -> Self {
        let mut verticals = Vec::new();
        let mut horizontals = Vec::new();
        if borders.contains(TableBorders::LEFT) {
            verticals.push(frame.left());
        }
        if borders.contains(TableBorders::RIGHT) {
            verticals.push(frame.right().saturating_sub(1));
        }
        if borders.contains(TableBorders::TOP) {
            horizontals.push(frame.top());
        }
        if borders.contains(TableBorders::BOTTOM) {
            horizontals.push(frame.bottom().saturating_sub(1));
        }
        Self {
            borders,
            frame,
            verticals,
            horizontals,
            columns: Vec::new(),
            spans: Vec::new(),
        }
    }

    pub fn add_columns(
        &mut self,
        area: Rect,
        column_widths: &[Rect],
        symbol_columns: &[bool],
        selection_width: u16,
    ) {
        if !self.borders.contains(TableBorders::INNER_VERTICAL) {
            return;
        }
        self.columns = column_widths
            .iter()
            .map(|column| Rect {
                x: area.x.saturating_add(column.x),
                ..*column
            })
            .collect();
        for (index, pair) in self.columns.windows(2).enumerate() {
            let start = pair[0].right();
            let slot = if symbol_columns.get(index + 1).copied().unwrap_or(false) {
                selection_width
            } else {
                0
            };
            let end = pair[1].x.saturating_sub(slot).min(area.right());
            if start < end {
                self.verticals.push(start + (end - start - 1) / 2);
            }
        }
    }

    pub fn add_horizontal(&mut self, y: u16) {
        self.horizontals.push(y);
    }

    pub fn add_spans(&mut self, ys: Range<u16>, row: &Row) {
        if ys.is_empty() {
            return;
        }
        for columns in row.column_ranges().filter(|columns| columns.len() > 1) {
            let Some(first) = self.columns.get(columns.start) else {
                break;
            };
            let last = self.columns[columns.end.min(self.columns.len()) - 1];
            self.spans.push(Rect::new(
                first.x,
                ys.start,
                last.right().saturating_sub(first.x),
                ys.end - ys.start,
            ));
        }
    }

    pub fn render(&self, buf: &mut Buffer, set: &line::Set, style: Style) {
        let area = self.frame.intersection(buf.area);
        for &x in &self.verticals {
            for y in area.top()..area.bottom() {
                if !self.horizontals.contains(&y) && self.has_vertical(x, y) {
                    paint(buf, x, y, set.vertical, style);
                }
            }
        }
        for &y in &self.horizontals {
            if !(area.top()..area.bottom()).contains(&y) {
                continue;
            }
            for x in area.left()..area.right() {
                let arms = Arms {
                    up: y > self.frame.top() && self.has_vertical(x, y - 1),
                    down: self.has_vertical(x, y.saturating_add(1)),
                    left: x > self.frame.left(),
                    right: x.saturating_add(1) < self.frame.right(),
                };
                paint(buf, x, y, arms.glyph(set), style);
            }
        }
    }

    fn has_vertical(&self, x: u16, y: u16) -> bool {
        let position = Position { x, y };
        self.verticals.contains(&x)
            && self.frame.contains(position)
            && !self.spans.iter().any(|span| span.contains(position))
    }
}

fn paint(buf: &mut Buffer, x: u16, y: u16, glyph: &str, style: Style) {
    if let Some(cell) = buf.cell_mut((x, y)) {
        cell.set_symbol(glyph).set_style(style);
    }
}

#[derive(Debug, Clone, Copy)]
#[expect(clippy::struct_excessive_bools)]
struct Arms {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Arms {
    const fn glyph<'a>(self, set: &line::Set<'a>) -> &'a str {
        match (self.up, self.down, self.left, self.right) {
            (true, true, true, true) => set.cross,
            (false, true, true, true) => set.horizontal_down,
            (true, false, true, true) => set.horizontal_up,
            (true, true, false, true) => set.vertical_right,
            (true, true, true, false) => set.vertical_left,
            (false, true, false, true) => set.top_left,
            (false, true, true, false) => set.top_right,
            (true, false, false, true) => set.bottom_left,
            (true, false, true, false) => set.bottom_right,
            (true, _, false, false) | (_, true, false, false) => set.vertical,
            (false, false, _, _) => set.horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::cross((true, true, true, true), "┼")]
    #[case::tee_down((false, true, true, true), "┬")]
    #[case::tee_up((true, false, true, true), "┴")]
    #[case::tee_right((true, true, false, true), "├")]
    #[case::tee_left((true, true, true, false), "┤")]
    #[case::top_left((false, true, false, true), "┌")]
    #[case::top_right((false, true, true, false), "┐")]
    #[case::bottom_left((true, false, false, true), "└")]
    #[case::bottom_right((true, false, true, false), "┘")]
    #[case::horizontal((false, false, true, true), "─")]
    #[case::right_end_of_a_horizontal((false, false, true, false), "─")]
    #[case::left_end_of_a_horizontal((false, false, false, true), "─")]
    #[case::vertical((true, true, false, false), "│")]
    #[case::bottom_end_of_a_vertical((true, false, false, false), "│")]
    #[case::top_end_of_a_vertical((false, true, false, false), "│")]
    #[case::one_cell_line((false, false, false, false), "─")]
    fn arms_pick_the_glyph_that_joins_them(
        #[case] (up, down, left, right): (bool, bool, bool, bool),
        #[case] expected: &str,
    ) {
        let arms = Arms {
            up,
            down,
            left,
            right,
        };
        assert_eq!(arms.glyph(&line::NORMAL), expected);
    }
}
