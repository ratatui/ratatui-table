# Examples

Each example is its own crate under `apps/`, so it can pull in whatever dependencies it needs
without those reaching the library's own dependency tree.

```shell
cargo run -p selection
```

| example | what it shows |
| --- | --- |
| [selection](apps/selection) | row, column, and cell selection, and where the highlight symbol goes |

These are built against the working tree, not the released crate, so they may use APIs that are
not published yet.

## Recordings

The tapes in `vhs/` drive the examples for the GIFs in the README and in pull requests. Install
[VHS], then run one from the repository root:

```shell
just record selection
```

The recipe builds the example first and writes `target/<example>.gif`, which is ignored along with
every other recording. It is deliberately not part of `just check-all`, since VHS is an extra tool
to install and a recording takes about a minute.

[VHS]: https://github.com/charmbracelet/vhs
