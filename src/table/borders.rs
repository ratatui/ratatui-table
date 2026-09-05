/// Configuration for table borders.
///
/// This struct controls whether and how borders are drawn around and between
/// cells in a [`Table`](super::Table).
///
/// # Example
///
/// ```rust
/// use ratatui_table::borders::TableBorders;
///
/// let borders = TableBorders::default()
///     .top()
///     .bottom()
///     .sides()
///     .inner_vertical()
///     .inner_horizontal();
/// ```
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct TableBorders {
    /// Whether to draw a border on the top edge of the table.
    pub top: bool,
    /// Whether to draw a border on the bottom edge of the table.
    pub bottom: bool,
    /// Whether to draw a border on the left edge of the table.
    pub left: bool,
    /// Whether to draw a border on the right edge of the table.
    pub right: bool,
    /// Whether to draw a vertical line between columns.
    pub inner_vertical: bool,
    /// Whether to draw a horizontal line between rows (and header/footer).
    pub inner_horizontal: bool,
}

impl TableBorders {
    /// Returns true if any border is enabled.
    pub fn has_any(&self) -> bool {
        self.top || self.bottom || self.left || self.right || self.inner_vertical || self.inner_horizontal
    }

    /// Add a top border.
    pub fn top(self) -> Self {
        Self { top: true, ..self }
    }

    /// Add a bottom border.
    pub fn bottom(self) -> Self {
        Self { bottom: true, ..self }
    }

    /// Add left and right borders.
    pub fn sides(self) -> Self {
        Self {
            left: true,
            right: true,
            ..self
        }
    }

    /// Add inner vertical borders between columns.
    pub fn inner_vertical(self) -> Self {
        Self {
            inner_vertical: true,
            ..self
        }
    }

    /// Add inner horizontal borders between rows.
    pub fn inner_horizontal(self) -> Self {
        Self {
            inner_horizontal: true,
            ..self
        }
    }

    /// Add all borders (top, bottom, left, right, inner).
    pub fn all(self) -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
            inner_vertical: true,
            inner_horizontal: true,
            ..self
        }
    }
}
