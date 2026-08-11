//! Mermaid source normaliser — narrow compatibility shim.
//!
//! Fixes one specific Mermaid.js 11 parse failure: unquoted flowchart node
//! labels of the form `[/text]` (leading `/`, no trailing `/`) that Mermaid.js
//! misreads as a malformed parallelogram shape.  The original source is never
//! mutated; this module only produces a locally-normalised copy for the JS call.

use std::borrow::Cow;

/// Return a Mermaid-safe version of `code` for passing to Mermaid.js.
///
/// **Only one transformation is applied.** In `flowchart` / `graph` diagrams,
/// an unquoted node label of the form `[/...]` whose content
///
/// * starts with `/` but does not end with `/` (not a valid parallelogram), and
/// * contains none of `"`, `\`, `[` (characters that would corrupt the quoted form
///   or make the `]` boundary unreliable),
///
/// is converted to `["/..."]`.  If any safety condition fails the label is left
/// unchanged and Mermaid.js reports its normal error.
///
/// Non-`flowchart` / non-`graph` diagrams are returned unchanged.
///
/// # Source immutability
///
/// The function signature is `fn(&str) -> String`.  Rust's type system
/// statically prevents the input from being mutated; the caller's `code`
/// is always intact after this call.
pub fn normalize_mermaid_source(code: &str) -> String {
    if !is_flowchart(code) {
        return code.to_owned();
    }
    // Fix 2: skip Mermaid comment lines (`%% …`) before normalising.
    // Comment lines must be returned byte-for-byte unchanged.
    let lines: Vec<Cow<'_, str>> = code
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("%%") {
                Cow::Borrowed(line)
            } else {
                normalize_line(line)
            }
        })
        .collect();
    let joined = lines.join("\n");
    if code.ends_with('\n') {
        joined + "\n"
    } else {
        joined
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Return `true` iff the first meaningful (non-blank, non-comment) line of
/// `code` declares a `flowchart` or `graph` diagram (case-insensitive).
///
/// Fix 3: token-aware matching.  The keyword must be followed by end-of-string,
/// ASCII whitespace, or nothing — not by another identifier character.  This
/// prevents `flowchartSomething` or `graphSomething` from being mistaken for a
/// flowchart declaration.
fn is_flowchart(code: &str) -> bool {
    for line in code.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("%%") {
            continue;
        }
        let lower = t.to_ascii_lowercase();
        return is_keyword_token(&lower, "flowchart") || is_keyword_token(&lower, "graph");
    }
    false
}

/// Return `true` if `s` starts with `keyword` and is either exactly that
/// keyword or has ASCII whitespace immediately after it.
///
/// This is the token-boundary check for `is_flowchart`.
fn is_keyword_token(s: &str, keyword: &str) -> bool {
    if !s.starts_with(keyword) {
        return false;
    }
    // Must be an exact match or followed by whitespace (e.g. `flowchart TD`).
    matches!(
        s.as_bytes().get(keyword.len()),
        None | Some(&b' ') | Some(&b'\t')
    )
}

