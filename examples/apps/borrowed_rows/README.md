# Borrowed rows demo

A package index of 200,000 rows. The app builds the rows and the header once, keeps them in its
state, and borrows them into a new `Table` on every frame. The status line shows how long the last
draw took.

```shell
cargo run -p borrowed_rows
```

| key                    | effect                                  |
| ---------------------- | --------------------------------------- |
| `↑` `↓` / `k` `j`      | move the selection one row              |
| `PgUp` `PgDn`          | scroll twenty rows                      |
| `g` `G` / `Home` `End` | jump to the first or last row           |
| `c`                    | switch between borrowing and cloning    |
| `q`                    | quit                                    |

[`Table::from`] borrows the rows and `Row::from(&header)` borrows the header, so a frame copies
nothing. In a release build a frame takes about 80 µs.

Press `c` to clone the rows into the table instead, which is what `Table::new` needs. Every frame
then copies 200,000 rows and their strings, and the draw takes about 250 ms in a release build.
Moving the selection starts to lag. Press `c` again to go back.

The table counts its columns from the header, which has a cell for every width. It never reads
the other rows to count them, so a borrowed table takes the same time to draw at 200 rows or at
200,000.

[`Table::from`]: https://docs.rs/ratatui-table/latest/ratatui_table/struct.Table.html#borrowing-rows
