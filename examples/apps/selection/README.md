# Selection demo

Row, column, and cell selection, the highlight style of each axis, and the two settings that
decide where the highlight symbol gets its space.

```shell
cargo run -p selection
```

| key | effect |
| --- | --- |
| `↑` `↓` / `k` `j` | move the row selection |
| `←` `→` / `h` `l` | move the column selection |
| `r` `c` `x` `n` | select a row, a column, a cell, or nothing |
| `p` | cycle [`HighlightPlacement`] |
| `s` | cycle [`HighlightSpacing`] |
| `q` | quit |

The selection starts on the second column: `SelectedColumn` and `FirstColumn` are the same layout
whenever the first column is the selected one.

[`HighlightPlacement`]: https://docs.rs/ratatui-table/latest/ratatui_table/enum.HighlightPlacement.html
[`HighlightSpacing`]: https://docs.rs/ratatui-table/latest/ratatui_table/enum.HighlightSpacing.html
