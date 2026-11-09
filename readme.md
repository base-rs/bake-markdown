# `bake-markdown`

Markdown normalization for Bake projects. It joins soft source line breaks in Markdown files while preserving explicit hard breaks, code, and block boundaries.

## Motivation

Hard-wrapped paragraphs are difficult to edit consistently. This crate uses the `base-markdown` syntax tree and serializer so normalization follows Markdown structure instead of changing lines with regular expressions.

## Usage

Add the crate to the project's private `bake/` package and regenerate its task links:

```sh
cargo bake --regenerate
cargo add --manifest-path bake/Cargo.toml bake-markdown
cargo bake --regenerate
```

Normalize one or more files relative to the project root:

```sh
cargo bake markdown:normalize readme.md context/guide.md
```

Rust callers can use `bake_markdown::normalize_document` directly.

## Releasing

Prepare a release with `cargo bake cargo:release:patch` (or `minor`, `major`, or `bump --version X.Y.Z`) from a clean checkout of `main`; it bumps the version, commits with the `Release: vVERSION` trailer, validates the candidate, and opens the pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it. Follow the shared [Releasing skill](https://github.com/base-rs/base-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.6.1

- Normalize through `base_markdown::normalize_markdown`, which now owns the normalization rules, so `markdown:normalize` and other tools built on `base-markdown`, such as `bake-readme`, write identical Markdown. Requires `base-markdown` 0.1.3.
- Raise the minimum supported Rust version in `rust-version` from 1.85 to 1.88, as the let-chains in `bake-macros` 0.20 already require.

### v0.6.0

- Depend on `base-markdown` under the `base-rs` organization, replacing `core-markdown`. Callers passing explicit `ParseOptions` or naming parser errors must use the corresponding types from `base-markdown`.

### v0.5.0

- Migrate to the source-backed `core-markdown` document AST, preserving supported Markdown behavior and source bytes. This changes the public parser option and diagnostic types to `core-markdown` 0.2; callers using explicit options must update their dependency too.

<!-- bake-readme:releases:end -->

## See Also

[`base-markdown`](https://github.com/base-rs/base-markdown) provides the parser and serializer used by this crate.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/bake-markdown).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
