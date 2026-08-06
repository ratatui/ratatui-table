/// State of a [`Table`] widget
///
/// This state can be used to scroll through the rows and select one of them. When the table is
/// rendered as a stateful widget, the selected row, column and cell will be highlighted and the
/// table will be shifted to ensure that the selected row is visible. This will modify the
/// [`TableState`] object passed to the `Frame::render_stateful_widget` method.
///
/// The state consists of two fields:
/// - [`offset`]: the index of the first row to be displayed
/// - [`selection`]: what is selected — a row, a column, or a cell — or `None`
///
/// [`offset`]: TableState::offset()
/// [`selection`]: TableState::selection()
///
/// Every accessor names the axis it acts on. [`selected_row`] and [`selected_column`] read one
/// component out of the selection; [`selection`] returns the whole [`TableSelection`]. The
/// unqualified `selected`/`select` names are deprecated aliases for the row-axis methods.
///
/// [`selected_row`]: TableState::selected_row()
/// [`selected_column`]: TableState::selected_column()
///
/// See the `table` example and the `recipe` and `traceroute` tabs in the demo2 example in the
/// [Examples] directory for a more in depth example of the various configuration options and for
/// how to handle state.
///
/// [Examples]: https://github.com/ratatui/ratatui/blob/master/examples/README.md
///
/// # Example
///
/// ```rust
/// use ratatui::Frame;
/// use ratatui::layout::{Constraint, Rect};
/// use ratatui_table::{Row, Table, TableState};
///
/// # fn ui(frame: &mut Frame) {
/// # let area = Rect::default();
/// let rows = [Row::new(vec!["Cell1", "Cell2"])];
/// let widths = [Constraint::Length(5), Constraint::Length(5)];
/// let table = Table::new(rows, widths).widths(widths);
///
/// // Note: TableState should be stored in your application state (not constructed in your render
/// // method) so that the selected row is preserved across renders
/// let mut table_state = TableState::default();
/// *table_state.offset_mut() = 1; // display the second row and onwards
/// table_state.select_row(Some(3)); // select the forth row (0-indexed)
/// table_state.select_column(Some(2)); // select the third column (0-indexed)
///
/// frame.render_stateful_widget(table, area, &mut table_state);
/// # }
/// ```
///
/// Note that if [`Table::widths`] is not called before rendering, the rendered columns will have
/// equal width.
///
/// [`Table`]: super::Table
/// [`Table::widths`]: crate::table::Table::widths
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableState {
    pub(crate) offset: usize,
    pub(crate) selection: Option<TableSelection>,
}

/// What is currently selected in a [`Table`].
///
/// - [`Row`] — a full row is highlighted; no column is active.
/// - [`Column`] — a full column is highlighted; no row is active.
/// - [`Cell`] — a specific (row, column) intersection is highlighted.
///
/// Held by [`TableState`]. Use the component setters on that type ([`select_row`],
/// [`select_column`]) to change one axis at a time, or [`set_selection`] to replace the whole
/// value.
///
/// [`Table`]: super::Table
/// [`Row`]: TableSelection::Row
/// [`Column`]: TableSelection::Column
/// [`Cell`]: TableSelection::Cell
/// [`select_row`]: TableState::select_row
/// [`select_column`]: TableState::select_column
/// [`set_selection`]: TableState::set_selection
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableSelection {
    /// The row at this index is selected. No column is active.
    Row(usize),
    /// The column at this index is selected. No row is active.
    Column(usize),
    /// The intersection of a row and a column is selected.
    Cell {
        /// Index of the selected row.
        row: usize,
        /// Index of the selected column.
        column: usize,
    },
}

impl From<usize> for TableSelection {
    /// Converts a bare index to [`TableSelection::Row`].
    fn from(value: usize) -> Self {
        Self::Row(value)
    }
}

impl TableSelection {
    /// Returns the row index if this selection includes one.
    pub const fn row(&self) -> Option<usize> {
        match *self {
            Self::Row(row) | Self::Cell { row, .. } => Some(row),
            Self::Column(_) => None,
        }
    }

    /// Returns the column index if this selection includes one.
    pub const fn column(&self) -> Option<usize> {
        match *self {
            Self::Column(column) | Self::Cell { column, .. } => Some(column),
            Self::Row(_) => None,
        }
    }

