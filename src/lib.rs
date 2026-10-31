// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Markdown normalization tasks for Bake.
//!
//! [`normalize_document`] parses CommonMark, GFM, front matter, and math, then
//! serializes the result with soft source line breaks unwrapped. CommonMark
//! includes inline code. The `markdown:normalize` Bake task applies this to
//! one or more files. Normalization is implemented by
//! [`base_markdown::normalize_markdown`], so other tools can produce the same
//! output without depending on these tasks.
use base_markdown::{
    ParseOptions, message::Message, normalize_markdown, normalize_markdown_with_options,
};

/// Normalize a Markdown document by joining soft source line breaks.
///
/// CommonMark, GFM, front matter, and math syntax are parsed so these features
/// survive the round trip. This includes inline code, fenced code, tables,
/// task lists, footnotes, and strikethrough. Explicit hard breaks, code
/// content, and block boundaries are preserved. The result ends with a newline
/// when it is non-empty.
///
/// # Errors
///
/// Returns an error if the document cannot be serialized.
pub fn normalize_document(source: &str) -> Result<String, Message> {
    normalize_markdown(source)
}

/// Normalize a Markdown document using explicit parser options.
///
/// Use this for syntax beyond the default GitHub Flavored Markdown, front
/// matter, and math support, or to disable individual constructs.
///
/// # Errors
///
/// Returns an error if the document cannot be serialized.
pub fn normalize_document_with_options(
    source: &str,
    parse_options: &ParseOptions,
) -> Result<String, Message> {
    normalize_markdown_with_options(source, parse_options)
}

mod file_system;

use std::path::PathBuf;

/// Normalize Markdown files beneath the Bake project root.
#[bake::task]
pub fn normalize(
    context: &mut bake::Context,
    #[bake(positional, help = "Markdown files, relative to the project root.")] paths: Vec<PathBuf>,
) -> bake::Result<String> {
    let changed = file_system::normalize_files(context.root(), &paths)?;
    Ok(format!(
        "Normalized {changed} of {} Markdown files",
        paths.len()
    ))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
