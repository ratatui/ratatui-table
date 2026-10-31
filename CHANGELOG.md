# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-10-31

### 🚀 Features

- _(table)_ Add configurable highlight placement (#15)

- _(table)_ Add lazy row support (#14)
  > Closes ratatui/ratatui-table#5
  >
  > ## What
  >
  > Adds `Table::lazy_rows`, which builds rows on demand instead of
  > collecting them into a `Vec` up front, so a table over a very large
  > dataset costs the viewport rather than the dataset.
  >
  > ```rust
  > let table = Table::lazy_rows(10_000_000, widths, |index| Row::new([format!("item {index}")]))
  >     .row_height_with(|index| if index % 25 == 0 { 3 } else { 1 });
  > ```
  >
  > - `Table::lazy_rows(row_count, widths, factory)` — rows are one line
  >   tall by default.
  > - `Table::row_height(u16)` — uniform height override.
  > - `Table::row_height_with(impl Fn(usize) -> u16)` — per-row heights.

### 🐛 Bug Fixes

- Render changelog whitespace correctly

### ⚙️ Miscellaneous Tasks

- Update justfile to include default list (#16)

## [0.1.0] - 2026-07-31

### 🚀 Features

- Seed the Ratatui table experiment
  > Extract the current built-in Table into an independently governed crate
  > while preserving its initial API and rendering behavior.

### 🐛 Bug Fixes

- Prepare minor releases before 1.0