    /// Returns a new selection with the row component set to `row`.
    ///
    /// The single place all row-component transition logic lives. Private: the transition table
    /// is documented on the public methods that use it ([`TableState::select_row`]).
    ///
    /// | current      | `row = Some(r)` | `row = None`  |
    /// |--------------|-----------------|---------------|
    /// | `None`       | `Row(r)`        | `None`        |
    /// | `Row(_)`     | `Row(r)`        | `None`        |
    /// | `Column(c)`  | `Cell{r, c}`    | `Column(c)`   |
    /// | `Cell{_, c}` | `Cell{r, c}`    | `Column(c)`   |
    pub(crate) const fn with_row(selection: Option<Self>, row: Option<usize>) -> Option<Self> {
        match row {
            Some(r) => Some(match selection {
                Some(Self::Column(c) | Self::Cell { column: c, .. }) => {
                    Self::Cell { row: r, column: c }
                }
                _ => Self::Row(r),
            }),
            None => match selection {
                Some(Self::Column(c) | Self::Cell { column: c, .. }) => Some(Self::Column(c)),
                _ => None,
            },
        }
    }

    /// Returns a new selection with the column component set to `column`.
    ///
    /// Mirror image of [`Self::with_row`].
    ///
    /// | current      | `column = Some(c)` | `column = None` |
    /// |--------------|--------------------|-----------------|
    /// | `None`       | `Column(c)`        | `None`          |
    /// | `Row(r)`     | `Cell{r, c}`       | `Row(r)` (kept) |
    /// | `Column(_)`  | `Column(c)`        | `None`          |
    /// | `Cell{r, _}` | `Cell{r, c}`       | `Row(r)`        |
    pub(crate) const fn with_column(
        selection: Option<Self>,
        column: Option<usize>,
    ) -> Option<Self> {
        match column {
            Some(c) => Some(match selection {
                Some(Self::Row(r) | Self::Cell { row: r, .. }) => Self::Cell { row: r, column: c },
                _ => Self::Column(c),
            }),
            None => match selection {
                Some(Self::Row(r) | Self::Cell { row: r, .. }) => Some(Self::Row(r)),
                _ => None,
            },
        }
    }
}

