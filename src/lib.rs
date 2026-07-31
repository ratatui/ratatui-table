#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
//! An experimental evolution of Ratatui's built-in [`Table`] widget.
//!
//! `ratatui-table` starts with the Table implementation from Ratatui main at
//! [`3d8639c`]. Version 0.1.0 is intended to render like that built-in Table while giving Table
//! contributors room to explore borders, virtualization, selection, marking, borrowed data, and
//! other changes independently.
//!
//! [`3d8639c`]: https://github.com/ratatui/ratatui/commit/3d8639cbb2f910200f30e680a8923ccaf99ba1bf
//!
//! This crate is an opt-in development path for the built-in widget, not a competing Table design.
//! If the experiment succeeds, Ratatui should be able to offer an easy upgrade or re-export path.
//! No such re-export is implemented or promised yet.
//!
//! # Stability
//!
//! Releases before 1.0 may make breaking changes in minor versions. Each breaking release should
//! document how to migrate. The 0.1.0 baseline deliberately avoids folding the existing Table
//! proposals into the initial extraction so each proposal remains independently reviewable.
//!
//! # Installation
//!
//! ```shell
//! cargo add ratatui-table
//! ```
//!
//! # Example
//!
//! ```rust
//! use ratatui::layout::Constraint;
//! use ratatui_table::{Row, Table};
//!
//! let rows = [
//!     Row::new(["Alice", "Engineer"]),
//!     Row::new(["Bob", "Designer"]),
//! ];
//! let widths = [Constraint::Length(10), Constraint::Length(12)];
//! let table = Table::new(rows, widths);
//! ```
//!
//! # Ratatui compatibility
//!
//! [`Table::block`] accepts [`ratatui_widgets::block::Block`] to preserve the built-in API. This
//! means the initial crate depends on `ratatui-widgets` with default features disabled. A future
//! Ratatui facade can re-export this crate without requiring this crate to become part of
//! `ratatui-widgets` itself.
//!
//! # Feature flags
//!
//! - `std` enables the standard-library features of the Ratatui dependencies. The Table itself
//!   remains usable with `no_std` plus `alloc` by default.
//! - `serde` adds serialization and deserialization for [`TableState`] and [`HighlightSpacing`].
//!
//! # Governance
//!
//! Contributors with repository Write access can review and merge ordinary Table work. Ratatui
//! maintainers retain control of protected release configuration and approve every crates.io
//! publication.

extern crate alloc;

mod table;

#[doc(inline)]
pub use table::{Cell, HighlightSpacing, Row, Table, TableState};