/// Scan one flowchart line and quote any `[/...]` node label that is both
/// ambiguous and safe to quote.
///
/// The scanner triggers **only on the two-byte sequence `[/`**.  For each
/// match it reads forward to `]` in a single pass that simultaneously
/// validates the content.  If any of the bytes `"`, `\`, or `[` appear
/// before `]`, the group is left unchanged (quoting it would produce invalid
/// Mermaid or rely on an unreliable `]` boundary).
///
/// # Why `"`, `\`, `[` are disqualifying
///
/// | Char | Risk if quoted |
/// |------|----------------|
/// | `"`  | `["/foo"bar"]` -- unmatched quote, invalid Mermaid |
/// | `\`  | `["/foo\bar"]` -- undefined escape semantics |
/// | `[`  | `["/foo[bar"]` -- the `]` we found may not be the label's `]` |
///
/// # UTF-8 safety
///
/// `[`, `/`, `]`, `"`, `\` are single-byte ASCII codepoints (<= 0x7F), so
/// every byte index used for `&line[a..b]` slices falls on a valid UTF-8
/// character boundary.
fn normalize_line(line: &str) -> Cow<'_, str> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut flush = 0usize; // last byte position written into `out`
    let mut out = String::new();
    let mut i = 0usize;
    let mut in_quotes = false; // Fix 1: are we inside a `"…"` quoted span?
    let mut in_edge_label = false; // Fix 5: are we inside an `|…|` edge label?

    // We need at least two bytes for `[/`.
    while i + 1 < len {
        // Fix 4: Handle escape sequences ONLY in quoted state.
        // `\` outside quotes is treated as an ordinary character.
        if in_quotes && bytes[i] == b'\\' {
            i += 2; // skip `\` and the escaped character
            continue;
        }

        // Fix 1: track entry/exit of quoted spans.
        if bytes[i] == b'"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }

        // Fix 5: track entry/exit of edge labels (only when not in quotes).
        if !in_quotes && bytes[i] == b'|' {
            in_edge_label = !in_edge_label;
            i += 1;
            continue;
        }

        // While inside a quoted span or an edge label, advance without triggering normalization.
        if in_quotes || in_edge_label {
            i += 1;
            continue;
        }

        if bytes[i] != b'[' || bytes[i + 1] != b'/' {
            i += 1;
            continue;
        }

        // Found `[/`.  Scan forward for `]`, checking safety in the same pass.
        let content_start = i + 1; // byte index of the leading `/`
        let mut j = content_start;
        let mut safe = true;

        while j < len {
            match bytes[j] {
                b']' => break,
                // These make the resulting quoted form invalid or the boundary
                // unreliable -- conservatively abort and leave the source alone.
                b'"' | b'\\' | b'[' => {
                    safe = false;
                    break;
                }
                _ => j += 1,
            }
        }

        if safe && j < len && bytes[j] == b']' {
            let content = &line[content_start..j]; // the `/...` portion
            if needs_quoting(content) {
                out.push_str(&line[flush..i]); // flush unchanged prefix
                out.push('[');
                out.push('"');
                out.push_str(content);
                out.push('"');
                out.push(']');
                flush = j + 1;
            }
            i = j + 1; // advance past `]` regardless
        } else {
            // Unsafe content or no closing `]` -- advance past `[` and continue.
            i += 1;
        }
    }

    if flush == 0 {
        Cow::Borrowed(line) // nothing changed -- zero allocation
    } else {
        out.push_str(&line[flush..]); // flush remaining tail
        Cow::Owned(out)
    }
}

