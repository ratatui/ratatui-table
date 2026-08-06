use alloc::vec::Vec;

use strum::{Display, EnumString};

/// Where the highlight symbol gets space, i.e. *which* columns are indented to make room for it.
///
/// [`HighlightSpacing`] decides *when* the symbol column is allocated; this decides *where*.
///
/// [`HighlightSpacing`]: super::HighlightSpacing
///
/// # How the layout moves
///
/// Every column that gets a slot is pushed right by the symbol width, and the remaining width is
/// split between the columns. With a 2-cell symbol `>>` and three columns:
///
/// ```text
/// FirstColumn      >>Alice   Engineer  Berlin
/// SelectedColumn     Alice >>Engineer  Berlin      (column 1 selected)
/// SelectedColumn     Alice   Engineer>>Berlin      (column 2 selected)
/// AllColumns       >>Alice >>Engineer>>Berlin
/// ```
///
/// Note the consequence of [`SelectedColumn`]: moving the selection from one column to the next
/// moves the slot with it, so text to the left of the new slot shifts left and text to its right
/// shifts right. Use [`FirstColumn`] (the default) or [`AllColumns`] if you need the columns to
/// stay put as the selection moves.
///
/// [`SelectedColumn`]: HighlightPlacement::SelectedColumn
/// [`FirstColumn`]: HighlightPlacement::FirstColumn
/// [`AllColumns`]: HighlightPlacement::AllColumns
#[derive(Debug, Display, EnumString, PartialEq, Eq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HighlightPlacement {
    /// Only the first column gets a slot.
    ///
    /// This is the default, and reproduces the behavior of a table without column selection: the
    /// symbol sits at the left edge of the row and the layout never moves.
    #[default]
    FirstColumn,

    /// Only the selected column gets a slot.
    ///
    /// The slot follows the selection, so the columns shift as the selected column changes. With
    /// no column selected, falls back to the first column.
    SelectedColumn,

    /// Each listed column gets a slot.
    ///
    /// Indices at or beyond the column count are ignored. Parsing this variant from a string
    /// yields an empty list, i.e. no slots.
    Columns(Vec<usize>),

    /// Every column gets a slot.
    AllColumns,
}

impl HighlightPlacement {
    /// Returns a `column_count`-long mask: `mask[i]` is true when column `i` gets a symbol slot.
    ///
    /// A mask rather than a list of indices because the render path asks "does column `i` have a
    /// slot" once per column; a `contains()` lookup inside that loop would be quadratic.
    pub(crate) fn column_mask(
        &self,
        selected_column: Option<usize>,
        column_count: usize,
    ) -> Vec<bool> {
        let mut mask = alloc::vec![false; column_count];
        match self {
            Self::AllColumns => mask.fill(true),
            Self::FirstColumn => {
                if let Some(first) = mask.first_mut() {
                    *first = true;
                }
            }
            Self::SelectedColumn => {
                // No column selected: behave like `FirstColumn` rather than showing no symbol.
                if let Some(slot) = mask.get_mut(selected_column.unwrap_or(0)) {
                    *slot = true;
                }
            }
            Self::Columns(indices) => {
                for &index in indices {
                    if let Some(slot) = mask.get_mut(index) {
                        *slot = true;
                    }
                }
            }
        }
        mask
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;

    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(
            HighlightPlacement::FirstColumn.to_string(),
            "FirstColumn".to_string()
        );
        assert_eq!(
            HighlightPlacement::SelectedColumn.to_string(),
            "SelectedColumn".to_string()
        );
        assert_eq!(
            HighlightPlacement::Columns(vec![1]).to_string(),
            "Columns".to_string()
        );
        assert_eq!(
            HighlightPlacement::AllColumns.to_string(),
            "AllColumns".to_string()
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "FirstColumn".parse::<HighlightPlacement>(),
            Ok(HighlightPlacement::FirstColumn)
        );
        assert_eq!(
            "SelectedColumn".parse::<HighlightPlacement>(),
            Ok(HighlightPlacement::SelectedColumn)
        );
        assert_eq!(
            "Columns".parse::<HighlightPlacement>(),
            Ok(HighlightPlacement::Columns(Vec::new()))
        );
        assert_eq!(
            "AllColumns".parse::<HighlightPlacement>(),
            Ok(HighlightPlacement::AllColumns)
        );
        assert_eq!(
            "".parse::<HighlightPlacement>(),
            Err(strum::ParseError::VariantNotFound)
        );
    }

    #[test]
    fn mask_first_column() {
        let mask = HighlightPlacement::FirstColumn.column_mask(Some(2), 3);
        assert_eq!(mask, vec![true, false, false]);
    }

    #[test]
    fn mask_selected_column() {
        let mask = HighlightPlacement::SelectedColumn.column_mask(Some(2), 3);
        assert_eq!(mask, vec![false, false, true]);
    }

    #[test]
    fn mask_selected_column_without_selection() {
        let mask = HighlightPlacement::SelectedColumn.column_mask(None, 3);
        assert_eq!(mask, vec![true, false, false]);
    }

    #[test]
    fn mask_columns_ignores_out_of_range() {
        let mask = HighlightPlacement::Columns(vec![1, 9]).column_mask(None, 3);
        assert_eq!(mask, vec![false, true, false]);
    }

    #[test]
    fn mask_all_columns() {
        let mask = HighlightPlacement::AllColumns.column_mask(None, 3);
        assert_eq!(mask, vec![true, true, true]);
    }

    #[test]
    fn mask_zero_columns() {
        assert!(
            HighlightPlacement::AllColumns
                .column_mask(Some(0), 0)
                .is_empty()
        );
        assert!(
            HighlightPlacement::FirstColumn
                .column_mask(None, 0)
                .is_empty()
        );
    }
}
