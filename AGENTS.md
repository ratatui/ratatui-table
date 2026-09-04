# Repository Guidelines

## Project Structure & Module Organization

- `src/lib.rs`: crate documentation and public exports.
- `src/table.rs`: rendering implementation.
- `src/table/`: rows, cells, selection state, and highlight spacing.
- Unit tests: beside implementation.
- `tests/parity.rs`: rendering comparisons with the built-in widget.

Keep default builds usable with `no_std` plus `alloc`; gate standard-library behavior behind `std`.

## Design Scope & Upstream Proposals

Consult the [Table evolution roadmap](https://github.com/ratatui/ratatui-table/issues/7) for current
proposals and coordination with Ratatui.

- Keep proposals independently reviewable and document migrations for breaking changes.
- When porting upstream work, link its successor issue and preserve original authorship.
- Consult prior reviews, tests, and design discussion; implementations may be redesigned.
- Coordinate shared widget and core changes upstream; keep work here scoped to Table.

## Build, Test, and Development Commands

Use the Rust version and edition declared in [Cargo.toml](Cargo.toml). Formatting and docs require
nightly:

- `just fmt` / `just fmt-check`: apply or verify nightly rustfmt formatting.
- `just clippy-all`: lint all targets and features on stable and beta, denying warnings.
- `just test`: test all features, no default features, and serde without default features.
- `just docs`: build Rustdoc with warnings denied.
- `just rdme` / `just rdme-check`: regenerate or verify README content from crate documentation.
- `just check-all`: run all local checks, including dependencies, semver, and packaging.
- `markdownlint-cli2 "**/*.md"`: lint Markdown using repository configuration.

## Coding Style & Naming Conventions

- Follow `rustfmt.toml`: four-space indentation, `snake_case` functions/modules, `PascalCase` types.
- Keep behavior understandable locally. Unsafe code is forbidden.
- Document public APIs with runnable Rustdoc examples.
- Wrap Markdown prose at 100 characters; separate headings, lists, and code blocks with blank lines.
- Prefer lists for parallel items; use prose for connected explanations.
- Keep guidance durable; link to roadmap issues for proposal details and current status.

## Testing Guidelines

- Use Rust tests, `rstest` for parameterized cases, and `pretty_assertions` for diffs.
- Use behavior-based test names and deterministic buffer assertions for rendering and boundary cases.
- Run `cargo test --test parity` for baseline comparisons; document intentional divergences.

## Commit & Pull Request Guidelines

Use Conventional Commits for commits and PR titles, e.g. `fix: correct selection rendering`.
Keep PRs focused; describe behavior, validation, and linked issues. Discuss substantial changes in an
issue first. Contributors with Write access may review and merge ordinary source work; see
`CONTRIBUTING.md`.

Maintainers review `.github/`, release configuration, `Cargo.toml`, and `Cargo.lock` changes.
Release-plz owns versions and generated release notes; do not hand-edit changelog release entries.
