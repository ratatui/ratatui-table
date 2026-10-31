# Lazy table demo

A journal viewer over ten million entries, none of them stored. The row factory _is_ the log, and
the counter in the status line reports how many entries the last frame actually had to build.

```shell
cargo run -p lazy_table
```

| key                    | effect                             |
| ---------------------- | ---------------------------------- |
| `↑` `↓` / `k` `j`      | move the selection one entry       |
| `PgUp` `PgDn`          | move it twenty entries             |
| `g` `G` / `Home` `End` | jump to the oldest or newest entry |
| `q`                    | quit                               |

Ten million lines is about a month of a chatty service. Collecting them into a `Vec` of rows would
cost the month; [`Table::lazy_rows`] costs the screen. Hold `↓` for as long as you like, or press
`G` to land on entry 9,999,999 — the build counter stays at a screenful either way.

`ERROR` entries are three lines tall, carrying an error code and a source location beneath the
message. That is the job of [`Table::lazy_row_height_with`]: scrolling needs the height of entries
it will never draw, and a lazy row cannot report its height without being built — which is the
cost the feature exists to avoid. Severity here is derived from the entry index, so the height is
known without touching the row.

[`Table::lazy_rows`]: https://docs.rs/ratatui-table/latest/ratatui_table/struct.Table.html#method.lazy_rows
[`Table::lazy_row_height_with`]: https://docs.rs/ratatui-table/latest/ratatui_table/struct.Table.html#method.lazy_row_height_with
