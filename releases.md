# Releases

## Unreleased

- Normalize through `base_markdown::normalize_markdown`, which now owns the normalization rules, so `markdown:normalize` and other tools built on `base-markdown`, such as `bake-readme`, write identical Markdown. Requires `base-markdown` 0.1.3.

## v0.6.0

- Depend on `base-markdown` under the `base-rs` organization, replacing `core-markdown`. Callers passing explicit `ParseOptions` or naming parser errors must use the corresponding types from `base-markdown`.

## v0.5.0

- Migrate to the source-backed `core-markdown` document AST, preserving supported Markdown behavior and source bytes. This changes the public parser option and diagnostic types to `core-markdown` 0.2; callers using explicit options must update their dependency too.

## v0.4.0

- Use `core-markdown` in place of `socketry-markdown`. Parser options and errors in the public API now use `core_markdown` types.

## v0.3.2

- Document the shared release process.

## v0.3.1

- Resolve development task dependencies to the current checkout.

- Adopt `socketry-project` 0.3.7 for shared project tasks and Markdown normalization.

- Require the aggregate test and coverage result for pull request merges.

- Avoid duplicate ordinary test runs in pull request publishing checks.

## v0.3.0

- Normalize unordered Markdown lists with hyphen markers.

## v0.2.0

- Accept Markdown paths as positional arguments to `markdown:normalize`.

## v0.1.0

- Add the `markdown:normalize` task to join soft source line breaks in Markdown files while preserving explicit hard breaks, code, and block boundaries.