impl TableState {
    /// Creates a new [`TableState`] with no selection and offset 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let state = TableState::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            offset: 0,
            selection: None,
        }
    }

    /// Sets the index of the first row to be displayed
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let state = TableState::new().with_offset(1);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the selected row, preserving any existing column selection (fluent).
    ///
    /// Assigns the selection directly — does **not** reset `offset` even when passed `None`. Use
    /// [`select_row`] if you need the offset reset on deselection.
    ///
    /// [`select_row`]: TableState::select_row
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let state = TableState::new().with_selected_row(3);
    /// assert_eq!(state.selection(), Some(TableSelection::Row(3)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selected_row<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selection = TableSelection::with_row(self.selection, selected.into());
        self
    }

    /// Sets the selected column, preserving any existing row selection (fluent).
    ///
    /// Assigns the selection directly — never resets `offset`.
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let state = TableState::new().with_selected_column(2);
    /// assert_eq!(state.selection(), Some(TableSelection::Column(2)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selected_column<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.selection = TableSelection::with_column(self.selection, selected.into());
        self
    }

    /// Sets the selected cell by (row, column) pair (fluent).
    ///
    /// Passing `None` clears both axes. Does **not** reset `offset` — use [`select_cell`] if you
    /// need that.
    ///
    /// [`select_cell`]: TableState::select_cell
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let state = TableState::new().with_selected_cell((1, 5));
    /// assert_eq!(state.selected_cell(), Some((1, 5)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selected_cell<T>(mut self, selected: T) -> Self
    where
        T: Into<Option<(usize, usize)>>,
    {
        self.selection = selected
            .into()
            .map(|(row, column)| TableSelection::Cell { row, column });
        self
    }

    /// Replaces the whole selection (fluent).
    ///
    /// Does **not** preserve either axis. Does **not** reset `offset`.
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let state = TableState::new().with_selection(TableSelection::Cell { row: 1, column: 2 });
    /// assert_eq!(state.selected_cell(), Some((1, 2)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selection<T>(mut self, selection: T) -> Self
    where
        T: Into<Option<TableSelection>>,
    {
        self.selection = selection.into();
        self
    }

    /// Index of the first row to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let state = TableState::new();
    /// assert_eq!(state.offset(), 0);
    /// ```
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Mutable reference to the index of the first row to be displayed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let mut state = TableState::default();
    /// *state.offset_mut() = 1;
    /// ```
    pub const fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    /// The full selection, or `None` if nothing is selected.
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let state = TableState::new().with_selected_column(Some(1));
    /// assert_eq!(state.selection(), Some(TableSelection::Column(1)));
    /// ```
    pub const fn selection(&self) -> Option<TableSelection> {
        self.selection
    }

    /// Mutable reference to the full selection.
    ///
    /// Prefer the component setters for single-axis changes; reach for this only when replacing
    /// the entire value in one assignment.
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let mut state = TableState::new();
    /// *state.selection_mut() = Some(TableSelection::Row(2));
    /// assert_eq!(state.selected_row(), Some(2));
    /// ```
    pub const fn selection_mut(&mut self) -> &mut Option<TableSelection> {
        &mut self.selection
    }

    /// Selected row index, or `None` if no row is part of the current selection.
    ///
    /// Returns `None` for a column-only selection.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// assert_eq!(TableState::new().selected_row(), None);
    /// ```
    pub const fn selected_row(&self) -> Option<usize> {
        match self.selection {
            Some(selection) => selection.row(),
            None => None,
        }
    }

    /// Selected column index, or `None` if no column is part of the current selection.
    ///
    /// Returns `None` for a row-only selection.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// assert_eq!(TableState::new().selected_column(), None);
    /// ```
    pub const fn selected_column(&self) -> Option<usize> {
        match self.selection {
            Some(selection) => selection.column(),
            None => None,
        }
    }

    /// Selected (row, column) pair, or `None` if the selection is not a [`Cell`].
    ///
    /// Returns `None` for row-only and column-only selections.
    ///
    /// [`Cell`]: TableSelection::Cell
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// assert_eq!(TableState::new().selected_cell(), None);
    /// ```
    pub const fn selected_cell(&self) -> Option<(usize, usize)> {
        match self.selection {
            Some(TableSelection::Cell { row, column }) => Some((row, column)),
            _ => None,
        }
    }

    /// Replaces the whole selection.
    ///
    /// Does **not** preserve either axis and does **not** reset `offset`.
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let mut state = TableState::new();
    ///
    /// state.set_selection(Some(TableSelection::Cell { row: 0, column: 1 }));
    /// assert_eq!(state.selected_cell(), Some((0, 1)));
    /// ```
    pub fn set_selection<T>(&mut self, selection: T)
    where
        T: Into<Option<TableSelection>>,
    {
        self.selection = selection.into();
    }

    /// Sets the selected row, preserving any existing column selection.
    ///
    /// This is a *component* setter: it changes the row axis and leaves the column axis alone, so
    /// it can never produce a [`Column`] out of `Some`. Passing `None` clears the row axis, which
    /// demotes a cell selection to column-only, and resets `offset` to 0. Use [`set_selection`] to
    /// replace the whole selection instead.
    ///
    /// | current      | `row = Some(r)` | `row = None` |
    /// |--------------|-----------------|--------------|
    /// | `None`       | `Row(r)`        | `None`       |
    /// | `Row(_)`     | `Row(r)`        | `None`       |
    /// | `Column(c)`  | `Cell{r, c}`    | `Column(c)`  |
    /// | `Cell{_, c}` | `Cell{r, c}`    | `Column(c)`  |
    ///
    /// [`Column`]: TableSelection::Column
    /// [`set_selection`]: TableState::set_selection
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let mut state = TableState::new().with_selected_column(Some(3));
    ///
    /// state.select_row(Some(1));
    /// assert_eq!(
    ///     state.selection(),
    ///     Some(TableSelection::Cell { row: 1, column: 3 })
    /// );
    /// ```
    pub const fn select_row(&mut self, row: Option<usize>) {
        if row.is_none() {
            self.offset = 0;
        }
        self.selection = TableSelection::with_row(self.selection, row);
    }

    /// Sets the selected column, preserving any existing row selection.
    ///
    /// The mirror image of [`select_row`], except that it does **not** reset `offset` — the
    /// column axis does not affect vertical scrolling.
    ///
    /// | current      | `column = Some(c)` | `column = None` |
    /// |--------------|--------------------|-----------------|
    /// | `None`       | `Column(c)`        | `None`          |
    /// | `Row(r)`     | `Cell{r, c}`       | `Row(r)`        |
    /// | `Column(_)`  | `Column(c)`        | `None`          |
    /// | `Cell{r, _}` | `Cell{r, c}`       | `Row(r)`        |
    ///
    /// [`select_row`]: TableState::select_row
    ///
    /// ```rust
    /// use ratatui_table::{TableSelection, TableState};
    ///
    /// let mut state = TableState::new().with_selected_row(Some(5));
    ///
    /// state.select_column(Some(2));
    /// assert_eq!(
    ///     state.selection(),
    ///     Some(TableSelection::Cell { row: 5, column: 2 })
    /// );
    /// ```
    pub const fn select_column(&mut self, column: Option<usize>) {
        self.selection = TableSelection::with_column(self.selection, column);
    }

    /// Sets the selected cell by (row, column) pair.
    ///
    /// Passing `None` clears both axes and resets `offset` to 0.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    ///
    /// let mut state = TableState::new();
    ///
    /// state.select_cell(Some((1, 5)));
    /// assert_eq!(state.selected_cell(), Some((1, 5)));
    /// ```
    pub const fn select_cell(&mut self, indexes: Option<(usize, usize)>) {
        if let Some((row, column)) = indexes {
            self.selection = Some(TableSelection::Cell { row, column });
        } else {
            self.offset = 0;
            self.selection = None;
        }
    }

    /// Selects the next row, or row 0 if no row is currently selected.
    ///
    /// The index may exceed the actual row count; it is clamped on the next render.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    /// let mut state = TableState::new();
    /// state.select_next_row();
    /// assert_eq!(state.selected_row(), Some(0));
    /// ```
    pub fn select_next_row(&mut self) {
        let next = self.selected_row().map_or(0, |i| i.saturating_add(1));
        self.select_row(Some(next));
    }

    /// Selects the previous row, or `usize::MAX` (clamped on render) if no row is selected.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    /// let mut state = TableState::new();
    /// state.select_previous_row();
    /// assert_eq!(state.selected_row(), Some(usize::MAX));
    /// ```
    pub fn select_previous_row(&mut self) {
        let prev = self
            .selected_row()
            .map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select_row(Some(prev));
    }

    /// Selects the next column, or column 0 if no column is currently selected.
    ///
    /// The index may exceed the actual column count; it is clamped on the next render.
    pub fn select_next_column(&mut self) {
        let next = self.selected_column().map_or(0, |i| i.saturating_add(1));
        self.select_column(Some(next));
    }

    /// Selects the previous column, or `usize::MAX` (clamped on render) if no column is selected.
    pub fn select_previous_column(&mut self) {
        let prev = self
            .selected_column()
            .map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select_column(Some(prev));
    }

    /// Selects the first row (index 0), preserving any existing column selection.
    pub const fn select_first_row(&mut self) {
        self.selection = TableSelection::with_row(self.selection, Some(0));
    }

    /// Selects the last row (`usize::MAX`, clamped on render), preserving any column selection.
    pub const fn select_last_row(&mut self) {
        self.selection = TableSelection::with_row(self.selection, Some(usize::MAX));
    }

    /// Selects the first column (index 0), preserving any existing row selection.
    pub const fn select_first_column(&mut self) {
        self.selection = TableSelection::with_column(self.selection, Some(0));
    }

    /// Selects the last column (`usize::MAX`, clamped on render), preserving any row selection.
    pub const fn select_last_column(&mut self) {
        self.selection = TableSelection::with_column(self.selection, Some(usize::MAX));
    }

    /// Moves the row selection down by `amount`, saturating at `usize::MAX`.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    /// let mut state = TableState::new();
    /// state.select_row(Some(2));
    /// state.scroll_down_by(4);
    /// assert_eq!(state.selected_row(), Some(6));
    /// ```
    pub fn scroll_down_by(&mut self, amount: u16) {
        let next = self
            .selected_row()
            .unwrap_or(0)
            .saturating_add(amount as usize);
        self.select_row(Some(next));
    }

    /// Moves the row selection up by `amount`, saturating at 0.
    ///
    /// ```rust
    /// use ratatui_table::TableState;
    /// let mut state = TableState::new();
    /// state.select_row(Some(6));
    /// state.scroll_up_by(4);
    /// assert_eq!(state.selected_row(), Some(2));
    /// ```
    pub fn scroll_up_by(&mut self, amount: u16) {
        let prev = self
            .selected_row()
            .unwrap_or(0)
            .saturating_sub(amount as usize);
        self.select_row(Some(prev));
    }

    /// Moves the column selection right by `amount`, saturating at `usize::MAX`.
    pub fn scroll_right_by(&mut self, amount: u16) {
        let next = self
            .selected_column()
            .unwrap_or(0)
            .saturating_add(amount as usize);
        self.select_column(Some(next));
    }

    /// Moves the column selection left by `amount`, saturating at 0.
    pub fn scroll_left_by(&mut self, amount: u16) {
        let prev = self
            .selected_column()
            .unwrap_or(0)
            .saturating_sub(amount as usize);
        self.select_column(Some(prev));
    }

    /// Use [`selected_row`] instead.
    ///
    /// [`selected_row`]: TableState::selected_row
    #[deprecated(since = "0.2.0", note = "renamed to `selected_row`")]
    pub const fn selected(&self) -> Option<usize> {
        self.selected_row()
    }

    /// Use [`select_row`] instead.
    ///
    /// [`select_row`]: TableState::select_row
    #[deprecated(since = "0.2.0", note = "renamed to `select_row`")]
    pub const fn select(&mut self, index: Option<usize>) {
        self.select_row(index);
    }

    /// Use [`with_selected_row`] instead.
    ///
    /// [`with_selected_row`]: TableState::with_selected_row
    #[deprecated(since = "0.2.0", note = "renamed to `with_selected_row`")]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_selected<T>(self, selected: T) -> Self
    where
        T: Into<Option<usize>>,
    {
        self.with_selected_row(selected)
    }

    /// Use [`select_next_row`] instead.
    ///
    /// [`select_next_row`]: TableState::select_next_row
    #[deprecated(since = "0.2.0", note = "renamed to `select_next_row`")]
    pub fn select_next(&mut self) {
        self.select_next_row();
    }

    /// Use [`select_previous_row`] instead.
    ///
    /// [`select_previous_row`]: TableState::select_previous_row
    #[deprecated(since = "0.2.0", note = "renamed to `select_previous_row`")]
    pub fn select_previous(&mut self) {
        self.select_previous_row();
    }

    /// Use [`select_first_row`] instead.
    ///
    /// [`select_first_row`]: TableState::select_first_row
    #[deprecated(since = "0.2.0", note = "renamed to `select_first_row`")]
    pub const fn select_first(&mut self) {
        self.select_first_row();
    }

    /// Use [`select_last_row`] instead.
    ///
    /// [`select_last_row`]: TableState::select_last_row
    #[deprecated(since = "0.2.0", note = "renamed to `select_last_row`")]
    pub const fn select_last(&mut self) {
        self.select_last_row();
    }

    /// Clamps the selection to valid indices and adjusts the offset so the selected row is visible.
    ///
    /// Called by the renderer once it knows `row_count` and `column_count`. Sentinel values
    /// (`usize::MAX` written by `select_last_row` / `select_last_column`) are rewritten to the
    /// actual last index.
    pub(crate) fn clamp(&mut self, row_count: usize, column_count: usize) {
        // Zero rows/columns drop that axis entirely; a `Cell` demotes rather than vanishing.
        // Matches the seed, which called `select(None)` / `select_column(None)` here.
        if row_count == 0 {
            self.select_row(None);
        } else {
            let last = row_count - 1;
            self.selection = TableSelection::with_row(
                self.selection,
                self.selected_row().map(|row| row.min(last)),
            );
        }

        if column_count == 0 {
            self.select_column(None);
        } else {
            let last = column_count - 1;
            self.selection = TableSelection::with_column(
                self.selection,
                self.selected_column().map(|column| column.min(last)),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let state = TableState::new();
        assert_eq!(state.offset, 0);
        assert_eq!(state.selection, None);
    }

    #[test]
    fn with_offset() {
        assert_eq!(TableState::new().with_offset(1).offset, 1);
    }

    #[test]
    fn with_selected_row_some() {
        let s = TableState::new().with_selected_row(Some(1));
        assert_eq!(s.selection, Some(TableSelection::Row(1)));
    }

    #[test]
    fn with_selected_row_preserves_column() {
        let s = TableState::new()
            .with_selected_column(Some(2))
            .with_selected_row(Some(1));
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 1, column: 2 })
        );
    }

    #[test]
    fn with_selected_row_none_does_not_reset_offset() {
        // Builder must not route through select_row — offset must stay put.
        let s = TableState::new().with_offset(5).with_selected_row(None);
        assert_eq!(s.offset, 5);
        assert_eq!(s.selection, None);
    }

    #[test]
    fn with_selected_column_some() {
        let s = TableState::new().with_selected_column(Some(1));
        assert_eq!(s.selection, Some(TableSelection::Column(1)));
    }

    #[test]
    fn with_selected_column_preserves_row() {
        let s = TableState::new()
            .with_selected_row(Some(3))
            .with_selected_column(Some(1));
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 3, column: 1 })
        );
    }

    #[test]
    fn with_selected_cell_some() {
        let s = TableState::new().with_selected_cell(Some((1, 5)));
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 1, column: 5 })
        );
    }

    #[test]
    fn with_selected_cell_none() {
        let s = TableState::new().with_offset(4).with_selected_cell(None);
        assert_eq!(s.selection, None);
        // with_selected_cell(None) clears selection but does NOT reset offset
        assert_eq!(s.offset, 4);
    }

    #[test]
    fn with_selection_replaces_whole() {
        let sel = TableSelection::Cell { row: 2, column: 3 };
        let s = TableState::new().with_selection(Some(sel));
        assert_eq!(s.selection, Some(sel));
    }

    #[test]
    fn offset_getter_and_mut() {
        let mut s = TableState::new();
        assert_eq!(s.offset(), 0);
        *s.offset_mut() = 5;
        assert_eq!(s.offset(), 5);
    }

    #[test]
    fn selection_getter_and_mut() {
        let mut s = TableState::new();
        assert_eq!(s.selection(), None);
        *s.selection_mut() = Some(TableSelection::Row(7));
        assert_eq!(s.selection(), Some(TableSelection::Row(7)));
    }

    #[test]
    fn selected_row_only() {
        let s = TableState::new().with_selected_row(Some(3));
        assert_eq!(s.selected_row(), Some(3));
        assert_eq!(s.selected_column(), None);
        assert_eq!(s.selected_cell(), None);
    }

    #[test]
    fn selected_column_only() {
        let s = TableState::new().with_selected_column(Some(2));
        assert_eq!(s.selected_row(), None);
        assert_eq!(s.selected_column(), Some(2));
        assert_eq!(s.selected_cell(), None);
    }

    #[test]
    fn selected_cell_getters() {
        let s = TableState::new().with_selected_cell(Some((4, 7)));
        assert_eq!(s.selected_row(), Some(4));
        assert_eq!(s.selected_column(), Some(7));
        assert_eq!(s.selected_cell(), Some((4, 7)));
    }

    #[test]
    fn select_row_from_none() {
        let mut s = TableState::new();
        s.select_row(Some(2));
        assert_eq!(s.selection, Some(TableSelection::Row(2)));
    }

    #[test]
    fn select_row_upgrades_to_cell() {
        let mut s = TableState::new().with_selected_column(Some(3));
        s.select_row(Some(1));
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 1, column: 3 })
        );
    }

    #[test]
    fn select_row_none_from_cell_keeps_column() {
        let mut s = TableState::new().with_selected_cell(Some((2, 4)));
        s.select_row(None);
        assert_eq!(s.selection, Some(TableSelection::Column(4)));
        assert_eq!(s.offset, 0);
    }

    #[test]
    fn select_row_none_from_row_clears() {
        let mut s = TableState::new().with_selected_row(Some(5)).with_offset(3);
        s.select_row(None);
        assert_eq!(s.selection, None);
        assert_eq!(s.offset, 0);
    }

    #[test]
    fn select_column_from_none() {
        let mut s = TableState::new();
        s.select_column(Some(2));
        assert_eq!(s.selection, Some(TableSelection::Column(2)));
    }

    #[test]
    fn select_column_upgrades_to_cell() {
        let mut s = TableState::new().with_selected_row(Some(5));
        s.select_column(Some(2));
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 5, column: 2 })
        );
    }

    #[test]
    fn select_column_none_from_cell_keeps_row() {
        let mut s = TableState::new().with_selected_cell(Some((3, 7)));
        s.select_column(None);
        assert_eq!(s.selection, Some(TableSelection::Row(3)));
    }

    #[test]
    fn select_column_none_does_not_reset_offset() {
        let mut s = TableState::new()
            .with_offset(5)
            .with_selected_column(Some(2));
        s.select_column(None);
        assert_eq!(s.offset, 5);
    }

    #[test]
    fn select_cell_and_clear() {
        let mut s = TableState::new();
        s.select_cell(Some((1, 5)));
        assert_eq!(s.selected_cell(), Some((1, 5)));
        s.select_cell(None);
        assert_eq!(s.selection, None);
        assert_eq!(s.offset, 0);
    }

    #[test]
    fn set_selection_replaces_whole() {
        let mut s = TableState::new().with_selected_row(Some(10)).with_offset(3);
        s.set_selection(Some(TableSelection::Column(3)));
        assert_eq!(s.selection, Some(TableSelection::Column(3)));
        assert_eq!(s.selected_row(), None);
        // set_selection never touches offset
        assert_eq!(s.offset, 3);
    }

    #[test]
    fn navigation_rows() {
        let mut s = TableState::new();

        s.select_first_row();
        assert_eq!(s.selected_row(), Some(0));

        s.select_previous_row(); // saturates at 0
        assert_eq!(s.selected_row(), Some(0));

        s.select_next_row();
        assert_eq!(s.selected_row(), Some(1));

        s.select_last_row();
        assert_eq!(s.selected_row(), Some(usize::MAX));

        s.select_next_row(); // saturates at usize::MAX
        assert_eq!(s.selected_row(), Some(usize::MAX));

        s.select_previous_row();
        assert_eq!(s.selected_row(), Some(usize::MAX - 1));
    }

    #[test]
    fn select_next_row_from_empty() {
        let mut s = TableState::new();
        s.select_next_row();
        assert_eq!(s.selected_row(), Some(0));
    }

    #[test]
    fn select_previous_row_from_empty() {
        let mut s = TableState::new();
        s.select_previous_row();
        assert_eq!(s.selected_row(), Some(usize::MAX));
    }

    #[test]
    fn first_last_row_preserve_column() {
        let mut s = TableState::new().with_selected_column(Some(4));
        s.select_first_row();
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 0, column: 4 })
        );
        s.select_last_row();
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell {
                row: usize::MAX,
                column: 4
            })
        );
    }

    #[test]
    fn navigation_columns() {
        let mut s = TableState::new();

        s.select_first_column();
        assert_eq!(s.selected_column(), Some(0));

        s.select_previous_column(); // saturates at 0
        assert_eq!(s.selected_column(), Some(0));

        s.select_next_column();
        assert_eq!(s.selected_column(), Some(1));

        s.select_last_column();
        assert_eq!(s.selected_column(), Some(usize::MAX));

        s.select_previous_column();
        assert_eq!(s.selected_column(), Some(usize::MAX - 1));
    }

    #[test]
    fn first_last_column_preserve_row() {
        let mut s = TableState::new().with_selected_row(Some(2));
        s.select_first_column();
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell { row: 2, column: 0 })
        );
        s.select_last_column();
        assert_eq!(
            s.selection,
            Some(TableSelection::Cell {
                row: 2,
                column: usize::MAX
            })
        );
    }

    #[test]
    fn scroll_down_up() {
        let mut s = TableState::new();
        s.select_row(Some(2));
        s.scroll_down_by(4);
        assert_eq!(s.selected_row(), Some(6));

        s.scroll_up_by(4);
        assert_eq!(s.selected_row(), Some(2));

        s.scroll_up_by(10); // saturates at 0
        assert_eq!(s.selected_row(), Some(0));
    }

    #[test]
    fn scroll_right_left() {
        let mut s = TableState::new().with_selected_column(Some(12));
        s.scroll_right_by(4);
        assert_eq!(s.selected_column(), Some(16));

        s.scroll_left_by(20); // saturates at 0
        assert_eq!(s.selected_column(), Some(0));

        s.scroll_right_by(100);
        assert_eq!(s.selected_column(), Some(100));

        s.scroll_left_by(20);
        assert_eq!(s.selected_column(), Some(80));
    }

    #[test]
    fn clamp_row() {
        let mut s = TableState::new().with_selected_row(Some(usize::MAX));
        s.clamp(5, 3);
        assert_eq!(s.selected_row(), Some(4));
    }

    #[test]
    fn clamp_column() {
        let mut s = TableState::new().with_selected_column(Some(usize::MAX));
        s.clamp(5, 3);
        assert_eq!(s.selected_column(), Some(2));
    }

    #[test]
    fn clamp_cell() {
        let mut s = TableState::new().with_selected_cell(Some((usize::MAX, usize::MAX)));
        s.clamp(5, 3);
        assert_eq!(s.selected_cell(), Some((4, 2)));
    }

    #[test]
    fn clamp_zero_rows_drops_row_and_resets_offset() {
        let mut s = TableState::new().with_offset(10).with_selected_row(Some(2));
        s.clamp(0, 1);
        assert_eq!(s.selection, None);
        assert_eq!(s.offset, 0);
    }

    #[test]
    fn clamp_zero_rows_demotes_cell_to_column() {
        let mut s = TableState::new().with_selected_cell((2, 1));
        s.clamp(0, 3);
        assert_eq!(s.selection, Some(TableSelection::Column(1)));
    }

    #[test]
    fn clamp_zero_columns_demotes_cell_to_row() {
        let mut s = TableState::new().with_selected_cell((2, 1));
        s.clamp(5, 0);
        assert_eq!(s.selection, Some(TableSelection::Row(2)));
    }

    #[test]
    fn clamp_leaves_offset_alone_when_in_range() {
        let mut s = TableState::new().with_offset(10).with_selected_row(Some(2));
        s.clamp(5, 1);
        assert_eq!(s.offset, 10);
    }

    #[test]
    fn usize_converts_to_row() {
        assert_eq!(TableSelection::from(3), TableSelection::Row(3));
        assert_eq!(Into::<TableSelection>::into(5usize), TableSelection::Row(5));
    }

    #[test]
    fn row_accessor() {
        assert_eq!(TableSelection::Row(2).row(), Some(2));
        assert_eq!(TableSelection::Cell { row: 2, column: 4 }.row(), Some(2));
        assert_eq!(TableSelection::Column(4).row(), None);
    }

    #[test]
    fn column_accessor() {
        assert_eq!(TableSelection::Column(4).column(), Some(4));
        assert_eq!(TableSelection::Cell { row: 2, column: 4 }.column(), Some(4));
        assert_eq!(TableSelection::Row(2).column(), None);
    }

    /// The renamed methods still accept a bare `usize` where the old ones did, so signature-
    /// compatible call sites keep working without going through the deprecated aliases.
    #[test]
    fn builders_accept_bare_indices() {
        let state = TableState::new()
            .with_selected_row(3)
            .with_selected_column(1);
        assert_eq!(state.selected_cell(), Some((3, 1)));
        assert_eq!(
            TableState::new().with_selected_cell((2, 4)).selected_cell(),
            Some((2, 4))
        );
        assert_eq!(
            TableState::new()
                .with_selection(TableSelection::Row(1))
                .selection(),
            Some(TableSelection::Row(1))
        );
    }

    #[expect(deprecated)]
    #[test]
    fn deprecated_aliases_match_their_replacements() {
        // Each alias must be observationally identical to the method it forwards to, including
        // the offset reset that `select(None)` has always performed.
        let mut old = TableState::new().with_selected(2);
        let mut new = TableState::new().with_selected_row(2);
        assert_eq!(old.selected(), new.selected_row());

        for (apply_old, apply_new) in [
            (
                TableState::select_next as fn(&mut TableState),
                TableState::select_next_row as fn(&mut TableState),
            ),
            (TableState::select_previous, TableState::select_previous_row),
            (TableState::select_first, TableState::select_first_row),
            (TableState::select_last, TableState::select_last_row),
        ] {
            apply_old(&mut old);
            apply_new(&mut new);
            assert_eq!(old, new);
        }

        old.select(None);
        new.select_row(None);
        assert_eq!(old, new);
    }
}
