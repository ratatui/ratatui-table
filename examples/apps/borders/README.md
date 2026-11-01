# Borders demo

A table that cycles through [`TableBorders`] presets and glyph sets. Use it to see where the lines
sit in the column gap and how the selection draws across them.

```shell
cargo run -p borders
```

| key               | effect                                   |
| ----------------- | ---------------------------------------- |
| `↑` `↓` / `k` `j` | move the row selection                   |
| `b`               | cycle the [`TableBorders`] presets       |
| `h`               | toggle `TableBorders::INNER_HORIZONTAL`  |
| `s`               | cycle the [`line::Set`] the lines use    |
| `-` `+`           | narrow or widen the column spacing       |
| `q`               | quit                                     |

The footer spans every column, so the lines above it end in `┴` junctions. Under
`INNER_VERTICAL` a spacing of 0 lays out like 1, since the line needs a cell to sit in.

[`TableBorders`]: https://docs.rs/ratatui-table/latest/ratatui_table/struct.TableBorders.html
[`line::Set`]: https://docs.rs/ratatui/latest/ratatui/symbols/line/struct.Set.html
