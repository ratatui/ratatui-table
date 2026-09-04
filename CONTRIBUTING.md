# Contributing to ratatui-table

Thank you for helping evolve Ratatui's Table widget.

## Design and review

For substantial or breaking changes, open or join an issue before implementation. Explain:

- The use case and constraints.
- Alternatives considered.
- The expected migration story.

Keep each pull request focused on one reviewable behavior and include:

- The resulting behavior.
- Validation performed.
- Links to related issues.

Consult the [Table evolution roadmap](https://github.com/ratatui/ratatui-table/issues/7) for current
proposals and coordination with Ratatui. Keep proposals independently reviewable and document
migrations for breaking changes.

Contributors with Write access are encouraged to review each other's ordinary source changes. The
Ratatui maintainers own repository security settings and release approval; changes to protected
release, workflow, manifest, and lockfile paths require maintainer review.

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) so release-plz can create
versions and changelog entries. Pull request titles should use the same format.

## Bringing work from Ratatui

When porting an upstream proposal:

- Link its successor issue.
- Preserve the original author's attribution.
- Use existing discussions, reviews, tests, and history to inform the proposal; you may revisit
  the implementation rather than port it unchanged.
- Keep upstream pull requests open until a successor exists or their authors agree to closure.

Coordinate changes to shared widget behavior and core primitives in the main Ratatui repository.
Keep work here scoped to Table; consult the roadmap for proposal-specific ownership.

## Development

Install these development prerequisites:

- Rust meeting the version and edition requirements in [Cargo.toml](Cargo.toml).
- `just` for development recipes.
- Nightly for formatting and documentation.
- Stable and beta with Clippy for `just clippy-all`.

Run commands from the crate root. `cargo build` builds the library; the main local checks are:

```shell
just fmt-check   # Check nightly rustfmt formatting
just clippy-all  # Lint all targets/features on stable and beta
just test        # Test all features, no defaults, and serde without defaults
just docs        # Build Rustdoc with warnings denied
just check-all   # Run all local checks, including dependency and package checks
```

Run `just --list` for the complete command list. CI also verifies the MSRV, minimal dependency
floors, dependency policy, Markdown, spelling, package contents, and workflow security.

### Code and tests

The source layout is:

- `src/lib.rs`: crate documentation and public exports.
- `src/table.rs`: rendering implementation.
- `src/table/`: rows, cells, selection state, and highlight spacing.

Keep default builds usable with `no_std` plus `alloc`; gate standard-library behavior behind `std`.
Unsafe code is forbidden.

Follow `rustfmt.toml` and run `just fmt` to format changes. Use four-space indentation,
`snake_case` functions/modules, and `PascalCase` types. Keep behavior understandable locally.

For tests:

- Add unit tests beside the implementation and use behavior-based names.
- Use `rstest` for parameterized cases and `pretty_assertions` for readable diffs.
- Cover rendering changes with deterministic buffer assertions and boundary cases.
- Run `cargo test --test parity` to compare with the built-in widget, and document intentional
  divergences from the baseline.

### Documentation

- Document public APIs with runnable Rustdoc examples.
- After changing crate-level documentation, run `just rdme` to regenerate README content and
  `just rdme-check` to verify it.
- Wrap Markdown prose at 100 characters.
- Separate headings, lists, and code blocks with blank lines.
- Prefer lists for parallel items; use prose for connected explanations.
- Keep guidance durable; link to roadmap issues for proposal details and current status.
- Run `markdownlint-cli2 "**/*.md"`.

Do not edit `CHANGELOG.md` for release entries. release-plz and git-cliff own generated release
notes.
