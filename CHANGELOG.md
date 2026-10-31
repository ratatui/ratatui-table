# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-10-31

### 🚀 Features

- *(table)* Add configurable highlight placement (#15)

- *(table)* Add lazy row support (#14)
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
  > tall by default.
  > - `Table::row_height(u16)` — uniform height override.
  > - `Table::row_height_with(impl Fn(usize) -> u16)` — per-row heights.
  >
  > All three are additive; no existing API changes behaviour.
  >
  > ## Why heights are declared separately
  >
  > The scroll pass needs the height of rows it will not draw — for instance
  > to work out where the window starts when the selection is below the
  > viewport. For an eager table that is free, because `visible_rows` can
  > index `self.rows`. For a lazy table, asking a row for its height means
  > building it, which is the cost the feature exists to avoid. Declaring
  > heights separately keeps the whole frame proportional to the viewport,
  > and matches how other virtualised lists work (Qt `sizeHintForRow`, UIKit
  > `heightForRowAt`, react-window `itemSize`).
  >
  > Consequence, documented on the constructor: for lazy rows the declared
  > height is authoritative and `Row::height` / `Row::top_margin` /
  > `Row::bottom_margin` on the returned row are ignored.
  >
  > ## Consistency with existing code
  >
  > `LazyRowHeights` follows the same shape as `Effect` in
  > `block/shadow.rs`: an enum with cheap built-in variants plus one
  > `Arc<dyn …>` variant, with hand-written `PartialEq` (`Arc::ptr_eq`),
  > `Eq` and `Hash` (`ptr::hash`) so the widget keeps its derives.
  >
  > It differs in one respect: `CellEffect` requires `Send + Sync +
  > UnwindSafe + RefUnwindSafe`, whereas the lazy closures require only
  > `Send + Sync + RefUnwindSafe`. `Arc<T>: UnwindSafe` needs just `T:
  > RefUnwindSafe`, so the extra bound would restrict callers without buying
  > anything — `Table<'static>` remains `UnwindSafe`, as
  > `ratatui-widgets/tests/auto_traits.rs` already asserts. Happy to add
  > `UnwindSafe` for symmetry if you would rather the two APIs read the
  > same.
  >
  > ## Please look closely at
  >
  > 1. **`LazyRowProvider::visible_rows`** mirrors `Table::visible_rows`
  > step for step, but grows the window upward from the selection instead of
  > scrolling down one row at a time. Two tests assert the two produce
  > identical buffers and offsets across a matrix of row heights, area
  > heights, offsets and selections.
  > 2. **One deliberate divergence**: when the selected row is taller than
  > the whole area, the eager algorithm shrinks `start` past the selection
  > and draws the row *below* it, leaving the selected row off screen. The
  > lazy path keeps it anchored and clips it. I left the eager behaviour
  > alone and documented the difference — happy to open a separate fix if
  > you agree it is a bug.
  > 3. **`row_height` / `row_height_with` are no-ops on eager tables**,
  > since `Table::new` rows carry their own `Row::height`. Documented on
  > both methods and covered by a test. Open to a better name or shape here.
  >
  > ## Testing
  >
  > - New `tests/lazy_rows.rs`: laziness (asserting the exact set of rows
  > built), offsets, selection anchoring, partial rows,
  > headers/footers/blocks, highlight styles, zero-height rows,
  > clone/eq/hash, and equivalence with `Table::new`.
  > - `just check-all` passes locally.
  >
  > ## AI assistance
  > This uses AI for comments and logic validation.
  >
  > ---------

### 🐛 Bug Fixes

- Render changelog whitespace correctly
  > Use git-cliff whitespace controls instead of emitting literal escape
  > characters, and clean the initial checked changelog.

### ⚙️ Miscellaneous Tasks

- Update justfile to include default list (#16)
  > With recent versions of just, you can remove the default recipe when you
  > are using it for a list with this syntax. The advantage is it doesn't
  > show the default recipe as an option. Source:
  > https://just.systems/man/en/the-default-recipe.html
  >
  > Alternatively, you can add the `[private]` attribute:
  >
  > ```
  > [private]
  > default:
  >     @just --list
  > ```
  >
  > But the syntax in the PR is nicer imo.


## [0.1.0] - 2026-07-31

### 🚀 Features

- Seed the Ratatui table experiment
  > Extract the current built-in Table into an independently governed crate
  > while preserving its initial API and rendering behavior.

### 🐛 Bug Fixes

- Prepare minor releases before 1.0
