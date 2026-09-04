# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - 2026-09-04

### 🐛 Bug Fixes

- Render changelog whitespace correctly
  > Use git-cliff whitespace controls instead of emitting literal escape
  > characters, and clean the initial checked changelog.

### ⚙️ Miscellaneous Tasks

- Update justfile to include default list (#16)
  > With recent versions of just, you can remove the default recipe when you
  > are using it for a list with this syntax. The advantage is it doesn't
  > show the default recipe as an option. Source:
  > https://just.systems/man/en/the-default-recipe.html
  >
  > Alternatively, you can add the `[private]` attribute:
  >
  > ```
  > [private]
  > default:
  >     @just --list
  > ```
  >
  > But the syntax in the PR is nicer imo.


## [0.1.0] - 2026-07-31

### 🚀 Features

- Seed the Ratatui table experiment
  > Extract the current built-in Table into an independently governed crate
  > while preserving its initial API and rendering behavior.

### 🐛 Bug Fixes

- Prepare minor releases before 1.0