/// Return `true` if `content` (the bytes between `[` and `]`, which always
/// starts with `/` when called from `normalize_line`) is an ambiguous slash
/// label that should be quoted.
///
/// Returns `false` for:
/// * a lone `/` -- not meaningful; let Mermaid.js decide.
/// * a valid parallelogram: starts with `/` **and** ends with `/` (e.g. `/x/`).
fn needs_quoting(content: &str) -> bool {
    // A lone `/` is degenerate -- do not guess.
    if content.len() < 2 {
        return false;
    }
    // Valid Mermaid parallelogram shape: `/text/` ends with `/`.
    !content.ends_with('/')
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Runtime rendering note
    //
    // Verifying that the *normalized* source is actually accepted by Mermaid.js
    // 11 and produces valid SVG requires a browser / WASM runtime environment.
    // Native Rust unit tests cannot invoke Mermaid.js directly.
    //
    // These tests prove the *string transformation* is correct and conservative.
    // Runtime acceptance must be verified separately via `trunk serve` or an
    // equivalent browser test.  Do not infer runtime success from these alone.
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // needs_quoting
    // -----------------------------------------------------------------------

    #[test]
    fn needs_quoting_yes_for_slash_prefix() {
        assert!(needs_quoting("/harness-write-goal"));
        assert!(needs_quoting("/harness-goal slug"));
        assert!(needs_quoting("/foo"));
        // Internal `/` is fine as long as content does not END with `/`.
        assert!(needs_quoting("/a/b"));
    }

    #[test]
    fn needs_quoting_no_for_parallelogram() {
        assert!(!needs_quoting("/Deploy/"));
        assert!(!needs_quoting("/Input Data/"));
        assert!(!needs_quoting("/x/")); // minimal valid parallelogram
    }

    #[test]
    fn needs_quoting_no_for_lone_slash() {
        assert!(!needs_quoting("/"));
    }

    // -----------------------------------------------------------------------
    // is_flowchart
    // -----------------------------------------------------------------------

    #[test]
    fn detects_flowchart_and_graph_variants() {
        assert!(is_flowchart("flowchart TD\n  A-->B"));
        assert!(is_flowchart("flowchart LR\n  A-->B"));
        assert!(is_flowchart("flowchart RL\n  A-->B"));
        assert!(is_flowchart("flowchart TB\n  A-->B"));
        assert!(is_flowchart("flowchart\n  A-->B")); // keyword alone
        assert!(is_flowchart("FLOWCHART TD\n  A-->B")); // case-insensitive
        assert!(is_flowchart("graph TD\n  A-->B"));
        assert!(is_flowchart("graph LR\n  A-->B"));
        assert!(is_flowchart("graph\n  A-->B")); // keyword alone
        assert!(is_flowchart("GRAPH LR\n  A-->B"));
    }

    #[test]
    fn non_flowchart_diagrams_not_detected() {
        assert!(!is_flowchart("sequenceDiagram\n  A->>B: Hi"));
        assert!(!is_flowchart("classDiagram\n  A <|-- B"));
        assert!(!is_flowchart("stateDiagram-v2\n  [*] --> S"));
        assert!(!is_flowchart("gantt\n  title G"));
        assert!(!is_flowchart("erDiagram\n  A }|--|{ B : has"));
    }

    /// Fix 3: ambiguous identifier prefixes must NOT be detected as flowchart.
    #[test]
    fn flowchart_keyword_is_token_aware() {
        assert!(!is_flowchart("flowchartSomething\n  A-->B"));
        assert!(!is_flowchart("graphSomething\n  A-->B"));
        assert!(!is_flowchart("flowchartTD\n  A-->B")); // no space separator
        assert!(!is_flowchart("graphLR\n  A-->B"));
    }

    #[test]
    fn skips_blank_lines_and_comments_before_type() {
        assert!(is_flowchart("\n%% comment\nflowchart TD\n  A-->B"));
    }

    // -----------------------------------------------------------------------
    // normalize_line -- transformation correctness
    // -----------------------------------------------------------------------

    #[test]
    fn quotes_slash_prefix_label() {
        assert_eq!(
            normalize_line("  B[/harness-write-goal]"),
            "  B[\"/harness-write-goal\"]"
        );
    }

    #[test]
    fn quotes_slash_prefix_label_with_space() {
        assert_eq!(
            normalize_line("  F[/harness-goal slug]"),
            "  F[\"/harness-goal slug\"]"
        );
    }

    #[test]
    fn transforms_multiple_nodes_on_one_line() {
        assert_eq!(
            normalize_line("  B[/write-goal] --> C[/read-goal]"),
            "  B[\"/write-goal\"] --> C[\"/read-goal\"]"
        );
    }

    #[test]
    fn transforms_only_ambiguous_when_mixed_with_parallelogram() {
        // `[/foo]` is ambiguous -- must be quoted.
        // `[/bar/]` is a valid parallelogram -- must NOT be quoted.
        assert_eq!(
            normalize_line("  A[/foo] --> B[/bar/]"),
            "  A[\"/foo\"] --> B[/bar/]"
        );
    }

    // -----------------------------------------------------------------------
    // normalize_line -- non-regression (must NOT transform)
    // -----------------------------------------------------------------------

    #[test]
    fn leaves_parallelogram_unchanged() {
        let line = "  B[/Deploy/]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_already_quoted_label_unchanged() {
        // `["/Deploy"]` -- `bytes[i+1]` is `"`, not `/`.  The `[/` trigger never fires.
        let line = "  B[\"/Deploy\"]";
        assert_eq!(normalize_line(line), line);
    }

    /// Quoting a label containing `"` would produce `["/foo"bar"]` -- invalid Mermaid.
    #[test]
    fn leaves_label_with_embedded_quote_unchanged() {
        let line = "  B[/foo\"bar]";
        assert_eq!(normalize_line(line), line);
    }

    /// Quoting a label containing `\` would produce `["/foo\bar"]` -- escape semantics unknown.
    #[test]
    fn leaves_label_with_backslash_unchanged() {
        let line = "  B[/foo\\bar]";
        assert_eq!(normalize_line(line), line);
    }

    /// `[` inside content means the `]` we found may not be the label boundary.
    #[test]
    fn leaves_label_with_open_bracket_unchanged() {
        let line = "  B[/foo[bar]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_ordinary_label_unchanged() {
        let line = "  A[User intent]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_internal_slash_no_leading_slash_unchanged() {
        // `[Fix...` -- `bytes[i+1]` is `F`, not `/`.  Trigger never fires.
        let line = "  J[Fix / log iteration]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_decision_node_unchanged() {
        // `{...}` -- delimiter is `{`, not `[`.  Trigger never fires.
        let line = "  D{/harness-review-card?}";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_edge_label_with_slash_unchanged() {
        // Edge labels use `|...|`.  Only `[/` triggers the normaliser.
        let line = "  I -->|exit != 0| J[Fix / log iteration]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_url_in_edge_label_unchanged() {
        let line = "  A -->|https://example.com/path| B[Normal]";
        assert_eq!(normalize_line(line), line);
    }

    /// Fix 1: a `[/...]` sequence inside a quoted label must not be rewritten.
    #[test]
    fn leaves_quoted_label_containing_slash_bracket_unchanged() {
        let line = r#"  A["Example [/harness-goal] command"]"#;
        assert_eq!(normalize_line(line), line);
    }

    /// Fix 1: quoted label with the ambiguous pattern on the same line as a real node.
    #[test]
    fn leaves_quoted_with_slash_bracket_unchanged_alongside_real_node() {
        // `B[/real-node]` must be quoted; `A["desc [/harness] x"]` must not be touched.
        let line = r#"  A["desc [/harness] x"] --> B[/real-node]"#;
        assert_eq!(
            normalize_line(line),
            r#"  A["desc [/harness] x"] --> B["/real-node"]"#
        );
    }

    #[test]
    fn zero_copy_fast_path_when_nothing_changes() {
        // When no transformation occurs, normalize_line must return Cow::Borrowed
        // (zero allocation), not a cloned String.
        let line = "  A --> B";
        let result = normalize_line(line);
        assert!(
            matches!(result, Cow::Borrowed(_)),
            "unmodified line should be returned as Cow::Borrowed"
        );
        assert_eq!(result, line);
    }

    // -----------------------------------------------------------------------
    // normalize_mermaid_source -- end-to-end
    // -----------------------------------------------------------------------

    /// The exact failing flowchart from the bug report.
    #[test]
    fn fixes_exact_failing_flowchart() {
        let src = concat!(
            "flowchart TD\n",
            "  A[User intent] --> B[/harness-write-goal]\n",
            "  B --> C[goals/slug.yaml]\n",
            "  C --> D{/harness-review-card?}\n",
            "  D -->|NO-GO| E[Fix goal YAML]\n",
            "  E --> D\n",
            "  D -->|GO or skip if low| F[/harness-goal slug]\n",
            "  F --> G[Plan + stay on branch]\n",
            "  G --> H[Implement]\n",
            "  H --> I[verify.ps1 slug]\n",
            "  I -->|exit != 0| J[Fix / log iteration]\n",
            "  J --> H\n",
            "  I -->|exit 0| K{risk medium/high?}\n",
            "  K -->|yes| L[harness-review]\n",
            "  L -->|REQUEST_CHANGES| H\n",
            "  L -->|APPROVE| M[Summarize branch + diff]\n",
            "  K -->|low| M\n",
            "  M --> N[STOP]\n",
            "  I -->|blocked| O[blocker.md + STOP]\n"
        );
        let normalized = normalize_mermaid_source(src);

        // Ambiguous labels must be quoted.
        assert!(
            normalized.contains("B[\"/harness-write-goal\"]"),
            "B not quoted"
        );
        assert!(
            normalized.contains("F[\"/harness-goal slug\"]"),
            "F not quoted"
        );

        // The original still has the unquoted form (source never mutated).
        assert!(src.contains("B[/harness-write-goal]"));
        assert!(src.contains("F[/harness-goal slug]"));

        // Safe labels must be unchanged.
        assert!(normalized.contains("A[User intent]"));
        assert!(normalized.contains("E[Fix goal YAML]"));
        assert!(normalized.contains("J[Fix / log iteration]"));
        assert!(normalized.contains("D{/harness-review-card?}"));
        assert!(normalized.contains("|NO-GO|"));
        assert!(normalized.contains("|exit != 0|"));
    }

    #[test]
    fn fixes_harness_write_goal() {
        let src = "flowchart TD\n  B[/harness-write-goal]\n";
        assert!(normalize_mermaid_source(src).contains("B[\"/harness-write-goal\"]"));
    }

    #[test]
    fn fixes_harness_goal_slug() {
        let src = "flowchart LR\n  F[/harness-goal slug]\n";
        assert!(normalize_mermaid_source(src).contains("F[\"/harness-goal slug\"]"));
    }

    #[test]
    fn preserves_parallelogram() {
        let src = "flowchart TD\n  B[/Input Data/]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn preserves_already_quoted_label() {
        let src = "flowchart TD\n  B[\"/Deploy\"]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    /// A label with an embedded `"` must remain unchanged.
    /// If we were to quote it we would produce `["/foo"bar"]` -- invalid Mermaid.
    #[test]
    fn preserves_label_with_embedded_quote() {
        let src = "flowchart TD\n  B[/foo\"bar]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    /// A label with a `[` inside must remain unchanged -- the `]` boundary is unreliable.
    #[test]
    fn preserves_label_with_open_bracket() {
        let src = "flowchart TD\n  B[/foo[bar]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn non_flowchart_diagrams_unchanged() {
        let cases = [
            "sequenceDiagram\n  A->>B: /path\n  X[/fake]\n",
            "classDiagram\n  A <|-- B\n",
            "stateDiagram-v2\n  [*] --> S\n",
            "gantt\n  title G\n",
        ];
        for src in &cases {
            assert_eq!(
                normalize_mermaid_source(src),
                *src,
                "non-flowchart diagram was modified: {src}"
            );
        }
    }

    #[test]
    fn original_source_is_not_mutated() {
        let original = "flowchart TD\n  B[/harness-write-goal]\n";
        let normalized = normalize_mermaid_source(original);
        // Rust's type system guarantees immutability; also verify values.
        assert!(
            original.contains("B[/harness-write-goal]"),
            "original changed"
        );
        assert!(
            normalized.contains("B[\"/harness-write-goal\"]"),
            "normalized wrong"
        );
        assert_ne!(original, normalized.as_str());
    }

    #[test]
    fn trailing_newline_preserved() {
        let src = "flowchart TD\n  A[/foo]\n";
        assert!(normalize_mermaid_source(src).ends_with('\n'));
    }

    #[test]
    fn no_trailing_newline_not_added() {
        let src = "flowchart TD\n  A[/foo]";
        assert!(!normalize_mermaid_source(src).ends_with('\n'));
    }

    #[test]
    fn graph_keyword_detected() {
        let src = "graph TD\n  B[/harness-write-goal]\n";
        assert!(normalize_mermaid_source(src).contains("B[\"/harness-write-goal\"]"));
    }

    #[test]
    fn url_in_edge_label_unchanged() {
        let src = "flowchart TD\n  A -->|https://example.com/path| B[Normal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    /// Fix 2: Mermaid comment lines (`%% ...`) must be returned byte-for-byte unchanged.
    #[test]
    fn leaves_comment_line_with_slash_bracket_unchanged() {
        let src = concat!(
            "flowchart TD\n",
            "  %% Example: [/harness-goal]\n",
            "  A --> B\n",
        );
        assert_eq!(normalize_mermaid_source(src), src);
    }

    /// Fix 2: comment at the start of a line with leading whitespace.
    #[test]
    fn leaves_indented_comment_unchanged() {
        let src = "flowchart TD\n    %% [/some-command] -- skip this\n  A --> B\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    /// Fix 3: ambiguous identifier prefixes must not be treated as flowchart diagrams.
    #[test]
    fn flowchart_something_not_normalized() {
        let src = "flowchartSomething\n  B[/harness-write-goal]\n";
        // Not a flowchart -- returned unchanged.
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn graph_something_not_normalized() {
        let src = "graphSomething\n  B[/harness-write-goal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    // -----------------------------------------------------------------------
    // Escape-sequence handling (Fix 4)
    // -----------------------------------------------------------------------

    /// `\"` inside a quoted label must NOT toggle quote state.
    /// The `[/...]` that follows must be treated as still inside the quote and left untouched.
    #[test]
    fn leaves_quoted_label_with_escaped_quote_unchanged() {
        let line = r#"  A["Example \"quoted\" [/harness-goal]"]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn does_not_normalize_after_escaped_quote_inside_label() {
        let line = r#"  A["Command \"test\" [/harness-goal]"]"#;
        assert_eq!(normalize_line(line), line);
    }

    // -----------------------------------------------------------------------
    // Edge-label tracking (Fix 5)
    // -----------------------------------------------------------------------

    /// `|...|` edge labels must not trigger normalization.
    #[test]
    fn leaves_edge_label_with_slash_bracket_unchanged() {
        let line = r#"  A -->|[/harness-goal]| B[Normal]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn preserves_slash_bracket_inside_edge_label() {
        let src = "flowchart TD\n  A -->|[/harness-goal]| B[Normal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }
}
