use ratatui_core::layout::Rect;

bitflags::bitflags! {
    /// Which lines a [`Table`] draws around and between its cells.
    ///
    /// Each flag draws one family of lines:
    ///
    /// - [`TOP`], [`RIGHT`], [`BOTTOM`] and [`LEFT`] draw a line on that edge of the table area,
    ///   where [`Block::bordered`] would draw its border.
    /// - [`INNER_VERTICAL`] draws a line in the gap between each pair of adjacent columns.
    /// - [`INNER_HORIZONTAL`] draws a line between each pair of adjacent rows.
    /// - [`HEADER_SEPARATOR`] draws a line between the header and the rows.
    /// - [`FOOTER_SEPARATOR`] draws a line between the rows and the footer.
    ///
    /// [`OUTER`], [`INNER`] and [`ALL`] combine them. The default, [`TableBorders::empty`], draws
    /// no lines. Set the flags with [`Table::borders`], the glyphs with [`Table::border_set`] and
    /// the style with [`Table::border_style`]. See [Borders] for where each line goes.
    ///
    /// With a header `A B` and two rows:
    ///
    /// ```text
    /// ALL      INNER_VERTICAL | HEADER_SEPARATOR   HEADER_SEPARATOR
    /// ┌─┬─┐    A│B                                 A B
    /// │A│B│    ─┼─                                 ───
    /// ├─┼─┤    1│2                                 1 2
    /// │1│2│    3│4                                 3 4
    /// ├─┼─┤
    /// │3│4│
    /// └─┴─┘
    /// ```
    ///
    /// # Examples
    ///
    /// Flags combine with `|` or [`TableBorders::union`]:
    ///
    /// ```rust
    /// use ratatui::layout::Constraint;
    /// use ratatui_table::{Row, Table, TableBorders};
    ///
    /// let columns = TableBorders::INNER_VERTICAL | TableBorders::HEADER_SEPARATOR;
    /// let framed = TableBorders::OUTER.union(columns);
    /// assert!(framed.contains(TableBorders::LEFT | TableBorders::INNER_VERTICAL));
    /// let rows = TableBorders::INNER_HORIZONTAL | TableBorders::FOOTER_SEPARATOR;
    /// assert_eq!(framed | rows, TableBorders::ALL);
    ///
    /// let table = Table::new([Row::new(["1", "2"])], [Constraint::Length(1); 2])
    ///     .header(Row::new(["A", "B"]))
    ///     .borders(framed);
    /// ```
    ///
    /// [`Table`]: crate::Table
    /// [`Table::borders`]: crate::Table::borders
    /// [`Table::border_set`]: crate::Table::border_set
    /// [`Table::border_style`]: crate::Table::border_style
    /// [Borders]: crate::Table#borders
    /// [`Block::bordered`]: ratatui_widgets::block::Block::bordered
    /// [`TOP`]: TableBorders::TOP
    /// [`RIGHT`]: TableBorders::RIGHT
    /// [`BOTTOM`]: TableBorders::BOTTOM
    /// [`LEFT`]: TableBorders::LEFT
    /// [`INNER_VERTICAL`]: TableBorders::INNER_VERTICAL
    /// [`INNER_HORIZONTAL`]: TableBorders::INNER_HORIZONTAL
    /// [`HEADER_SEPARATOR`]: TableBorders::HEADER_SEPARATOR
    /// [`FOOTER_SEPARATOR`]: TableBorders::FOOTER_SEPARATOR
    /// [`OUTER`]: TableBorders::OUTER
    /// [`INNER`]: TableBorders::INNER
    /// [`ALL`]: TableBorders::ALL
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct TableBorders: u8 {
        /// A line along the top edge of the table area.
        const TOP = 1 << 0;
        /// A line along the right edge of the table area.
        const RIGHT = 1 << 1;
        /// A line along the bottom edge of the table area.
        const BOTTOM = 1 << 2;
        /// A line along the left edge of the table area.
        const LEFT = 1 << 3;
        /// A line in the gap between each pair of adjacent columns.
        const INNER_VERTICAL = 1 << 4;
        /// A line between each pair of adjacent rows.
        const INNER_HORIZONTAL = 1 << 5;
        /// A line between the header and the rows, drawn only when the table has a header.
        const HEADER_SEPARATOR = 1 << 6;
        /// A line between the rows and the footer, drawn only when the table has a footer.
        const FOOTER_SEPARATOR = 1 << 7;
        /// The four edges of the table area.
        const OUTER = Self::TOP.bits()
            | Self::RIGHT.bits()
            | Self::BOTTOM.bits()
            | Self::LEFT.bits();
        /// Every line inside the table area, including the header and footer separators.
        const INNER = Self::INNER_VERTICAL.bits()
            | Self::INNER_HORIZONTAL.bits()
            | Self::HEADER_SEPARATOR.bits()
            | Self::FOOTER_SEPARATOR.bits();
        /// Every line.
        const ALL = Self::OUTER.bits() | Self::INNER.bits();
    }
}

impl TableBorders {
    pub(crate) fn inner(self, area: Rect) -> Rect {
        let left = u16::from(self.contains(Self::LEFT));
        let top = u16::from(self.contains(Self::TOP));
        let right = u16::from(self.contains(Self::RIGHT));
        let bottom = u16::from(self.contains(Self::BOTTOM));
        Rect {
            x: area.x.saturating_add(left),
            y: area.y.saturating_add(top),
            width: area.width.saturating_sub(left + right),
            height: area.height.saturating_sub(top + bottom),
        }
    }

    pub(crate) fn column_spacing(self, spacing: u16) -> u16 {
        if self.contains(Self::INNER_VERTICAL) {
            spacing.max(1)
        } else {
            spacing
        }
    }

    pub(crate) fn separator_height_below(self, index: usize, row_count: usize) -> u16 {
        u16::from(self.contains(Self::INNER_HORIZONTAL) && index + 1 < row_count)
    }
}
