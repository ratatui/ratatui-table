# Contributing to ratatui-table

Thank you for helping evolve Ratatui's Table widget.

## Design and review

For substantial or breaking changes, open or join an issue before implementation. Explain the use
case, constraints, alternatives, and expected migration story. Keep each pull request focused on one
reviewable behavior.

Contributors with Write access are encouraged to review each other's ordinary source changes. The
Ratatui maintainers own repository security settings and release approval; changes to protected
release, workflow, manifest, and lockfile paths require maintainer review.

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) so release-plz can create
versions and changelog entries. Pull request titles should use the same format.

## Development

The main local checks are:

```shell
just fmt-check
just clippy-all
just test
just docs
just check-all
```

Run `just --list` for the complete command list. CI also verifies the MSRV, minimal dependency
floors, dependency policy, Markdown, spelling, package contents, and workflow security.

Do not edit `CHANGELOG.md` for release entries. release-plz and git-cliff own generated release
notes.
