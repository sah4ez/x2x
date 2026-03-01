# GitHub Actions Workflows

This directory contains CI/CD workflows for the x2x-rust project.

## Workflows

### 1. test.yml
Runs tests and generates coverage reports.

**Triggers:**
- Push to `rust_refactor`, `rust_refactor_cleanup`, `main`
- Pull requests to same branches

**Jobs:**
- **Tests**: Runs on Ubuntu with Rust stable, beta, and nightly
  - Check formatting with `rustfmt`
  - Run clippy
  - Build project
  - Run tests

- **Coverage**: Runs after tests
  - Generates coverage with `cargo-tarpaulin`
  - Uploads to Codecov
  - Uploads coverage artifact

### 2. lint.yml
Runs linting checks separately from tests.

**Jobs:**
- **rustfmt**: Check code formatting
- **clippy**: Run linter checks

## Dependencies

The workflows install X11 dependencies needed for x2x-rust:
- `libx11-dev`
- `libxi-dev`
- `libxtst-dev`

## Coverage

Coverage is generated using `cargo-tarpaulin` and uploaded to Codecov.
Results are also available as artifacts in the GitHub Actions UI.

## Status Badges

You can add these badges to your README.md:

```markdown
[![Test](https://github.com/sah4ez/x2x/actions/workflows/test.yml/badge.svg)]
[![Lint](https://github.com/sah4ez/x2x/actions/workflows/lint.yml/badge.svg)]
[![codecov](https://codecov.io/gh/sah4ez/x2x/branch/main/graph/badge.svg)]
```
