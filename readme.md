# `bake-markdown`

Markdown normalization for Bake projects. It joins soft source line breaks in Markdown files while preserving explicit hard breaks, code, and block boundaries.

## Motivation

Hard-wrapped paragraphs are difficult to edit consistently. This crate uses the `core-markdown` syntax tree and serializer so normalization follows Markdown structure instead of changing lines with regular expressions.

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

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), commit with a `Release: vVERSION` trailer, then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it. Follow the shared [Releasing skill](https://github.com/core-rs/core-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.5.0

- Migrate to the source-backed `core-markdown` document AST, preserving supported Markdown behavior and source bytes. This changes the public parser option and diagnostic types to `core-markdown` 0.2; callers using explicit options must update their dependency too.

### v0.4.0

- Use `core-markdown` in place of `socketry-markdown`. Parser options and errors in the public API now use `core_markdown` types.

### v0.3.2

- Document the shared release process.

<!-- bake-readme:releases:end -->

## See Also

[`core-markdown`](https://github.com/core-rs/core-markdown) provides the parser and serializer used by this crate.

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/core-rs/bake-markdown).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
