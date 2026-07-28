# Contributing to cargo-qc

First off, thank you for considering contributing to `cargo-qc`! It's people like you that make this tool great.

## Development Setup

1. Fork and clone the repository.
2. Ensure you have Rust installed (`rustup`). The project uses the 2024 edition, so you'll need Rust 1.85 or later.
3. Run `cargo build` to build the CLI.

## Testing Your Changes

Before submitting a Pull Request, please ensure all checks pass. Ironically (but fittingly), the best way to do this is to run `cargo-qc` on itself:

```bash
cargo run --bin cargo-qc
```

If it passes with 4 green checks, you are good to go!
You can also run the full test suite manually:

```bash
cargo test
```

## Pull Request Process

1. **Commit Messages**: We strictly follow [Conventional Commits](https://www.conventionalcommits.org/). Your commit messages should look like:
   - `feat(ux): add native spinners`
   - `fix(parser): handle missing config gracefully`
   - `docs: update readme`
   Our `git-cliff` configuration relies on these to automatically generate the changelog.

2. **Atomic Commits**: Ensure your commits are logical and focused. Do not mix formatting changes with logic changes.

3. **No MSRV Breakage**: Ensure your code compiles on the MSRV declared in `Cargo.toml`.

Thank you for your contribution!
